// crates
use std::collections::HashMap;

// modules
use crate::{ Ability, Proficiency, AbilityStruct};

// saving throws in pf2e
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SaveThrow {
    Fortitude,
    Reflex,
    Will
}

// struct to store save throws
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SaveThrowStruct {
    pub save_throw_modifiers: HashMap<SaveThrow, (Ability, Proficiency)>,
}

impl SaveThrowStruct {
    pub fn new() -> Self {
        let mut save_throw_modifiers = HashMap::<SaveThrow, (Ability, Proficiency)>::new();

        save_throw_modifiers.insert(SaveThrow::Fortitude, (Ability::Consitution, Proficiency::Untrained));
        save_throw_modifiers.insert(SaveThrow::Reflex, (Ability::Dexterity, Proficiency::Untrained));
        save_throw_modifiers.insert(SaveThrow::Will, (Ability::Wisdom, Proficiency::Untrained));

        Self {
            save_throw_modifiers
        }
    }

    pub fn get_modifier(&self, save_throw: &SaveThrow, abilities: &AbilityStruct, level: &u8) -> i8 {
        let (ability, proficiency): &(Ability, Proficiency) = self.save_throw_modifiers.get(save_throw).unwrap();
        abilities.get(ability) + proficiency.get_modifier(*level as i8)
    }

    pub fn get(&self, save_throw: &SaveThrow) -> (Ability, Proficiency) {
        *self.save_throw_modifiers.get(save_throw).unwrap()
    }

    pub fn get_mut(&mut self, save_throw: &SaveThrow) -> &mut (Ability, Proficiency) {
        self.save_throw_modifiers.get_mut(save_throw).unwrap()
    }

    pub fn train(&mut self, save_throw: &SaveThrow, proficiency: &Proficiency) {
        let (_, p) = self.get_mut(save_throw);

        if *p < *proficiency {
            *p = *proficiency;
        }
    }
}
