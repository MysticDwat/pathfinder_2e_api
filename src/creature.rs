// modules
use crate::{ SkillStruct, AbilityStruct, SaveThrowStruct };

// creatures sizes in pf2e
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, Copy)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan
}

// player struct to store player data
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Creature {
    pub level:              u8,               // stores creature level
    pub name:               String,           // stores name
    pub size:               Size,             // stores size
    pub speed:              u8,               // stores speed
    pub abilities:          AbilityStruct,    // stores abilitiy mods
    pub maximum_hit_points: u8,               // stores max hp
    pub saves:              SaveThrowStruct,  // stores save proficiency and ability
    pub skills:             SkillStruct,      // stores skills
}

// player functions
impl Creature {
    // function to create new creature
    pub fn new() -> Self {
        Self {
            level: 1,
            name: "Creature".to_string(),
            size: Size::Medium,
            speed: 25,
            abilities: AbilityStruct::new(),
            maximum_hit_points: 8,
            saves: SaveThrowStruct::new(),
            skills: SkillStruct::new(),
        }
    }
}