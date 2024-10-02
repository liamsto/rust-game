use std::vec;

use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::assets::all_moves;
use crate::enemy::Enemy;

lazy_static!(
    pub(crate) static ref DEBUG: Enemy = Enemy {
        alive: true,
        name: "DEBUG",
        attack: 5.0,
        defense: 5.0,
        speed: 5.0,
        health: 100.0,
        max_health: 100.0,
        max_attack: 5.0,
        max_defense: 5.0,
        max_speed: 5.0,
        level: 1,
        moves: [Some(Arc::new(all_moves::DISMANTLE.clone())), Some(Arc::new(all_moves::HEAL.clone())), Some(Arc::new(all_moves::SACRIFICE.clone())), Some(Arc::new(all_moves::FOCUS.clone()))],
        items: vec![],
        held_item: None,
        aggression: 0.6,
        effects: Arc::new(Mutex::new(Vec::new())),
        ai_data: Arc::new(all_moves::DEBUG_AI_MOVES.clone()), 
    };
);