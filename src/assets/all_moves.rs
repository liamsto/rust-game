use lazy_static::lazy_static;
use crate::{assets::all_effects, combatant::Combatant, move_mod::Move};
use std::{collections::HashMap, sync::Arc};


//BASIC MOVES
lazy_static! {
    pub(crate) static ref PUNCH: Move = Move {
        name: "Punch",
        priority: 0,
        description: "A basic punch",
        effect_fn: Arc::new(|character: &mut dyn Combatant, enemy: &mut dyn Combatant| {
            let user_attack = *character.attack();
            let enemy_defense = *enemy.defense();
            let mut damage = 10.0 + (user_attack - enemy_defense) * 0.5;
            damage = character.crit(damage);
            if damage <= 0.0 {
                println!("The attack was ineffective!");
                return;
            }
            *enemy.health() -= damage;
            println!("{} took {} damage!", enemy.name(), damage);
            enemy.check_death();
            return;
        }),
    };

    pub(crate) static ref DISMANTLE: Move = Move {
        name: "Dismantle",
        priority: 0,
        description: "An attack that ignores enemy defense",
        effect_fn: Arc::new(|character: &mut dyn Combatant, enemy: &mut dyn Combatant | {
            let user_attack = *character.attack();
            let mut damage = 5.0 + user_attack * 0.75;
            damage = character.crit(damage);
            if damage <= 0.0 {
                println!("The attack was ineffective!");
                return;
            }
            *enemy.health() -= damage;
            println!("{} took {} damage!", enemy.name(), damage);
            enemy.check_death();
            return;
        }),
    };
    pub(crate) static ref FLAMETHROWER: Move = Move {
        name: "Flamethrower",
        priority: 0,
        description: "Deals burn damage",
        effect_fn: Arc::new(|character: &mut dyn Combatant, enemy: &mut dyn Combatant| {
            let user_attack = *character.attack();
            let enemy_defense = *enemy.defense();
            let mut damage = 5.0 + (user_attack - enemy_defense) * 0.5;
            damage = character.crit(damage);
            if damage <= 0.0 {
                println!("The attack was ineffective!");
                return;
            }
            *enemy.health() -= damage;
            println!("{} took {} damage!", enemy.name(), damage);
            if enemy.has_effect(&all_effects::BURN.lock().unwrap().clone()) {
                println!("The enemy is already burned!");
                return;
            }
            println!("{} was burned!", enemy.name());
            enemy.apply_effect(all_effects::BURN.lock().unwrap().clone().into());
            enemy.check_death();
            return;
        })
    };

    pub(crate) static ref FOCUS: Move = Move {
        name: "Focus",
        priority: -1,
        description: "Raises attack and speed",
        effect_fn: Arc::new(|character: &mut dyn Combatant, _| {
            let attack = *character.attack();
            let speed = *character.speed();
            *character.attack() = attack * 1.1;
            *character.speed() = speed * 1.1;
            println!("{} focuses. {}'s attack and speed rose!", character.name(), character.name());
            return;
        }),
    };

    pub(crate) static ref HEAL: Move = Move {
        name: "Heal",
        priority: -1,
        description: "Heals the user",
        effect_fn: Arc::new(|character: &mut dyn Combatant, _| {
            let health = *character.health();
            let max_health = character.max_health();
            *character.health() = health + max_health * 0.15;
            if *character.health() > max_health {
                *character.health() = max_health;
            }
            println!("{} healed!", character.name());
            return;
        }),
    };

    pub(crate) static ref SACRIFICE: Move = Move {
        name: "Sacrifice",
        description: "Trades health for a large increase in attack",
        priority: 0,
        effect_fn: Arc::new(|character: &mut dyn Combatant, _| {
            let health = *character.health();
            let attack = *character.attack();
            *character.health() = health - 0.1 * character.max_health();
            *character.attack() = attack * 1.5;
            println!("{}'s health fell!", character.name());
            println!("{}", character.name().to_owned() + "'s attack rose sharply!");
            character.check_death();
            return;
        }),
    };
}


//struct to allow AI to make decisions without having to include the damage and healthinc values in the move struct, since they are only used by the AI
pub struct AiData {
    pub name: String,
    pub damage: f32,
    pub healthinc: f32,
    pub speedinc: f32,
    pub attackinc: f32,
    pub defenseinc: f32,
    pub has_effect: bool,
    pub ignores_defense: bool,
    pub self_damage: f32,
    pub priority: i32,
}

impl AiData {
    pub fn new(name: String, damage: f32, healthinc: f32, speedinc: f32, attackinc: f32, defenseinc: f32, has_effect: bool, ignores_defense: bool, self_damage: f32, priority: i32) -> AiData {
        AiData {
            name,
            damage,
            healthinc,
            speedinc,
            attackinc,
            defenseinc,
            has_effect,
            ignores_defense,
            self_damage,
            priority,
        }
    }


}


impl Clone for AiData {
    fn clone(&self) -> AiData {
        AiData {
            name: self.name.clone(),
            damage: self.damage,
            healthinc: self.healthinc,
            speedinc: self.speedinc,
            attackinc: self.attackinc,
            defenseinc: self.defenseinc,
            has_effect: self.has_effect,
            ignores_defense: self.ignores_defense,
            self_damage: self.self_damage,
            priority: self.priority
        }
    }
}

//AI MOVES
lazy_static!{
    pub static ref punch: AiData = AiData::new("Punch".to_string(), 10.0, 0.0, 0.0, 0.0, 0.0, false, false, 0.0, 0);
    pub static ref dismantle: AiData = AiData::new("Dismantle".to_string(), 5.0, 0.0, 0.0, 0.0, 0.0, false, true, 0.0, 0);
    pub static ref flamethrower: AiData = AiData::new("Flamethrower".to_string(), 5.0, 0.0, 0.0, 0.0, 0.0, true, false, 0.0, 0);
    pub static ref focus: AiData = AiData::new("Focus".to_string(), 0.0, 0.0, 0.0, 0.1, 0.0, false, false, 0.0, -1);
    pub static ref heal: AiData = AiData::new("Heal".to_string(), 0.0, 15.0, 0.0, 0.0, 0.0, false, false, 0.0, -1);
    pub static ref sacrifice: AiData = AiData::new("Sacrifice".to_string(), 0.0, 0.0, 0.0, 5.0, 0.0, false, false, 5.0, 0);
}

lazy_static!(
    pub static ref DEBUG_AI_MOVES: HashMap<&'static str, AiData> = {
        let mut m = HashMap::new();
        m.insert("Dismantle", dismantle.clone());
        m.insert("Sacrifice", sacrifice.clone());
        m.insert("Heal", heal.clone());
        m.insert("Focus", focus.clone());
        m
    };
);

