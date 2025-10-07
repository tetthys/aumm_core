use serde::{ Deserialize, Serialize };
use hashbrown::HashMap;
use crate::{ gesture::Gesture, key::KeyId, macros::MacroId };

/// Lookup key: (KeyId, Gesture)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BindingKey(pub KeyId, pub Gesture);

/// Table mapping a (key, gesture) to a macro id.
#[derive(Debug, Default)]
pub struct BindingTable {
    map: HashMap<BindingKey, MacroId>,
}

impl BindingTable {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn bind(&mut self, key: KeyId, gesture: Gesture, macro_id: MacroId) {
        self.map.insert(BindingKey(key, gesture), macro_id);
    }

    pub fn resolve(&self, key: &KeyId, gesture: &Gesture) -> Option<&MacroId> {
        self.map.get(&BindingKey(key.clone(), *gesture))
    }
}
