#![feature(trait_upcasting)]
use std::{
    sync::{Arc, Mutex},
    vec,
};

use assets::{all_enemies, all_moves};
use battle::Battle;
use character::Character;
use combatant::Combatant;
use server::handler::Handler;
extern crate lazy_static;

mod assets;
mod battle;
mod character;
mod cloneablefn;
mod cloneablefn_combatant;
mod combatant;
mod effect;
mod enemy;
mod item;
mod move_mod;
mod server;
mod battle_multiplayer;

fn main() {
    println!("Welcome to the game!");
    println!("Enter your name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    let player = Character {
        name: name.to_string(),
        health: 100.0,
        max_health: 100.0,
        moves: [
            move_mod::MOVE_DICTIONARY.get("Dismantle").cloned(),
            move_mod::MOVE_DICTIONARY.get("Heal").cloned(),
            move_mod::MOVE_DICTIONARY.get("Sacrifice").cloned(),
            move_mod::MOVE_DICTIONARY.get("Focus").cloned(),
        ],
        known_moves: Vec::new(),
        effects: Arc::new(Mutex::new(vec![])),
        items: Vec::new(),
        alive: true,
        attack: 10.0,
        defense: 10.0,
        speed: 10.0,
        max_attack: 10.0,
        max_defense: 10.0,
        max_speed: 10.0,
        level: 1,
        experience: 0.0,
        held_item: None,
        crit_chance: 0.25,
    };

    let player_team: Vec<Arc<Mutex<dyn Combatant>>> = vec![Arc::new(Mutex::new(player))];
    let enemy_team: Vec<Arc<Mutex<dyn Combatant>>> =
        vec![Arc::new(Mutex::new(all_enemies::DEBUG.clone()))];
    let battle = Arc::new(Mutex::new(Battle::new(player_team, enemy_team, true)));
    let mut battle_lock = battle.lock().unwrap();
    let handler = Arc::new(Mutex::new(Handler::new("127.0.0.1".to_string(), 8080, Arc::clone(&battle))));
    let handler_clone = Arc::clone(&handler);
    battle_lock.init_server(handler_clone);
    battle_lock.run_battle();
    handler.lock().unwrap().run_server();
    drop(battle_lock);
}
