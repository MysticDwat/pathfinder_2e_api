// crates
use std::collections::HashMap;

// modules
use crate::{ Ability, Proficiency};

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
        Self {
            save_throw_modifiers: HashMap::<SaveThrow, (Ability, Proficiency)>::new()
        }
    }
}
