use aumm_core::{ BindingTable, BindingKey, KeyId, Gesture, TapKind, MacroId };

#[test]
fn bind_and_resolve_existing_pair() {
    let mut bt = BindingTable::new();
    bt.bind("F1".into(), Gesture::Tap(TapKind::Short), MacroId("m1".into()));
    let mid = bt.resolve(&KeyId("F1".into()), &Gesture::Tap(TapKind::Short)).unwrap();
    assert_eq!(mid.0, "m1");
}

#[test]
fn resolve_returns_none_if_not_bound() {
    let bt = BindingTable::new();
    assert!(bt.resolve(&KeyId("F9".into()), &Gesture::Tap(TapKind::Normal)).is_none());
}

#[test]
fn bindingkey_is_hashable_and_equatable() {
    // compile-time check via usage in std map
    use std::collections::HashMap;
    let mut m: HashMap<BindingKey, &str> = HashMap::new();
    let k = BindingKey(KeyId("A".into()), Gesture::Tap(TapKind::VeryShort));
    m.insert(k, "ok");
    assert_eq!(
        m.get(&BindingKey(KeyId("A".into()), Gesture::Tap(TapKind::VeryShort))),
        Some(&"ok")
    );
}
