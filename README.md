# aumm_core

> **Abstract Universal Macro Model – Core Implementation (Rust)**  
> A deterministic, testable, and formally aligned implementation of the *Abstract Universal Macro Model* described in  
> _"Abstract Universal Macro Model – Theoretical Foundations"_ (2025).

---

## 1. Overview

`aumm_core` is the Rust implementation of the **Abstract Universal Macro Model (AUMM)** —  
a theoretical framework for interpreting key-based gestures (taps, holds, multi-taps)  
and mapping them to macro plans in a **deterministic**, **conflict-free**, and **extensible** way.

The package realizes the exact concepts presented in the paper:

| Theoretical Term | Implementation Component | Description |
|------------------|--------------------------|-------------|
| **Input Event Listener** | [`event.rs`](src/event.rs) | Models raw key press/release events with timestamps (`KeyEvent`, `KeyState`, `InputEvent`). |
| **Gesture Recognizer / State Machine** | [`recognizer.rs`](src/recognizer.rs) | A per-key finite state machine that converts input events into gesture events (`GestureEvent`). |
| **Gesture Definition** | [`gesture.rs`](src/gesture.rs) | Enumerates disjoint gesture types: `VeryShort`, `Short`, `Normal`, `Hold`, `DoubleTap`, `TripleTap`. |
| **Threshold Parameters** | [`config.rs`](src/config.rs) | Defines and validates timing thresholds (`t_vs`, `t_s`, `t_n`, `t_h`, `t_d`) with strict inequality checks. |
| **Macro Object** | [`macros.rs`](src/macros.rs) | Implements the Macro abstraction (`MacroId`, `MacroPlan`, `MacroRegistry`) — name, description, plan. |
| **Macro Mapper / Executor** | [`binding.rs`](src/binding.rs), [`executor.rs`](src/executor.rs) | Maps `(KeyId, Gesture)` → `MacroId`, and executes macros deterministically. |
| **Error and Time Abstractions** | [`error.rs`](src/error.rs), [`time.rs`](src/time.rs) | Logical error definitions and optional future time source abstraction. |

---

## 2. Theoretical Alignment

The implementation directly follows the logic and structure described in the **AUMM paper**:

### 2.1 Temporal Gesture Logic

> “Gestures are defined as logical conditions on sequences of key down/up events parameterized by temporal thresholds.”  
> — *Abstract Universal Macro Model – Theoretical Foundations*, §Gesture Categories and Definitions:contentReference[oaicite:0]{index=0}

Rust equivalent:
```rust
if dur < self.th.t_vs { TapKind::VeryShort }
else if dur < self.th.t_s { TapKind::Short }
else { TapKind::Normal }
````

Each gesture bucket corresponds exactly to the paper’s $T_{VS}$, $T_S$, $T_H$, and $T_D$ thresholds.

* `$T_{VS}$` → `t_vs`: maximum for **Very Short Tap**
* `$T_S$` → `t_s`: maximum for **Short Tap**
* `$T_H$` → `t_h`: **Hold** threshold
* `$T_D$` → `t_d`: **Inter-tap interval** (double/triple aggregation window)

---

### 2.2 Disjoint Logical Sets

> “Each gesture condition yields a boolean value; the categories are mutually exclusive by design using strict inequalities.”
> — *AUMM*, §Formal Logic and Conflict Avoidance

In `aumm_core`, this is enforced by:

* Strict `<` comparisons for duration ranges.
* Exclusive states (`Pressed`, `WaitingSecond`, `WaitingThird`, `Idle`).
* One `GestureEvent` emitted per key per sequence — no overlapping triggers.

Thus, a press sequence can be only **one** of {VeryShort, Short, Normal, Hold, DoubleTap, TripleTap}, never multiple.

---

### 2.3 Multi-Tap “Lookahead” Logic

> “The system may delay the final decision for a double-tap just long enough to see if a third tap occurs.”
> — *AUMM*, §Formal Logic and Conflict Avoidance

This is realized in the state machine through:

* `WaitingSecond` and `WaitingThird` states.
* `InputEvent::Tick(now)` — a deterministic, timer-less trigger driven by timestamps.
* Explicit `t_d`-based decision boundaries.

When `Tick(now)` exceeds the waiting threshold, the recognizer emits a single, final gesture event.

---

### 2.4 Determinism and Testability

> “We design the recognizer to be deterministic and testable, ideally timerless (driven by timestamps).”
> — *AUMM*, §System Architecture

`Recognizer::feed()` is pure and deterministic:

```rust
pub fn feed(&mut self, ev: InputEvent) -> Vec<GestureEvent>;
```

It produces the same result for the same input sequence regardless of runtime scheduling —
enabling **exact replay testing** and **formal verification** of gesture behavior.

Each subsystem is unit-tested in isolation:

* `recognizer_test.rs` simulates full event sequences and time progressions.
* `binding_test.rs`, `executor_test.rs`, `macros_test.rs` validate mapping and macro dispatch logic.
* `config_test.rs` verifies threshold ordering (`t_vs < t_s < t_n < t_h`).

---

### 2.5 Human Factors Compliance

> “We recommend keeping the number of active gesture types within 7±2 … aligning with human cognitive limits.”
> — *AUMM*, §Human Factors and Gesture Limitations

`aumm_core` implements **six** gesture categories (tap variants, hold, double, triple),
remaining within the paper’s cognitive guideline.
The system can be extended to new gestures (quadruple-tap, tap-and-hold)
while remaining formally verifiable and conflict-free.

---

### 2.6 Macro Plan Separation

> “Separating what the macro does (plan) from how it’s triggered (condition) makes the system extensible and testable.”
> — *AUMM*, §Macro Objects and Conditions

In `aumm_core`:

```rust
MacroRegistry::register(MacroId("mute".into()), LogPlan(log, "muted".into()));
BindingTable::bind(KeyId("F1".into()), Gesture::DoubleTap, MacroId("mute".into()));
Executor::handle(GestureEvent { ... });
```

This one-to-one reflection of the paper’s *condition → plan* mapping provides full modularity:
gesture logic and macro behavior can evolve independently.

---

## 3. Design Principles

| Principle           | Description                                                                                  |
| ------------------- | -------------------------------------------------------------------------------------------- |
| **Determinism**     | No internal timers or randomness; every decision is timestamp-driven.                        |
| **Isolation**       | Each key has its own finite state machine. Gestures on different keys never interfere.       |
| **Non-overlap**     | Thresholds and states are strictly disjoint, guaranteeing one gesture per event sequence.    |
| **Extensibility**   | New gestures can be added by extending `Gesture` and the FSM without changing other logic.   |
| **Configurability** | All timing constants (`Thresholds`) are externally configurable and validated.               |
| **Testability**     | Every behavior is reproducible and unit-testable, reflecting the paper’s verification goals. |

---

## 4. Example Timeline

| Gesture        | Press/Release Pattern                        | Condition                   | Fires       |
| -------------- | -------------------------------------------- | --------------------------- | ----------- |
| **Single Tap** | `Down → Up (Δ < t_h)` then wait `t_d`        | No second press occurs      | Tap(VS/S/N) |
| **Double Tap** | Two press-release cycles within `t_d`        | No third press within `t_d` | DoubleTap   |
| **Triple Tap** | Three press-release cycles, each gap ≤ `t_d` | Third within `t_d`          | TripleTap   |
| **Hold**       | Single press held ≥ `t_h`                    | Duration test               | Hold        |

---

## 5. Example Test Sequence

```rust
// 1. short single tap
r.feed(ke("A", KeyState::Down, 100));
r.feed(ke("A", KeyState::Up, 180));
r.feed(InputEvent::Tick(400));
// => emits Gesture::Tap(TapKind::Short)

// 2. double tap with triple enabled
r.feed(ke("A", KeyState::Down, 0));
r.feed(ke("A", KeyState::Up, 60));
r.feed(ke("A", KeyState::Down, 200));
r.feed(ke("A", KeyState::Up, 320));
r.feed(InputEvent::Tick(570));
// => emits Gesture::DoubleTap

// 3. hold
r.feed(ke("A", KeyState::Down, 0));
r.feed(ke("A", KeyState::Up, 700));
// => emits Gesture::Hold
```

---

## 6. Theoretical Traceability Summary

| AUMM Section                          | `aumm_core` Mechanism                              |
| ------------------------------------- | -------------------------------------------------- |
| *Gesture Categories and Definitions*  | `TapKind`, `Gesture`, duration buckets             |
| *Formal Logic and Conflict Avoidance* | Strict inequalities, one FSM per key               |
| *Parameter Guidelines*                | `Thresholds::sanity_check()` ensures logical order |
| *System Architecture*                 | `Recognizer` → `Executor` pipeline                 |
| *Macro Objects and Conditions*        | `MacroPlan`, `MacroRegistry`, `BindingTable`       |
| *Human Factors*                       | Six clear gestures (<7±2)                          |
| *Testability*                         | Modular unit tests per component                   |

---

## 7. Example Usage

```rust
use aumm_core::*;

fn main() {
    let mut recognizer = Recognizer::default();
    let mut bindings = BindingTable::new();
    let mut registry = MacroRegistry::new();

    registry.register(MacroId("say_hello".into()), macros::LogPlan(
        std::sync::Arc::new(std::sync::Mutex::new(vec![])),
        "Hello, world!".into(),
    ));
    bindings.bind("F1".into(), Gesture::Tap(TapKind::Short), MacroId("say_hello".into()));

    // simulate input
    let mut out = recognizer.feed(InputEvent::Key(KeyEvent { key: "F1".into(), state: KeyState::Down, ts_ms: 0 }));
    out.extend(recognizer.feed(InputEvent::Key(KeyEvent { key: "F1".into(), state: KeyState::Up, ts_ms: 100 })));
    out.extend(recognizer.feed(InputEvent::Tick(400)));

    let ex = Executor::new(&bindings, &registry);
    for g in out {
        ex.handle(&g);
    }
}
```

---

## 8. License

MIT © 2025 — *Based on the Abstract Universal Macro Model (Theoretical Foundations, 2025)*

---

## 9. References

* *Abstract Universal Macro Model – Theoretical Foundations*, 2025.
  (Sections: Gesture Categories, Formal Logic and Conflict Avoidance, Parameter Guidelines, System Architecture)
* Nielsen, Jakob. “Response Times: 3 Important Limits.” *NN/g*, 1993.
* *The Magical Number Seven, Plus or Minus Two*, G. A. Miller, 1956.
* Wikipedia: [Double-click](https://en.wikipedia.org/wiki/Double-click), [Triple-click](https://en.wikipedia.org/wiki/Triple-click)

---