use crate::combatant::Combatant;

pub trait CloneableFnCombatant: Fn(&mut dyn Combatant, &mut dyn Combatant) + Send + Sync {
    fn clone_box(&self) -> Box<dyn CloneableFnCombatant + Send + Sync>;
}

impl<F> CloneableFnCombatant for F
where
    F: Fn(&mut dyn Combatant, &mut dyn Combatant) -> () + Clone + Send + 'static + Sync,
{
    fn clone_box(&self) -> Box<dyn CloneableFnCombatant + Send + Sync> {
        println!("CloneableFnCombatant::clone_box");
        Box::new(self.clone())
    }
}

// Implement Clone for Box<dyn CloneableFn> using the clone_box method
impl Clone for Box<dyn CloneableFnCombatant + Send + Sync> {
    fn clone(&self) -> Box<dyn CloneableFnCombatant + Send + Sync> {
        self.clone_box()
    }
}
