use std::sync::{ Arc, Mutex };
use aumm_core::{
    BindingTable,
    MacroRegistry,
    Executor,
    MacroId,
    macros::LogPlan,
    GestureEvent,
    Gesture,
    KeyId,
};

#[test]
fn executor_runs_macro_when_bound() {
    let mut bt = BindingTable::new();
    let mut reg = MacroRegistry::new();
    let log = Arc::new(Mutex::new(Vec::<String>::new()));

    bt.bind("K".into(), Gesture::DoubleTap, MacroId("mute".into()));
    reg.register(MacroId("mute".into()), LogPlan(log.clone(), "mute!".into()));
    let ex = Executor::new(&bt, &reg);

    let fired = ex.handle(
        &(GestureEvent { key: KeyId("K".into()), gesture: Gesture::DoubleTap, decided_at_ms: 10 })
    );
    assert!(fired);
    assert_eq!(log.lock().unwrap().as_slice(), ["mute!"]);
}

#[test]
fn executor_noop_when_unbound() {
    let bt = BindingTable::new();
    let reg = MacroRegistry::new();
    let ex = Executor::new(&bt, &reg);

    let fired = ex.handle(
        &(GestureEvent { key: KeyId("X".into()), gesture: Gesture::Hold, decided_at_ms: 0 })
    );
    assert!(!fired);
}
