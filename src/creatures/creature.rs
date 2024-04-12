// crates
use std::collections::HashMap;

// modules
use crate::modifiers::attribute::Attribute;
use crate::modifiers::{
    modifier::Modifier,
    modifier_type::ModifierType,
};

// struct to handle creatures
#[derive(Debug)]
pub struct Creature {
    pub level: i16,
    pub attributes: HashMap<Attribute, i16>,
    pub proficiencies: HashMap<ModifierType, Modifier>,
}

impl Creature {
    pub fn new() -> Creature{
        Creature {
            level: 1,
            attributes: HashMap::from([
                (Attribute::Strength, 0), 
                (Attribute::Dexterity, 0), 
                (Attribute::Constitution, 0), 
                (Attribute::Intelligence, 0), 
                (Attribute::Wisdom, 0), 
                (Attribute::Charisma, 0)
            ]),
            proficiencies: HashMap::new(),
        }
    }

    // function to get attribute modifier
    pub fn get_attribute_modifier(&self, attribute: &Attribute) -> i16 {
        return *&self.attributes[attribute]
    }

    // function to get creature's dc for a check
    pub fn get_difficulty_class(&self, modifier: &ModifierType) -> i16 {
        return self.proficiencies[modifier].get_difficulty_class(self)
    }
}