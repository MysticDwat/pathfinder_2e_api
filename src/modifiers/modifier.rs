// modules
use crate::creatures::creature::Creature;
use crate::modifiers::{
    proficiency::Proficiency,
    attribute::Attribute,
};

// struct to store modifiers
#[derive(Debug)]
pub struct Modifier {
    pub attribute: Attribute,
    pub proficiency_rank: Proficiency,
    pub circumstance_bonuses: Vec<i16>,
    pub circumstance_penalties: Vec<i16>,
    pub status_bonuses: Vec<i16>,
    pub status_penalties: Vec<i16>,
    pub item_bonuses: Vec<i16>,
    pub item_penalties: Vec<i16>,
}

impl Modifier {
    // get attribute modifier
    pub fn get_attribute_modifier(&self, creature: &Creature) -> i16 {
        creature.get_attribute_modifier(&self.attribute)
    }

    // get largest circumstance bonus
    pub fn get_circumstance_bonus(&self) -> &i16 {
        match self.circumstance_bonuses.iter().max() {
            Some(value) => value,
            None => &0,
        }
    }

    // get largest circumstance penalty
    pub fn get_circumstance_penalty(&self) -> &i16 {
        match self.circumstance_penalties.iter().max() {
            Some(value) => value,
            None => &0,
        }
    }

    // get largest status penalty
    pub fn get_status_bonus(&self) -> &i16 {
        match self.status_bonuses.iter().max() {
            Some(value) => value,
            None => &0,
        }
    }

    // get largest status penalty
    pub fn get_status_penalty(&self) -> &i16 {
        match self.status_penalties.iter().max() {
            Some(value) => value,
            None => &0,
        }
    }

    // get largest item bonus
    pub fn get_item_bonus(&self) -> &i16 {
        match self.item_bonuses.iter().max() {
            Some(value) => value,
            None => &0,
        }
    }

    // get largest item penalty
    pub fn get_item_penalty(&self) -> &i16 {
        match self.item_penalties.iter().max() {
            Some(value) => value,
            None => &0,
        }
    }

    // TODO: replace level param with creature param to access level and attributes
    // adds up all bonuses and penalties to get final mod
    pub fn get_modifier(&self, creature: &Creature) -> i16 {
        self.get_attribute_modifier(creature)
        + self.proficiency_rank.get_bonus(creature.level)
        + self.get_circumstance_bonus()
        + self.get_circumstance_penalty()
        + self.get_status_bonus()
        + self.get_status_penalty()
        + self.get_item_bonus()
        + self.get_item_penalty()
    }

    // get modifier dc by adding 10 to mod
    pub fn get_difficulty_class(&self, creature: &Creature) -> i16 {
        self.get_modifier(creature) + 10
    }
}