use std::sync::{Arc, Mutex};
use crate::combatant::Combatant;

pub struct Battle {
    pub player_team: Vec<Arc<Mutex<dyn Combatant>>>,
    pub enemy_team: Vec<Arc<Mutex<dyn Combatant>>>,
    pub order: Vec<Arc<Mutex<dyn Combatant>>>,
    pub round: u32,
}

impl Battle {
    pub fn new(player_team: Vec<Arc<Mutex<dyn Combatant>>>, enemy_team: Vec<Arc<Mutex<dyn Combatant>>>) -> Self {
        let order = Vec::new();
        let round = 0;
        Battle { player_team, enemy_team, order, round }
    }

    fn get_current_combatant(&self, index: usize) -> Arc<Mutex<dyn Combatant>> {
        Arc::clone(&self.order[index])
    }

    pub fn run_battle(&mut self) {
        self.calculate_order(); // Initialize the order once
        while self.player_team.iter().any(|x| x.lock().unwrap().alive()) && self.enemy_team.iter().any(|x| x.lock().unwrap().alive()) {
            println!("===================================Round {}!===================================", self.round);
    
            for i in 0..self.order.len() {
                let current_combatant = self.get_current_combatant(i);
                if !current_combatant.lock().unwrap().alive() {
                    continue; // Skip dead combatants
                }
    
                let is_player_controlled = {
                    let combatant = current_combatant.lock().unwrap();
                    combatant.is_player_controlled()
                };
    
                if is_player_controlled {
                    self.player_turn(&current_combatant);
                } else {
                    self.enemy_turn(&current_combatant);
                }
            }
    
            self.run_effects();
            self.calculate_order(); // Recalculate order after each round
            self.round += 1; // Increment the round counter
        }
    }

    fn calculate_order(&mut self) {
        self.order = self
            .player_team
            .iter()
            .chain(self.enemy_team.iter())
            .map(|x| Arc::clone(x))
            .collect();
        self.order.sort_by(|a, b| b.lock().unwrap().speed().partial_cmp(&a.lock().unwrap().speed()).unwrap());
    }

    fn player_turn(&mut self, player_arc: &Arc<Mutex<dyn Combatant>>) {
        let mut player = player_arc.lock().unwrap();
        println!("{}'s turn!", player.name());
        println!("{}'s health: {}", player.name(), player.get_health());
        println!(
            "Select a target to attack (1-{}): {}",
            self.enemy_team.len(),
            self.enemy_team
                .iter()
                .map(|x| x.lock().unwrap().name().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        let target: usize;
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<usize>() {
                Ok(num) if num >= 1 && num <= self.enemy_team.len() => {
                    let mut enemy_guard = self.enemy_team[num - 1].lock().unwrap();
                    if !enemy_guard.alive() {
                        println!("Target is already defeated! Please select a different target.");
                    } else {
                        target = num;
                        break;
                    }
                }
                _ => {
                    println!(
                        "Invalid target! Please select a target between 1 and {}.",
                        self.enemy_team.len()
                    );
                }
            }
        }
    
        let target = Arc::clone(&self.enemy_team[target - 1]);
        let mut target_guard = target.lock().unwrap();
        // Pick a move for the player
        let mov = player.pick_move_guard(&mut target_guard);
        (mov.effect_fn)(&mut *player, &mut *target_guard);
    
        if !player.alive() {
            println!("{} was defeated!", player.name());
        }
        if !target_guard.alive() {
            println!("{} was defeated!", target_guard.name());
        }
    }
    
    fn enemy_turn(&mut self, enemy_arc: &Arc<Mutex<dyn Combatant>>) {
        let mut enemy = enemy_arc.lock().unwrap();
        println!("{}'s turn!", enemy.name());
        println!("{}'s health: {}", enemy.name(), enemy.get_health());
        let target = enemy.pick_target(&self.player_team);
        let mut target_guard = target.lock().unwrap();
        let mov = enemy.pick_move_guard(&mut target_guard);
        (mov.effect_fn)(&mut *enemy, &mut *target_guard);
    }

    fn run_effects(&mut self) {
        for combatant in self.player_team.iter().chain(self.enemy_team.iter()) {
            let mut combatant_guard = combatant.lock().unwrap();
            for effect in combatant_guard.get_effect_list() {
                let effect_fn = {
                    let effect_guard = effect.lock().unwrap();
                    effect_guard.effect_fn.clone()
                };
                (effect_fn)(&mut *combatant_guard);
            }
        }
    }
}
