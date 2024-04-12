// crates
use std::collections::HashMap;

// modules
use crate::{
    equipment::armor::Armor,
    modifiers::{
        attribute::Attribute,
        modifier::Modifier,
        modifier_type::{
            ModifierType,
            ArmorType,
        },
    },
};

// struct to handle creatures
#[derive(Debug)]
pub struct Creature {
    pub level: i16,
    pub attributes: HashMap<Attribute, i16>,
    pub proficiencies: HashMap<ModifierType, Modifier>,
    pub hit_points: i16,
    pub armor: Armor,
}

impl Creature {
    pub fn new() -> Creature{
        Creature {
            level: 1,
            attributes: HashMap::from([
                (Attribute::Strength, 5), 
                (Attribute::Dexterity, 0), 
                (Attribute::Constitution, 0), 
                (Attribute::Intelligence, 0), 
                (Attribute::Wisdom, 0), 
                (Attribute::Charisma, 0)
            ]),
            proficiencies: HashMap::from([]),
            hit_points: 10,
            armor: Armor::new()
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

    // function to handle damaging a creature
    pub fn damage(&mut self) {
        self.hit_points -= 10;
    }

    // function to handle healing a creature
    pub fn heal(&mut self) {
        self.hit_points += 10;
    }

    // funciton to calculate AC
    pub fn get_armor_class(&self) -> i16 {
        self.get_difficulty_class(&ModifierType::ArmorClass(self.armor.armor_type)) + self.armor.armor_class
    }
}