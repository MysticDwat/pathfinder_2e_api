//import crates
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate dwat20;

//crates
use dwat20::Die;
use rmps::{ Deserializer, Serializer };
use serde::{ Deserialize, Serialize };

//import modules
mod keyword;
mod storage;
mod creature;
mod ability;
mod proficiency;
mod ancestry;
mod background;
mod core;
mod skill;
mod save_throw;
mod player;

//modules
use ability::{ Ability, AbilityStruct };
use ancestry::Ancestry;
use background::Background;
use creature::{ Creature, Size };
use keyword::Keyword;
use player::Player;
use proficiency::Proficiency;
use save_throw::{ SaveThrow, SaveThrowStruct };
use skill::{ Skill, SkillStruct };
use storage::{ file_to_buf, serialize_to_file};

fn main() {
    let player: Player = Player::new();
    println!("{:#?}", player);
}
