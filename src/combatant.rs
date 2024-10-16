use std::sync::{Arc, Mutex, MutexGuard};

use crate::effect::Effect;
use crate::move_mod::Move;

impl<'a> AsMut<dyn Combatant + 'a> for dyn Combatant {
    fn as_mut(&mut self) -> &mut (dyn Combatant + 'a) {
        self
    }
}

pub trait Combatant {
    fn health(&mut self) -> &mut f32;
    fn get_health(&self) -> f32;
    fn attack(&mut self) -> &mut f32;
    fn defense(&mut self) -> &mut f32;
    fn speed(&mut self) -> &mut f32;
    fn get_speed(&self) -> f32;
    fn max_health(&self) -> f32;
    fn max_attack(&self) -> f32;
    fn max_defense(&self) -> f32;
    fn max_speed(&self) -> f32;
    fn set_health(&mut self, health: f32) {
        *self.health() = health;
    }
    fn set_attack(&mut self, attack: f32) {
        *self.attack() = attack;
    }
    fn set_defense(&mut self, defense: f32) {
        *self.defense() = defense;
    }
    fn set_speed(&mut self, speed: f32) {
        *self.speed() = speed;
    }
    fn set_alive(&mut self, alive: bool);

    fn name(&self) -> &str;
    fn apply_effect(&mut self, effect: Mutex<Effect>);
    fn clear_effect(&mut self, effect: Mutex<Effect>);
    fn has_effect(&self, effect: &Mutex<Effect>) -> bool;
    fn check_death(&mut self) {
        if *self.health() <= 0.0 {
            println!("{} was defeated!", self.name());
            self.set_alive(false);
        }
    }
    fn alive(&mut self) -> bool {
        *self.health() > 0.0
    }
    fn known_moves(&self) -> Vec<Arc<Move>>;

    fn pick_target(
        &self,
        enemy_team: &Vec<Arc<Mutex<(dyn Combatant + 'static)>>>,
    ) -> Arc<Mutex<dyn Combatant>> {
        //TODO: Implement a better target selection algorithm
        for enemy in enemy_team {
            let mut enemy_guard = enemy.lock().unwrap();
            if enemy_guard.alive() {
                return Arc::clone(enemy);
            }
        }
        panic!("No valid target found! This should never happen!");
    }

    fn pick_move<'a>(&self, enemy: &'a mut (dyn Combatant + 'a)) -> Arc<Move>;

    fn get_effect_list(&self) -> Vec<Arc<Mutex<Effect>>>;
    fn get_crit_chance(&self) -> f32;
    #[inline]
    fn crit(&self, damage: f32) -> f32 {
        let crit_chance = self.get_crit_chance();
        let random = rand::random::<f32>();
        if random < crit_chance {
            println!("Critical hit!");
            return damage * 1.5;
        }
        return damage;
    }

    fn moves(&self) -> [Option<Arc<Move>>; 4];

    fn is_player_controlled(&self) -> bool;

    fn pick_move_guard(&self, guard: &mut MutexGuard<'_, (dyn Combatant + 'static)>) -> Arc<Move> {
        self.pick_move(&mut **guard)
    }
}

pub trait CloneableCombatant: Combatant {
    fn clone_box(&self) -> Box<dyn CloneableCombatant + Sync + Send>;
}

impl Clone for Box<dyn CloneableCombatant + Sync> {
    fn clone(&self) -> Box<dyn CloneableCombatant + Sync> {
        self.clone_box()
    }
}

impl PartialEq for dyn Combatant {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
