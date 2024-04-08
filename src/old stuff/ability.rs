// crates
use strum_macros::EnumIter;
use std::fmt::{ Display, Formatter, Result };
use std::collections::HashMap;

// abilities in pf2e
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, EnumIter, Clone, Copy)]
pub enum Ability {
    Strength,
    Dexterity,
    Consitution,
    Intelligence,
    Wisdom,
    Charisma
}

// implement display to display ability name
impl Display for Ability {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Ability::Strength     => write!(f, "Strength",),
            Ability::Dexterity    => write!(f, "Dexterity",),
            Ability::Consitution  => write!(f, "Consitution",),
            Ability::Intelligence => write!(f, "Intelligence",),
            Ability::Wisdom       => write!(f, "Wisdom",),
            Ability::Charisma     => write!(f, "Charisma",)
        }
    }
}

// struct to store abilities
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AbilityStruct {
    pub ability_modifiers: HashMap<Ability, i8>,
}

impl AbilityStruct {
    pub fn new() -> Self {
        let mut ability_modifiers = HashMap::<Ability, i8>::new();

        ability_modifiers.insert(Ability::Strength, 0);
        ability_modifiers.insert(Ability::Dexterity, 0);
        ability_modifiers.insert(Ability::Consitution, 0);
        ability_modifiers.insert(Ability::Intelligence, 0);
        ability_modifiers.insert(Ability::Wisdom, 0);
        ability_modifiers.insert(Ability::Charisma, 0);

        Self {
            ability_modifiers
        }
    }

    pub fn get(&self, ability: &Ability) -> i8 {
        *self.ability_modifiers.get(ability).unwrap()
    }

    pub fn get_mut(&mut self, ability: &Ability) -> &mut i8 {
        self.ability_modifiers.get_mut(ability).unwrap()
    }
}