use std::sync::{Mutex, Arc};

use crate::combatant::Combatant;
use crate::effect::Effect;
use crate::move_mod::Move;
use crate::item::Item;


pub struct Character {
    pub alive: bool,
    pub name: String,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub health: f32,
    pub max_health: f32,
    pub max_attack: f32,
    pub max_defense: f32,
    pub max_speed: f32,
    pub level: i32,
    pub experience: f32,
    pub moves: [Option <Arc<Move>>; 4],
    pub known_moves: Vec<Arc<Move>>,
    pub items: Vec<Arc<Item>>,
    pub held_item: Option<Arc<Item>>,
    pub effects: Arc<Mutex<Vec<Effect>>>,
}

impl Combatant for Character {
    fn health(&mut self) -> &mut f32 {
        &mut self.health
    }

    fn attack(&mut self) -> &mut f32 {
        &mut self.attack
    }

    fn defense(&mut self) -> &mut f32 {
        &mut self.defense
    }

    fn speed(&mut self) -> &mut f32 {
        &mut self.speed
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn max_health(&self) -> f32 {
        self.max_health
    }

    fn max_attack(&self) -> f32 {
        self.max_attack
    }

    fn max_defense(&self) -> f32 {
        self.max_defense
    }

    fn max_speed(&self) -> f32 {
        self.max_speed
    }
    
    fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }

    // Apply a new effect by adding it to the character's list of effects
    fn apply_effect(&mut self, effect: Mutex<Effect>) {
        let mut effects = self.effects.lock().unwrap();
        effects.push(effect.into_inner().unwrap());
    }

    // Clear a specific effect by name from the character's list of effects
    fn clear_effect(&mut self, effect: Mutex<Effect>) {
        let mut effects = self.effects.lock().unwrap();
        effects.retain(|e| e.name != effect.lock().unwrap().name);
    }

    // Check if the character has a specific effect by name
    fn has_effect(&self, effect: &Mutex<Effect>) -> bool {
        let effects = self.effects.lock().unwrap();
        effects.iter().any(|e| e.name == effect.lock().unwrap().name)
    }

    // Allow the player to pick a move by presenting available moves
    fn pick_move<'a>(&self, enemy: &'a mut (dyn Combatant + 'a)) -> Arc<Move> {
        println!("Choose a move to use on {}:", enemy.name());
        let mut i = 0;
        for mov in &self.moves {
            if let Some(m) = mov {
                println!("{}: {}", i, m.name);
                println!("{}", m.description);
                println!("========================================");   
            }
            i += 1;
        }
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<usize>().unwrap();
        self.moves[choice].clone().unwrap()
    }

    // Return a clone of all known moves
    fn known_moves(&self) -> Vec<Arc<Move>> {
        self.known_moves.clone()
    }

    // Get a list of all active effects as Arc<Mutex<Effect>>
    fn get_effect_list(&self) -> Vec<Arc<Mutex<Effect>>> {
        let mut effect_list = Vec::new();
        for effect in &*self.effects.lock().unwrap() {
            effect_list.push(Arc::new(effect.clone()));
        }
        effect_list
    }
}



impl Character {
    pub fn new(
        alive: bool,
        name: String,
        attack: f32,
        defense: f32,
        speed: f32,
        health: f32,
        max_health: f32,
        max_attack: f32,
        max_defense: f32,
        max_speed: f32,
        level: i32,
        experience: f32,
        moves: [Option<Arc<Move>>; 4],
        known_moves: Vec <Arc<Move>>,
        items: Vec<Arc<Item>>,
        held_item: Option<Arc<Item>>
    ) -> Character {
        Character {
            alive,
            name,
            attack,
            defense,
            speed,
            health,
            max_health,
            max_attack,
            max_defense,
            max_speed,
            level,
            experience,
            moves,
            known_moves,
            items,
            held_item,
            effects: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn check_level_up(&self) -> bool {
        let currentxp = self.experience;
        let current_level = self.level;
        let next_level_xp = self.get_current_level_xp() + current_level as f32 * 1.2;
        if currentxp >= next_level_xp {
            return true;
        }
        false
    }

    pub fn get_current_level_xp(&self) -> f32 {
        let current_level = self.level;
        let mut current_level_xp: f32 = 0.0;
        for i in 1..current_level {
            current_level_xp += i as f32 * 1.2;
        }
        current_level_xp.round()
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.attack += 1.0;
        self.defense += 1.0;
        self.speed += 1.0;
        self.health += 1.0;
    }

    pub fn clone (&self) -> Character {
        Character {
            alive: self.alive,
            name: self.name.clone(),
            attack: self.attack,
            defense: self.defense,
            speed: self.speed,
            health: self.health,
            max_health: self.max_health,
            max_attack: self.max_attack,
            max_defense: self.max_defense,
            max_speed: self.max_speed,
            level: self.level,
            experience: self.experience,
            known_moves: self.known_moves.clone(),
            moves: self.moves.clone(),
            items: self.items.clone(),
            held_item: self.held_item.clone(),
            effects: self.effects.clone(),
        }
    }

    pub fn copy (&mut self, other: &Character) {
        self.alive = other.alive;
        self.name = other.name.clone();
        self.attack = other.attack;
        self.defense = other.defense;
        self.speed = other.speed;
        self.health = other.health;
        self.max_health = other.max_health;
        self.level = other.level;
        self.experience = other.experience;
        self.known_moves = other.known_moves.clone();
        self.moves = other.moves.clone();
        self.items = other.items.clone();
        self.held_item = other.held_item.clone();
    }

}
