use crate::character::Character;

#[derive(Clone)]
pub struct Item {
    pub name: &'static str,
    pub healthinc: f32,
    pub attackinc: f32,
    pub speedinc: f32,
    pub defenseinc: f32,
    pub speeddec: f32,
    pub defensedec: f32,
    pub attackdec: f32,
    pub healthdec: f32,
}

impl Item {
    pub fn clone (&self) -> Item {
        Item {
            name: self.name,
            healthinc: self.healthinc,
            attackinc: self.attackinc,
            speedinc: self.speedinc,
            defenseinc: self.defenseinc,
            speeddec: self.speeddec,
            defensedec: self.defensedec,
            attackdec: self.attackdec,
            healthdec: self.healthdec,
        }
    }

    pub fn equip (&self, character: &mut Character) {
        if character.held_item.as_ref() == None {
            character.held_item = Some(self.clone().into());
            match self.healthinc {
                0.0 => {},
                _ => character.health += self.healthinc,
            }
            match self.attackinc {
                0.0 => {},
                _ => character.attack += self.attackinc,
            }
            match self.speedinc {
                0.0 => {},
                _ => character.speed += self.speedinc,
            }
            match self.defenseinc {
                0.0 => {},
                _ => character.defense += self.defenseinc,
            }
            match self.speeddec {
                0.0 => {},
                _ => character.speed -= self.speeddec,
            }
            match self.defensedec {
                0.0 => {},
                _ => character.defense -= self.defensedec,
            }
            match self.attackdec {
                0.0 => {},
                _ => character.attack -= self.attackdec,
            }
            match self.healthdec {
                0.0 => {},
                _ => character.health -= self.healthdec,
            }
            return;
        }
        if character.held_item.as_ref().map(|item| item.as_ref()) == Some(self) {
            println!("Item already equipped");
            return;
        }
        else {
            let old_item = character.held_item.as_ref().unwrap().clone();
            old_item.unequip(character);
            character.held_item = Some(self.clone().into());
            match self.healthinc {
                0.0 => {},
                _ => character.health += self.healthinc,
            }
            match self.attackinc {
                0.0 => {},
                _ => character.attack += self.attackinc,
            }
            match self.speedinc {
                0.0 => {},
                _ => character.speed += self.speedinc,
            }
            match self.defenseinc {
                0.0 => {},
                _ => character.defense += self.defenseinc,
            }
            match self.speeddec {
                0.0 => {},
                _ => character.speed -= self.speeddec,
            }
            match self.defensedec {
                0.0 => {},
                _ => character.defense -= self.defensedec,
            }
            match self.attackdec {
                0.0 => {},
                _ => character.attack -= self.attackdec,
            }
            match self.healthdec {
                0.0 => {},
                _ => character.health -= self.healthdec,
            }
        }

    } 

    pub fn unequip (&self, character: &mut Character) {
        if character.held_item.as_ref().map(|item| item.as_ref()) == Some(self) {
            character.held_item = None;
            match self.healthinc {
                0.0 => {},
                _ => character.health -= self.healthinc,
            }
            match self.attackinc {
                0.0 => {},
                _ => character.attack -= self.attackinc,
            }
            match self.speedinc {
                0.0 => {},
                _ => character.speed -= self.speedinc,
            }
            match self.defenseinc {
                0.0 => {},
                _ => character.defense -= self.defenseinc,
            }
            match self.speeddec {
                0.0 => {},
                _ => character.speed += self.speeddec,
            }
            match self.defensedec {
                0.0 => {},
                _ => character.defense += self.defensedec,
            }
            match self.attackdec {
                0.0 => {},
                _ => character.attack += self.attackdec,
            }
            match self.healthdec {
                0.0 => {},
                _ => character.health += self.healthdec,
            }
            return;
        }
        else {
            println!("No item equipped");
        }
    }

}



impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}