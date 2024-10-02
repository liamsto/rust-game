// src/battle.rs

use crate::combatant::Combatant;

pub struct Battle<'a> {
    pub player: &'a mut dyn Combatant,
    pub enemy: &'a mut dyn Combatant,
    pub round: u32,
}

impl<'a> Battle<'a> {

    pub fn run_battle(&mut self) {
        loop {
            println!("========================================== Round {} ==========================================", self.round);
            self.round += 1;
            let player_health = *self.player.health();
            let enemy_health = *self.enemy.health();
            println!("{}'s health: {}", self.player.name(), player_health);
            println!("{}'s health: {}", self.enemy.name(), enemy_health);
            //check who has the higher speed
            let original_player_speed = *self.player.speed();
            if self.player.speed() > self.enemy.speed() {
                self.player_turn();
                if !self.enemy.alive() {
                    println!("{} fainted!", self.enemy.name());
                    break;
                }

                let player_speed = *self.player.speed();
                let speed_changed = player_speed > original_player_speed;
                //if the player's speed is double the enemy's speed, the player will go again
                //If the player used a move that changed their speed, they should not go again until the next round
                if *self.player.speed() > *self.enemy.speed() * 2.0 && !speed_changed {
                    self.player_turn();
                    if !self.enemy.alive() {
                        println!("{} fainted!", self.enemy.name());
                        break;
                    }
                }
                self.enemy_turn();
                if !self.player.alive() {
                    println!("{} fainted!", self.player.name());
                    break;
                }
                
                if self.player.get_effect_list().len() > 0 || self.enemy.get_effect_list().len() > 0 {
                    self.run_effects();                        
                    if !self.player.alive() {
                        println!("{} fainted!", self.player.name());
                        break;
                    }
                    if !self.enemy.alive() {
                        println!("{} fainted!", self.enemy.name());
                        break;
                    }
                }
                
            } else {
                let original_enemy_speed = *self.enemy.speed();
                self.enemy_turn();
                if !self.player.alive() {
                    println!("{} fainted!", self.player.name());
                    break;
                }
                let enemy_speed = *self.enemy.speed();
                let speed_changed = enemy_speed > original_enemy_speed;

                //if the enemy's speed is double the player's speed, the enemy will go again
                if *self.enemy.speed() > *self.player.speed() * 2.0 && !speed_changed {
                    self.enemy_turn();
                    if !self.player.alive() {
                        println!("{} fainted!", self.player.name());
                        break;
                    }
                }

                self.player_turn();
                if !self.enemy.alive() {
                    println!("{} fainted!", self.enemy.name());
                    break;
                }

                if self.player.get_effect_list().len() > 0 || self.enemy.get_effect_list().len() > 0 {
                    self.run_effects();                        
                    if !self.player.alive() {
                        println!("{} fainted!", self.player.name());
                        break;
                    }
                    if !self.enemy.alive() {
                        println!("{} fainted!", self.enemy.name());
                        break;
                    }
                }
            }
            
        }
    }

    fn player_turn(&mut self) {
        println!("{}'s turn!", self.player.name());
        let mov = self.player.pick_move(self.enemy);
        (mov.effect_fn)(self.player, self.enemy);

    }

    fn enemy_turn(&mut self) {
        println!("{}'s turn!", self.enemy.name());
        let mov = self.enemy.pick_move(self.player);
        (mov.effect_fn)(self.enemy, self.player);
    }

    fn run_effects(&mut self) {
        if self.player.get_effect_list().len() != 0 {
            for effect in self.player.get_effect_list() {
                let effect = effect.lock().unwrap();
                (effect.effect_fn)(self.player);
            }
        }

        if self.enemy.get_effect_list().len() != 0 {
            for effect in self.enemy.get_effect_list() {
                let effect = effect.lock().unwrap();
                (effect.effect_fn)(self.enemy);
            }
        }
    }

}
