use crate::combatant::Combatant;

pub trait CloneableFn: Fn(&mut dyn Combatant) -> () {
    fn clone_box(&self) -> Box<dyn CloneableFn + Sync + Send>;
}

impl<F> CloneableFn for F
where
    F: Fn(&mut dyn Combatant) -> () + Clone + 'static + Sync + Send,
{
    fn clone_box(&self) -> Box<dyn CloneableFn + Sync + Send> {
        println!("CloneableFn::clone_box");
        Box::new(self.clone())
    }
}

// Implement Clone for Box<dyn CloneableFn> using the clone_box method
impl Clone for Box<dyn CloneableFn + Sync + Send> {
    fn clone(&self) -> Box<dyn CloneableFn + Sync + Send> {
        self.clone_box()
    }
}
