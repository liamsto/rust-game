use std::{collections::HashMap, sync::Arc};

use lazy_static::lazy_static;

use crate::{assets::all_moves::{self, MoveData}, character::Character, cloneablefn_combatant::CloneableFnCombatant};

#[derive(Clone)]

pub struct Move {
    pub name: &'static str,
    pub description: &'static str,
    pub priority: i32,
    pub effect_fn: Arc<dyn CloneableFnCombatant + Send + Sync>,
}

impl Move {
    pub fn new(
        name: &'static str,
        priority: i32,
        effect_fn: Arc<dyn CloneableFnCombatant + Send + Sync>,
    ) -> Arc<Self> {
        Arc::new(Move {
            name,
            description: "No description",
            priority,
            effect_fn,
        })
    }

    pub fn learn_move(&mut self, character: &mut Character) {
        character
            .known_moves
            .push(Arc::clone(&Arc::new(self.clone())));
    }

    pub fn clone(&self) -> Move {
        Move {
            name: self.name,
            priority: self.priority,
            description: self.description,
            effect_fn: Arc::clone(&self.effect_fn),
        }
    }

    pub fn serialize(&self) -> String {
        format!(
            "{}|{}",
            self.name,
            self.priority,
        )
    }

    pub fn deserialize(serialized: &str) -> Arc<Move> {
        let name = serialized.split('|').nth(0).unwrap();
        let move_dictionary = &MOVE_DICTIONARY;
        let move_arc = move_dictionary.get(name).unwrap();
        move_arc.clone()
    }

}


lazy_static!{
    pub static ref MOVE_DICTIONARY: HashMap<&'static str, Arc<Move>> = {
        let mut m = HashMap::new();
        m.insert("Dismantle", all_moves::DISMANTLE.clone());
        m.insert("Flamethrower", all_moves::FLAMETHROWER.clone());
        m.insert("Heal", all_moves::HEAL.clone());
        m.insert("Sacrifice", all_moves::SACRIFICE.clone());
        m.insert("Focus", all_moves::FOCUS.clone());
        m
    };
}
