use aumm_core::{ Recognizer, Thresholds, InputEvent, KeyEvent, KeyState, Gesture, TapKind, KeyId };

// Helper to build events quickly.
fn ke(key: &str, state: KeyState, ts: u64) -> InputEvent {
    InputEvent::Key(KeyEvent { key: KeyId(key.into()), state, ts_ms: ts })
}

#[test]
fn single_tap_emits_exactly_at_first_down_plus_td() {
    let th = Thresholds { t_vs: 80, t_s: 200, t_n: 400, t_h: 500, t_d: 300 };
    let mut r = Recognizer::new(th);

    // down@100, up@180 (dur=80 -> Short because VS is <80)
    let _ = r.feed(ke("A", KeyState::Down, 100));
    let _ = r.feed(ke("A", KeyState::Up, 180));

    // window closes at 100 + 300 = 400
    assert!(r.feed(InputEvent::Tick(399)).is_empty());
    let out = r.feed(InputEvent::Tick(400));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].key, KeyId("A".into()));
    assert_eq!(out[0].gesture, Gesture::Tap(TapKind::Short));
    assert_eq!(out[0].decided_at_ms, 400);
}

#[test]
fn boundary_79ms_is_very_short_80ms_is_short_200ms_is_normal() {
    let th = Thresholds { t_vs: 80, t_s: 200, t_n: 400, t_h: 500, t_d: 250 };
    let mut r = Recognizer::new(th);

    // 79 -> VS
    let _ = r.feed(ke("B", KeyState::Down, 0));
    let _ = r.feed(ke("B", KeyState::Up, 79));
    let out = r.feed(InputEvent::Tick(0 + th.t_d));
    assert_eq!(out[0].gesture, Gesture::Tap(TapKind::VeryShort));

    // 80 -> Short
    let mut r = Recognizer::new(th);
    let _ = r.feed(ke("C", KeyState::Down, 0));
    let _ = r.feed(ke("C", KeyState::Up, 80));
    let out = r.feed(InputEvent::Tick(0 + th.t_d));
    assert_eq!(out[0].gesture, Gesture::Tap(TapKind::Short));

    // 200 -> Normal
    let mut r = Recognizer::new(th);
    let _ = r.feed(ke("D", KeyState::Down, 0));
    let _ = r.feed(ke("D", KeyState::Up, 200));
    let out = r.feed(InputEvent::Tick(0 + th.t_d));
    assert_eq!(out[0].gesture, Gesture::Tap(TapKind::Normal));
}

#[test]
fn hold_is_emitted_immediately_on_release_when_duration_exceeds_th() {
    let th = Thresholds { t_vs: 80, t_s: 200, t_n: 400, t_h: 500, t_d: 300 };
    let mut r = Recognizer::new(th);

    let mut out = Vec::new();
    out.extend(r.feed(ke("H", KeyState::Down, 0)));
    out.extend(r.feed(ke("H", KeyState::Up, 700)));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].gesture, Gesture::Hold);
}

#[test]
fn double_tap_fires_immediately_when_triple_disabled() {
    let th = Thresholds::default();
    let mut r = Recognizer::new(th);
    r.triple_enabled = false;

    // tap1: (0..50), tap2: (100..150)
    let mut out = Vec::new();
    out.extend(r.feed(ke("X", KeyState::Down, 0)));
    out.extend(r.feed(ke("X", KeyState::Up, 50)));
    out.extend(r.feed(ke("X", KeyState::Down, 100)));
    out.extend(r.feed(ke("X", KeyState::Up, 150)));

    assert_eq!(out.len(), 1);
    assert_eq!(out[0].gesture, Gesture::DoubleTap);
}

#[test]
fn double_tap_requires_tick_when_triple_enabled() {
    let th = Thresholds { t_vs: 80, t_s: 200, t_n: 400, t_h: 500, t_d: 250 };
    let mut r = Recognizer::new(th);
    r.triple_enabled = true;

    // tap1: (0..60 VS), tap2: (200..320 Short)
    let mut out = Vec::new();
    out.extend(r.feed(ke("K", KeyState::Down, 0)));
    out.extend(r.feed(ke("K", KeyState::Up, 60)));
    out.extend(r.feed(ke("K", KeyState::Down, 200)));
    out.extend(r.feed(ke("K", KeyState::Up, 320)));

    // double should finalize at 320 + t_d
    assert!(r.feed(InputEvent::Tick(569)).is_empty());
    let out2 = r.feed(InputEvent::Tick(570));
    assert_eq!(out2.len(), 1);
    assert_eq!(out2[0].gesture, Gesture::DoubleTap);
    assert_eq!(out2[0].decided_at_ms, 570);
}

#[test]
fn triple_tap_is_emitted_on_third_release() {
    let th = Thresholds { t_vs: 80, t_s: 200, t_n: 400, t_h: 500, t_d: 300 };
    let mut r = Recognizer::new(th);

    let mut out = Vec::new();
    // 1st
    out.extend(r.feed(ke("T", KeyState::Down, 0)));
    out.extend(r.feed(ke("T", KeyState::Up, 50)));
    // 2nd
    out.extend(r.feed(ke("T", KeyState::Down, 200)));
    out.extend(r.feed(ke("T", KeyState::Up, 260)));
    // 3rd
    out.extend(r.feed(ke("T", KeyState::Down, 400)));
    out.extend(r.feed(ke("T", KeyState::Up, 460)));

    assert_eq!(out.len(), 1);
    assert_eq!(out[0].gesture, Gesture::TripleTap);
}

#[test]
fn spurious_up_in_idle_is_ignored() {
    let th = Thresholds::default();
    let mut r = Recognizer::new(th);
    // Up without a prior Down
    let out = r.feed(ke("S", KeyState::Up, 0));
    assert!(out.is_empty());
}
