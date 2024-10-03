use std::process::exit;

use assets::{all_enemies, all_items, all_moves};

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
    println!("Enter your name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    let mut player = character::Character::new(
        true,
        name.to_string(),
        10.0,
        10.0,
        10.0,
        100.0,
        100.0,
        10.0,
        10.0,
        10.0,
        1,
        0.0,
        [
            Some(std::sync::Arc::new(all_moves::DISMANTLE.clone())),
            Some(std::sync::Arc::new(all_moves::FOCUS.clone())),
            Some(std::sync::Arc::new(all_moves::FLAMETHROWER.clone())),
            Some(std::sync::Arc::new(all_moves::HEAL.clone())),
        ],
        vec![],
        vec![],
        None,
        0.2,
    );
    let mut enemy = all_enemies::DEBUG.clone();
    player.items.push(all_items::ENERGY_DRINK.clone().into());
    let mut battle = battle::Battle {
        player: &mut player,
        enemy: &mut enemy,
        round: 1,
    };
    battle.run_battle();
    exit(0);
}

