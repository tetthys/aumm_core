use serde::{ Deserialize, Serialize };

/// Opaque key identifier.
/// You may map this to scan codes, HID usages, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyId(pub String);

impl<K: Into<String>> From<K> for KeyId {
    fn from(k: K) -> Self {
        KeyId(k.into())
    }
}
