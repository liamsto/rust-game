use std::sync::{Arc, Mutex};

use crate::cloneablefn::CloneableFn;
use crate::combatant::Combatant;

// Custom trait for clonable function pointers

// Implement CloneableFn for any clonable function

#[derive(Clone)]
pub struct Effect {
    pub name: &'static str,
    pub description: &'static str,
    pub duration: u32,
    pub effect_fn: Arc<dyn CloneableFn + Sync + Send>, // Use CloneableFn for cloning
    pub applied: bool,
}

impl Effect {
    pub fn new(
        name: &'static str,
        description: &'static str,
        duration: u32,
        effect_fn: Arc<dyn CloneableFn + Sync + Send>,
    ) -> Mutex<Effect> {
        Mutex::new(Effect {
            name,
            description,
            duration,
            effect_fn,
            applied: false,
        })
    }

    // Manually implement Clone for Effect
    pub fn clone(&self) -> Mutex<Effect> {
        Mutex::new(Effect {
            name: self.name,
            description: self.description,
            duration: self.duration,
            effect_fn: self.effect_fn.clone(), // Custom clone for the function
            applied: self.applied,
        })
    }
    pub fn apply_effect(&mut self, on: &mut dyn Combatant) {
        (self.effect_fn)(on);
        self.duration -= 1;
    }
}
