// modules

// enum to handle attribute types
#[derive(Debug)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma
}

// enum to handle proficiency ranks
#[derive(Debug)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary
}

impl Proficiency {
    // function to get proficiency bonus
    pub fn get_bonus (&self, level: i16) -> i16{
        match self {
            Proficiency::Untrained => 0,
            Proficiency::Trained   => 2 + level,
            Proficiency::Expert    => 4 + level,
            Proficiency::Master    => 6 + level,
            Proficiency::Legendary => 8 + level
        }
    }
}

// struct to store modifiers
#[derive(Debug)]
pub struct Modifier {
    pub attribute_type: Attribute,
    pub attribute_modifier: i16,
    pub proficiency_rank: Proficiency,
    pub circumstance_bonuses: Vec<i16>,
    pub circumstance_penalties: Vec<i16>,
    pub status_bonuses: Vec<i16>,
    pub status_penalties: Vec<i16>,
    pub item_bonuses: Vec<i16>,
    pub item_penalties: Vec<i16>,
}

impl Modifier {
    // get largest circumstance bonus
    pub fn get_circumstance_bonus(&self) -> &i16 {
        self.circumstance_bonuses.iter().max().unwrap()
    }

    // get largest circumstance penalty
    pub fn get_circumstance_penalty(&self) -> &i16 {
        self.circumstance_penalties.iter().max().unwrap()
    }

    // get largest status penalty
    pub fn get_status_bonus(&self) -> &i16 {
        self.status_bonuses.iter().max().unwrap()
    }

    // get largest status penalty
    pub fn get_status_penalty(&self) -> &i16 {
        self.status_penalties.iter().max().unwrap()
    }

    // get largest item bonus
    pub fn get_item_bonus(&self) -> &i16 {
        self.item_bonuses.iter().max().unwrap()
    }

    // get largest item penalty
    pub fn get_item_penalty(&self) -> &i16 {
        self.item_penalties.iter().max().unwrap()
    }

    // adds up all bonuses and penalties to get final mod
    pub fn get_modifier(&self, level: i16) -> i16 {
        self.attribute_modifier 
        + self.proficiency_rank.get_bonus(level)
        + self.get_circumstance_bonus()
        + self.get_circumstance_penalty()
        + self.get_status_bonus()
        + self.get_status_penalty()
        + self.get_item_bonus()
        + self.get_item_penalty()
    }

    // get modifier dc by adding 10 to mod
    pub fn get_difficulty_class(&self, level: i16) -> i16 {
        self.get_modifier(level) + 10
    }
}