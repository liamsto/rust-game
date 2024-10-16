use std::sync::{Arc, Mutex};

use crate::combatant::Combatant;
use crate::effect::Effect;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POISON: Mutex<Effect> = Effect::new(
        "Poison",
        "Poison deals damage and reduces defense while active.",
        5,
        Arc::new(|on: &mut dyn Combatant| {
            if POISON.lock().unwrap().duration == 0 {
                println!("{} has recovered from poison!", on.name());
                Combatant::clear_effect(on, POISON.lock().unwrap().clone().into());
                return;
            }
            if !POISON.lock().unwrap().applied {
                let defense = *on.defense();
                Combatant::set_defense(on, defense * 0.9);
            }
            let health = *on.health();
            Combatant::set_health(on, health - on.max_health() * 0.02);
            POISON.lock().unwrap().duration -= 1;
        })
    );
    pub static ref BURN: Mutex<Effect> = Effect::new(
        "Burn",
        "Burn deals damage while active.",
        5,
        Arc::new(|on: &mut dyn Combatant| {
            println!("{} is on fire!", on.name());
            println!("DEBUG: Burn duration: {}", BURN.lock().unwrap().duration);
            if BURN.lock().unwrap().duration == 0 {
                println!("{} is no longer on fire!", on.name());
                Combatant::clear_effect(on, BURN.lock().unwrap().clone().into());
                return;
            }
            let health = *on.health();
            Combatant::set_health(on, health - on.max_health() * 0.05);
            BURN.lock().unwrap().duration -= 1;
        })
    );
}
