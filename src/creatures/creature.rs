// crates


// modules
use crate::modifiers::modifier::{ Attribute, AttributeType};

// struct to handle creatures
#[derive(Debug)]
pub struct Creature {
    pub level: i16,
    pub abilities: Vec<Attribute>,
}

impl Creature {
    pub fn new() -> Creature{
        Creature {
            level: 1,
            abilities: vec![
                Attribute::Value(AttributeType::Strength, 4), 
                Attribute::Value(AttributeType::Dexterity, 0), 
                Attribute::Value(AttributeType::Constitution, 0), 
                Attribute::Value(AttributeType::Intelligence, 0), 
                Attribute::Value(AttributeType::Wisdom, 0), 
                Attribute::Value(AttributeType::Charisma, 0)
            ]
        }
    }

    pub fn get_attribute_modifier(&self, attribute: &Attribute) -> i16 {
        let attribute_type = match attribute {
            Attribute::Type(attribute_type) => attribute_type,
            Attribute::Value(attribute_type, _) => attribute_type,
        };

        let mut attributes_iter = self.abilities.iter();

        let attribute_value = loop {
            if let Attribute::Value(elem_type, elem_value) = attributes_iter.next().unwrap() {
                if elem_type == attribute_type {
                    break elem_value
                }
            }
        };

        return *attribute_value
    }
}