use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::combatant::Combatant;
use crate::effect::Effect;
use crate::item::Item;
use crate::move_mod::Move;
use crate::assets::all_moves::AiData;

pub struct Enemy {
    pub alive: bool,
    pub name: &'static str,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub health: f32,
    pub max_health: f32,
    pub max_attack: f32,
    pub max_defense: f32,
    pub max_speed: f32,
    pub level: i32,
    pub moves: [Option <Arc<Move>>; 4],
    pub items: Vec<Arc<Item>>,
    pub held_item: Option<Arc<Item>>,
    pub aggression: f32,
    pub effects: Arc<Mutex<Vec<Effect>>>,
    pub ai_data: Arc<HashMap<&'static str, AiData>>,
    pub crit_chance: f32,
}

impl Combatant for Enemy {
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

    fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
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

    fn name(&self) -> &str {
        &self.name
    }
    fn apply_effect(&mut self, effect: Mutex<Effect>) {
        let mut effects = self.effects.lock().unwrap();
        effects.push(effect.into_inner().unwrap());
    }
    fn clear_effect(&mut self, effect: Mutex<Effect>) {
        let mut effects = self.effects.lock().unwrap();
        effects.retain(|e| e.name != effect.lock().unwrap().name);
    }

    fn has_effect(&self, effect: &Mutex<Effect>) -> bool {
        let effects = self.effects.lock().unwrap();
        effects.iter().any(|e| e.name == effect.lock().unwrap().name)
    }

    fn get_effect_list(&self) -> Vec<Arc<Mutex<Effect>>> {
        let mut effect_list = Vec::new();
        for effect in &*self.effects.lock().unwrap() {
            effect_list.push(Arc::new(effect.clone()));
        }
        effect_list
    }
    

    fn known_moves(&self) -> Vec<Arc<Move>> {
            let mut known_moves = Vec::new();
            for mov in &self.moves {
                if let Some(m) = mov {
                    known_moves.push(m.clone());
                }
            }
            known_moves
    }


    fn pick_move<'a>(&self, enemy: &'a mut (dyn Combatant + 'a)) -> Arc<Move> {
        let mut best_move: Option<Arc<Move>> = None;
        let mut best_score: f32 = f32::MIN;
    
        // Factors that dynamically adjust the behavior based on AI's current state
        let health_ratio = self.health / self.max_health;  // AI's current health as a percentage
        let aggression_threshold = 0.9;  // Aggression above this makes the AI less likely to heal
        let danger_threshold = 0.3;  // Health ratio below this makes the AI favor defensive moves
        
        for mov in self.moves.iter().filter_map(|m| m.as_ref()) {
            let mut move_score: f32 = 0.0;
            let name = mov.name;
    
            let ai_data = match self.ai_data.get(&name) {
                Some(data) => data,
                None => {
                    println!("Error: Move not found in AI_MOVES");
                    continue;
                }
            };
    
            // 1. **Self-Damage Avoidance**: Skip any move that could make the AI KO itself.
            if ai_data.self_damage > self.health {
                continue;
            }
    
            // 2. **Healing Moves**: Prioritize healing moves when health is low relative to aggression.
            if health_ratio < danger_threshold && ai_data.healthinc > 0.0 {
                let healing_priority = if self.aggression < 0.5 {
                    2.0  // AI prioritizes healing more if aggression is lower
                } else {
                    1.0  // Moderate focus on healing if aggression is moderate
                };
                move_score += healing_priority * ai_data.healthinc;
            }
    
            // 3. **Aggression Modulation**: If aggression is low, favor defensive moves (heal/defense boost).
            if self.aggression < 0.2 {
                if ai_data.defenseinc > 0.0 {
                    move_score += 1.2;  // Prioritize defense boost moves
                }
                if ai_data.healthinc > 0.0 {
                    move_score += 1.0;  // Prioritize healing
                }
            } else if self.aggression >= 0.2 && self.aggression <= 0.5 {
                // Balance between offensive and defensive moves
                move_score += ai_data.damage * 0.3 + ai_data.defenseinc * 0.7;
            } else if self.aggression > 0.5 && self.aggression <= 0.9 {
                // Focus on offensive moves more as aggression increases
                move_score += ai_data.damage * 0.7 + ai_data.attackinc * 0.4;
            } else if self.aggression > aggression_threshold {
                // Highly aggressive AI: Prioritize attack and damage, even at the cost of defense
                move_score += ai_data.damage * 1.0;
            }
    
            // 4. **Speed Considerations**: Prioritize speed increase if it allows AI to move before the enemy.
            if ai_data.speedinc > 0.0 && (self.speed + ai_data.speedinc > *enemy.speed()) {
                move_score += 0.8;  // Speed advantage has significant value
            }
    
            // 5. **Killing Moves**: If a move can defeat the player, heavily prioritize it.
            if ai_data.damage > *enemy.health() {
                move_score += 2.5;  // This is a high priority condition
            }
    
            // 6. **Stat Comparison**: Adjust AI moves based on relative weaknesses in defense, speed, and attack.
            if *enemy.attack() > self.attack {
                move_score += ai_data.defenseinc * 1.5;  // Increase defense if enemy attack is stronger
            }
            if *enemy.defense() > self.attack {
                if ai_data.ignores_defense {
                    move_score += 1.0;  // Prioritize moves that ignore defense if enemy has high defense
                }
            }
            if *enemy.speed() > self.speed {
                move_score += ai_data.speedinc * 1.2;  // Prioritize speed boosts if the enemy is faster
            }
    
            // 7. **Effect-Based Scoring**: Prioritize moves that apply debuffs or status effects.
            if ai_data.has_effect {
                move_score += 0.5;
            }
    
            // 8. **Priority Moves**: Take into account the move's priority. 
            if mov.priority == 1 {
                move_score += 0.5;  // Higher priority moves are generally more favorable
            } else if mov.priority == -1 {
                move_score -= 0.5;  // Lower priority moves are less favorable
            }
    
            // DEBUG: Logging move scores for analysis
            println!("DEBUG: Move {} has a score of {}", name, move_score);
    
            // Update the best move if the current move's score is better than the previous best
            if move_score > best_score {
                best_score = move_score;
                best_move = Some((**mov).clone().into());
            }
        }
    
        best_move.unwrap().into()
    }

    fn get_crit_chance(&self) -> f32 {
        self.crit_chance
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn moves(&self) -> [Option<Arc<Move>>; 4] {
        self.moves.clone()
    }
    
}

impl Enemy {
    pub fn clone(&self) -> Enemy {
        Enemy {
            alive: self.alive,
            name: self.name,
            attack: self.attack,
            defense: self.defense,
            speed: self.speed,
            health: self.health,
            max_health: self.max_health,
            max_attack: self.max_attack,
            max_defense: self.max_defense,
            max_speed: self.max_speed,
            level: self.level,
            moves: self.moves.clone(),
            items: self.items.clone(),
            held_item: match &self.held_item {
                Some(item) => Some(item.clone()),
                None => None,
            },
            aggression: self.aggression,   
            effects: self.effects.clone(),
            ai_data: self.ai_data.clone(),
            crit_chance: self.crit_chance,
        }
    }



}



