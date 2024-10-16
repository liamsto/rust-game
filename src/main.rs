#![feature(trait_upcasting)]

use std::{process::exit, sync::{Arc, Mutex}};

use assets::{all_enemies, all_items, all_moves};
use combatant::Combatant;
use battle::Battle;
extern crate lazy_static;

mod character;
mod move_mod;
mod item;
mod battle;
mod assets;
mod enemy;
mod combatant;
mod effect;
mod cloneablefn;
mod cloneablefn_combatant;


    
fn main() {
    println!("Welcome to the game!");
    println!("Enter your name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();
    let player = character::Character {
        alive: true,
        name,
        attack: 10.0,
        defense: 10.0,
        speed: 10.0,
        health: 100.0,
        max_health: 100.0,
        max_attack: 10.0,
        max_defense: 10.0,
        max_speed: 10.0,
        level: 1,
        moves: [Some(all_moves::PUNCH.clone().into()), Some(all_moves::DISMANTLE.clone().into()), Some(all_moves::FLAMETHROWER.clone().into()), None],
        items: vec![all_items::ENERGY_DRINK.clone().into()],
        held_item: None,
        effects: Arc::new(Mutex::new(Vec::new())),
        crit_chance: 0.1,
        experience: 0.0,
        known_moves: vec![all_moves::PUNCH.clone().into(), all_moves::DISMANTLE.clone().into(), all_moves::FLAMETHROWER.clone().into()],
    };

    

    let enemies: Vec<Arc<Mutex<dyn Combatant>>> = vec![Arc::new(Mutex::new(all_enemies::DEBUG.clone()))];
    let player: Vec<Arc<Mutex<dyn Combatant>>> = vec![Arc::new(Mutex::new(player))];

    let mut battle = Battle::new(player, enemies);
    battle.run_battle();
    exit(0);

}

