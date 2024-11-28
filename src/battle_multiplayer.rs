use crate::{
    combatant::Combatant,
    move_mod::{self, Move},
    server::handler::Handler,
    server::message::{Message, MessageKind},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CombatantState {
    pub name: String,
    pub health: f32,
    pub is_alive: bool,
    // Add other necessary fields
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BattleState {
    pub player_team: Vec<CombatantState>,
    pub enemy_team: Vec<CombatantState>,
    pub round: u32,
    // Add other necessary fields
}

pub struct Battle {
    pub player_team: Vec<Arc<Mutex<dyn Combatant>>>,
    pub enemy_team: Vec<Arc<Mutex<dyn Combatant>>>,
    pub order: Vec<Arc<Mutex<dyn Combatant>>>,
    pub round: u32,
    pub is_multiplayer: bool,
    pub server: Option<Arc<Handler>>,
    // Synchronization primitives
    pub action_queue: Arc<(Mutex<Option<Message>>, Condvar)>,
}

impl Battle {
    pub fn new(
        player_team: Vec<Arc<Mutex<dyn Combatant>>>,
        enemy_team: Vec<Arc<Mutex<dyn Combatant>>>,
        is_multiplayer: bool,
    ) -> Self {
        let order = Vec::new();
        let round = 0;
        let server = None;
        let action_queue = Arc::new((Mutex::new(None), Condvar::new()));
        Battle {
            player_team,
            enemy_team,
            order,
            round,
            is_multiplayer,
            server,
            action_queue,
        }
    }

    pub fn get_current_combatant(&self, index: usize) -> Arc<Mutex<dyn Combatant>> {
        Arc::clone(&self.order[index])
    }

    pub fn init_server(&mut self, ipaddr: String, port: u16) {
        let battle_arc = Arc::new(Mutex::new(self));
        let handler = Handler::new(ipaddr, port, battle_arc.clone());
        self.server = Some(handler.clone());

        // Run the server in a separate thread
        let server_clone = handler.clone();
        std::thread::spawn(move || {
            server_clone.run_server();
        });
    }

    pub fn run_battle(&mut self) {
        if self.is_multiplayer {
            self.init_server("127.0.0.1".to_string(), 8080);
        }

        self.calculate_order(); // Initialize the order once
        while self.player_team.iter().any(|x| x.lock().unwrap().alive())
            && self.enemy_team.iter().any(|x| x.lock().unwrap().alive())
        {
            println!(
                "===================================Round {}!===================================",
                self.round
            );

            for i in 0..self.order.len() {
                let current_combatant = self.get_current_combatant(i);
                if !current_combatant.lock().unwrap().alive() {
                    continue; // Skip defeated combatants
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
        self.order.sort_by(|a, b| {
            b.lock()
                .unwrap()
                .speed()
                .partial_cmp(&a.lock().unwrap().speed())
                .unwrap()
        });
    }

    fn player_turn(&mut self, player_arc: &Arc<Mutex<dyn Combatant>>) {
        let mut player = player_arc.lock().unwrap();
        println!("{}'s turn!", player.name());
        println!("{}'s health: {}", player.name(), player.get_health());

        if self.is_multiplayer {
            // Wait for the action from the client
            let (lock, cvar) = &*self.action_queue;
            let mut action = lock.lock().unwrap();
            while action.is_none() {
                // Wait for the action to be available
                action = cvar.wait(action).unwrap();
            }
            // Now action is Some(Message)
            let msg = action.take().unwrap();
            drop(action); // Release the lock

            // Process the action
            match &msg.kind {
                MessageKind::Action { name, user, target } => {
                    // Find the target combatant
                    let target_arc = self.match_combatant(target).unwrap();
                    let mut target_guard = target_arc.lock().unwrap();

                    // Find the move
                    let mov = self.match_move(name);

                    // Process the move
                    (mov.effect_fn)(&mut *player, &mut *target_guard);

                    if !player.alive() {
                        println!("{} was defeated!", player.name());
                    }
                    if !target_guard.alive() {
                        println!("{} was defeated!", target_guard.name());
                    }

                    // After processing, send the updated battle state to clients
                    let battle_state = self.get_state();
                    let update_msg = Message::new_battle_update(&battle_state);

                    // Broadcast the updated battle state to all clients
                    if let Some(ref server) = self.server {
                        if let Err(e) = server.broadcast_message(&update_msg) {
                            println!("Failed to broadcast message: {}", e);
                        }
                    }
                }
                _ => {
                    println!("Invalid message kind received in player_turn");
                }
            }
        } else {
            // Single-player mode: read input from stdin
            // Existing code to select target and move
            // ...
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

        if !enemy.alive() {
            println!("{} was defeated!", enemy.name());
        }
        if !target_guard.alive() {
            println!("{} was defeated!", target_guard.name());
        }

        // After processing, send the updated battle state to clients
        let battle_state = self.get_state();
        let update_msg = Message::new_battle_update(&battle_state);

        // Broadcast the updated battle state to all clients
        if let Some(ref server) = self.server {
            if let Err(e) = server.broadcast_message(&update_msg) {
                println!("Failed to broadcast message: {}", e);
            }
        }
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

    fn match_combatant(&self, name: &str) -> Option<Arc<Mutex<dyn Combatant>>> {
        for combatant in self
            .player_team
            .iter()
            .chain(self.enemy_team.iter())
        {
            let combatant_guard = combatant.lock().unwrap();
            if combatant_guard.name() == name {
                return Some(Arc::clone(combatant));
            }
        }
        None
    }

    fn match_move(&self, name: &str) -> Arc<Move> {
        move_mod::MOVE_DICTIONARY.get(name).unwrap().clone()
    }

    pub fn get_state(&self) -> BattleState {
        BattleState {
            player_team: self
                .player_team
                .iter()
                .map(|c| {
                    let mut combatant = c.lock().unwrap();
                    CombatantState {
                        name: combatant.name().to_string(),
                        health: combatant.get_health(),
                        is_alive: combatant.alive(),
                        // Add other necessary fields
                    }
                })
                .collect(),
            enemy_team: self
                .enemy_team
                .iter()
                .map(|c| {
                    let mut combatant = c.lock().unwrap();
                    CombatantState {
                        name: combatant.name().to_string(),
                        health: combatant.get_health(),
                        is_alive: combatant.alive(),
                        // Add other necessary fields
                    }
                })
                .collect(),
            round: self.round,
            // Add other necessary fields
        }
    }

    pub fn process_server_move(
        &mut self,
        player_move: Arc<Move>,
        user: Arc<Mutex<dyn Combatant>>,
        target: Arc<Mutex<dyn Combatant>>,
    ) {
        let mut user_guard = user.lock().unwrap();
        let mut target_guard = target.lock().unwrap();
        (player_move.effect_fn)(&mut *user_guard, &mut *target_guard);

        // After processing, send the updated battle state to clients
        let battle_state = self.get_state();
        let update_msg = Message::new_battle_update(&battle_state);

        // Broadcast the updated battle state to all clients
        if let Some(ref server) = self.server {
            if let Err(e) = server.broadcast_message(&update_msg) {
                println!("Failed to broadcast message: {}", e);
            }
        }
    }
}
