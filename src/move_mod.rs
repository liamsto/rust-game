use std::sync::Arc;

use crate::{character::Character, cloneablefn_combatant::CloneableFnCombatant};

#[derive(Clone)]



pub struct Move {
    pub name: &'static str,
    pub description: &'static str,
    pub priority: i32,
    pub effect_fn: Arc<dyn CloneableFnCombatant + Send + Sync>,  
}

impl Move {
    pub fn new(name: &'static str, priority: i32, effect_fn: Arc<dyn CloneableFnCombatant + Send + Sync>) -> Arc<Self> {
        Arc::new(Move {
            name,
            description: "No description",
            priority,
            effect_fn,
        })
    }

    pub fn learn_move(&mut self, character: &mut Character) {
        character.known_moves.push(Arc::clone(&Arc::new(self.clone())));
    }

    pub fn clone(&self) -> Move {
        Move {
            name: self.name,
            priority: self.priority,
            description: self.description,
            effect_fn: Arc::clone(&self.effect_fn),  
        }
    }
}