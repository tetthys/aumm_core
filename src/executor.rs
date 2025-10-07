use crate::{ binding::BindingTable, gesture::GestureEvent, macros::MacroRegistry };

/// Consumes `GestureEvent`s and triggers macro plans via registry.
pub struct Executor<'a> {
    pub bindings: &'a BindingTable,
    pub registry: &'a MacroRegistry,
}

impl<'a> Executor<'a> {
    pub fn new(bindings: &'a BindingTable, registry: &'a MacroRegistry) -> Self {
        Self { bindings, registry }
    }

    /// Execute the macro bound to the gesture (if any). Returns `true` if a macro was run.
    pub fn handle(&self, ge: &GestureEvent) -> bool {
        if let Some(mid) = self.bindings.resolve(&ge.key, &ge.gesture) {
            if let Some(plan) = self.registry.get(mid) {
                return plan.execute();
            }
        }
        false
    }
}
