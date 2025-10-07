use serde::{ Deserialize, Serialize };
use hashbrown::HashMap;

/// Stable handle to a registered macro implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MacroId(pub String);

/// Strategy object for executing a macro.
pub trait MacroPlan: Send + Sync {
    /// Execute the plan. Return `true` if success.
    fn execute(&self) -> bool;
}

/// Registry wires `MacroId` to a concrete `MacroPlan` implementation.
#[derive(Default)]
pub struct MacroRegistry {
    plans: HashMap<MacroId, Box<dyn MacroPlan>>,
}

impl MacroRegistry {
    pub fn new() -> Self {
        Self { plans: HashMap::new() }
    }

    pub fn register<P: MacroPlan + 'static>(&mut self, id: MacroId, plan: P) {
        self.plans.insert(id, Box::new(plan));
    }

    pub fn get(&self, id: &MacroId) -> Option<&Box<dyn MacroPlan>> {
        self.plans.get(id)
    }
}

/// A simple test plan that records execution into a shared log.
pub struct LogPlan(pub std::sync::Arc<std::sync::Mutex<Vec<String>>>, pub String);

impl MacroPlan for LogPlan {
    fn execute(&self) -> bool {
        if let Ok(mut v) = self.0.lock() {
            v.push(self.1.clone());
            true
        } else {
            false
        }
    }
}
