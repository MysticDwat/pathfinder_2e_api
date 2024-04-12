// import crates
extern crate serde;
//#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate dwat20;

// crates
//use dwat20::Die;
//use rmps::{ Deserializer, Serializer };
//use serde::{ Deserialize, Serialize };

// import modules
mod checks;
mod modifiers;
mod creatures;

// modules
use modifiers::{
    attribute::Attribute,
    modifier::Modifier, 
    proficiency::Proficiency,
};
use creatures::creature::Creature;

// classes

// functions

fn main() {
    let creature = Creature::new();
    let modifier: Modifier = Modifier {attribute: Attribute::Strength, proficiency_rank: Proficiency::Untrained, circumstance_bonuses: Vec::<i16>::new(), circumstance_penalties: Vec::<i16>::new(), status_bonuses: Vec::<i16>::new(), status_penalties: Vec::<i16>::new(), item_bonuses: Vec::<i16>::new(), item_penalties: Vec::<i16>::new() };
    println!("{:?}", modifier.get_modifier(&creature))
}
