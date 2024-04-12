// modules
use crate:: modifiers::modifier_type::ArmorType;

// struct to store armor data
#[derive(Debug)]
pub struct Armor {
    pub armor_class: i16,
    pub armor_type: ArmorType,
}

impl Armor {
    pub fn new() -> Armor {
        Armor {
            armor_class: 1,
            armor_type: ArmorType::Light,
        }
    }
}