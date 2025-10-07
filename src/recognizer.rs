use hashbrown::HashMap;
use crate::{
    config::Thresholds,
    event::{ InputEvent, KeyEvent, KeyState },
    gesture::{ Gesture, TapKind, GestureEvent },
    key::KeyId,
};

#[derive(Debug, Clone)]
struct TapInfo {
    t_down: u64,
    t_up: u64,
    kind: TapKind,
}

#[derive(Debug, Clone)]
enum KeyFSM {
    Idle,
    Pressed {
        t_down: u64,
    },
    /// First tap decided, waiting for a second press until `wait_until`.
    WaitingSecond {
        first: TapInfo,
        wait_until: u64,
    },
    /// First+Second decided, waiting for a third press until `wait_until`.
    WaitingThird {
        first: TapInfo,
        second: TapInfo,
        wait_until: u64,
    },
}

impl KeyFSM {
    fn is_idle(&self) -> bool {
        matches!(self, KeyFSM::Idle)
    }
}

/// Deterministic recognizer (no internal timers). Drive time with `Tick(now_ms)`.
pub struct Recognizer {
    th: Thresholds,
    states: HashMap<KeyId, KeyFSM>,
    // Stashes keep context while FSM is temporarily in `Pressed`.
    second_stash: HashMap<KeyId, (TapInfo, u64)>,
    third_stash: HashMap<KeyId, (TapInfo, TapInfo, u64)>,
    /// If false, finalize DoubleTap immediately on 2nd release (no triple aggregation window).
    pub triple_enabled: bool,
}

impl Recognizer {
    pub fn new(thresholds: Thresholds) -> Self {
        assert!(thresholds.sanity_check(), "invalid thresholds");
        Self {
            th: thresholds,
            states: HashMap::new(),
            second_stash: HashMap::new(),
            third_stash: HashMap::new(),
            triple_enabled: true,
        }
    }

    pub fn thresholds(&self) -> Thresholds {
        self.th
    }

    /// Feed an input event and get zero or more decided gestures.
    pub fn feed(&mut self, ev: InputEvent) -> Vec<GestureEvent> {
        match ev {
            InputEvent::Key(k) => self.on_key(k),
            InputEvent::Tick(now) => self.on_tick(now),
        }
    }

    fn on_key(&mut self, kev: KeyEvent) -> Vec<GestureEvent> {
        let now = kev.ts_ms;
        let key = kev.key.clone();

        // Clone current state to work with, then write back once at end (avoid borrow issues).
        let current = self.states.entry(key.clone()).or_insert(KeyFSM::Idle).clone();

        let mut out = Vec::new();
        let mut next_state = current.clone();

        match (current, kev.state) {
            // ---------- First press ----------
            (KeyFSM::Idle, KeyState::Down) => {
                next_state = KeyFSM::Pressed { t_down: now };
            }

            // ---------- Possibly first/second/third release (unified) ----------
            (KeyFSM::Pressed { t_down }, KeyState::Up) => {
                let dur = now.saturating_sub(t_down);

                // Hold supersedes.
                if dur >= self.th.t_h {
                    // Cancel any waiting multi-tap context.
                    self.second_stash.remove(&key);
                    self.third_stash.remove(&key);
                    out.push(GestureEvent {
                        key: key.clone(),
                        gesture: Gesture::Hold,
                        decided_at_ms: now,
                    });
                    next_state = KeyFSM::Idle;
                } else if let Some((first, _wait_until)) = self.second_stash.remove(&key) {
                    // This was 2nd release (we had a first tap stashed).
                    let second_kind = if dur < self.th.t_vs {
                        TapKind::VeryShort
                    } else if dur < self.th.t_s {
                        TapKind::Short
                    } else {
                        TapKind::Normal
                    };
                    let second = TapInfo { t_down, t_up: now, kind: second_kind };

                    if self.triple_enabled {
                        // Triple window is based on SECOND tap's t_up.
                        let wait = now + self.th.t_d;
                        self.third_stash.insert(key.clone(), (first.clone(), second.clone(), wait));
                        next_state = KeyFSM::WaitingThird {
                            first: first.clone(),
                            second: second.clone(),
                            wait_until: wait,
                        };
                    } else {
                        // Finalize double immediately.
                        out.push(GestureEvent {
                            key: key.clone(),
                            gesture: Gesture::DoubleTap,
                            decided_at_ms: now,
                        });
                        next_state = KeyFSM::Idle;
                    }
                } else if let Some((_f, _s, _until)) = self.third_stash.remove(&key) {
                    // This was 3rd release (we had first+second stashed).
                    out.push(GestureEvent {
                        key: key.clone(),
                        gesture: Gesture::TripleTap,
                        decided_at_ms: now,
                    });
                    next_state = KeyFSM::Idle;
                } else {
                    // This is the 1st release (no stash yet) => open window for a second tap.
                    // SINGLE-TAP decision window is based on FIRST tap's t_down.
                    let kind = if dur < self.th.t_vs {
                        TapKind::VeryShort
                    } else if dur < self.th.t_s {
                        TapKind::Short
                    } else {
                        TapKind::Normal
                    };
                    let tap = TapInfo { t_down, t_up: now, kind };
                    let wait = t_down + self.th.t_d; // <-- 핵심 변경: now+t_d 가 아니라 t_down+t_d
                    next_state = KeyFSM::WaitingSecond { first: tap, wait_until: wait };
                }
            }

            // ---------- Second tap path: 2nd Down while waiting ----------
            (KeyFSM::WaitingSecond { first, wait_until }, KeyState::Down) => {
                // Remember first-tap context while we record 2nd press duration.
                self.second_stash.insert(key.clone(), (first, wait_until));
                next_state = KeyFSM::Pressed { t_down: now };
            }

            // ---------- Triple tap path: 3rd Down while waiting ----------
            (KeyFSM::WaitingThird { first, second, wait_until }, KeyState::Down) => {
                // Keep context for 3rd release duration measurement.
                self.third_stash.insert(key.clone(), (first, second, wait_until));
                next_state = KeyFSM::Pressed { t_down: now };
            }

            // Spurious Up events during waiting states: keep waiting.
            (KeyFSM::WaitingSecond { first, wait_until }, KeyState::Up) => {
                next_state = KeyFSM::WaitingSecond { first, wait_until };
            }
            (KeyFSM::WaitingThird { first, second, wait_until }, KeyState::Up) => {
                next_state = KeyFSM::WaitingThird { first, second, wait_until };
            }

            // Other combos: ignore to remain robust.
            (s, _) => {
                next_state = s;
            }
        }

        self.states.insert(key, next_state);
        out
    }

    fn on_tick(&mut self, now: u64) -> Vec<GestureEvent> {
        let mut out = Vec::new();
        let keys: Vec<KeyId> = self.states.keys().cloned().collect();

        for key in keys {
            let mut emit: Option<GestureEvent> = None;
            if let Some(state) = self.states.get(&key).cloned() {
                match state {
                    KeyFSM::WaitingSecond { first, wait_until } if now >= wait_until => {
                        emit = Some(GestureEvent {
                            key: key.clone(),
                            gesture: Gesture::Tap(first.kind),
                            decided_at_ms: wait_until,
                        });
                    }
                    KeyFSM::WaitingThird { .. } if
                        now >=
                        (match state {
                            KeyFSM::WaitingThird { wait_until, .. } => wait_until,
                            _ => 0,
                        })
                    => {
                        emit = Some(GestureEvent {
                            key: key.clone(),
                            gesture: Gesture::DoubleTap,
                            decided_at_ms: match state {
                                KeyFSM::WaitingThird { wait_until, .. } => wait_until,
                                _ => now,
                            },
                        });
                    }
                    _ => {}
                }
            }
            if let Some(ev) = emit {
                self.states.insert(key.clone(), KeyFSM::Idle);
                out.push(ev);
            }
        }

        out
    }
}

impl Default for Recognizer {
    fn default() -> Self {
        Self::new(Thresholds::default())
    }
}
