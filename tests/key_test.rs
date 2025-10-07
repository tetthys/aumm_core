use aumm_core::KeyId;
use std::collections::HashSet;

#[test]
fn keyid_from_into_string() {
    let k: KeyId = "F1".into();
    assert_eq!(k.0, "F1");
}

#[test]
fn keyid_hash_equality() {
    let mut set = HashSet::new();
    set.insert(KeyId("A".into()));
    set.insert(KeyId("A".into())); // duplicate
    set.insert(KeyId("B".into()));
    assert_eq!(set.len(), 2);
}
