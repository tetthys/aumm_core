use std::sync::{ Arc, Mutex };
use aumm_core::{ MacroRegistry, MacroId, macros::LogPlan };

#[test]
fn register_and_get_macro() {
    let mut reg = MacroRegistry::new();
    reg.register(MacroId("m1".into()), LogPlan(Arc::new(Mutex::new(vec![])), "x".into()));
    assert!(reg.get(&MacroId("m1".into())).is_some());
    assert!(reg.get(&MacroId("missing".into())).is_none());
}

#[test]
fn execute_logplan_appends_to_log() {
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut reg = MacroRegistry::new();
    reg.register(MacroId("m".into()), LogPlan(log.clone(), "ran".into()));
    let ok = reg.get(&MacroId("m".into())).unwrap().execute();
    assert!(ok);
    assert_eq!(log.lock().unwrap().as_slice(), ["ran"]);
}
