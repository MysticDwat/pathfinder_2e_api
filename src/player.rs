// modules
use crate::{
    Ancestry,
    Background,
    core::{
        ancestries::HUMAN,
        backgrounds::FARMHAND
    },
    Creature
};

// player struct to store player data
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    pub ancestry:      Ancestry,   // stores ancestry data
    pub background:    Background, // stores background data
    pub creature_data: Creature,   // stores creature data
}

// player functions
impl Player {
    // function to create new player
    pub fn new() -> Self {
        

        Self {
            ancestry:      Ancestry::template(&HUMAN),
            background:    Background::template(&FARMHAND),
            creature_data: Creature::new(),
        }
    }
}