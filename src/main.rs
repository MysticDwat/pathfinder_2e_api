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
mod equipment;

// modules
use checks::check::attack;
use modifiers::{
    attribute::Attribute,
    modifier::Modifier, 
    proficiency::Proficiency,
    modifier_type::{
        ModifierType,
        ArmorType,
    },
};
use creatures::creature::Creature;

// classes

// functions

fn main() {
    let ac_modifier: Modifier = Modifier {attribute: Attribute::Dexterity, proficiency_rank: Proficiency::Trained, circumstance_bonuses: Vec::<i16>::new(), circumstance_penalties: Vec::<i16>::new(), status_bonuses: Vec::<i16>::new(), status_penalties: Vec::<i16>::new(), item_bonuses: Vec::<i16>::new(), item_penalties: Vec::<i16>::new() };
    let mut defender = Creature::new();
    defender.proficiencies.insert(ModifierType::ArmorClass(ArmorType::Light), ac_modifier);
    println!("{:?}", attack(&defender, &defender))
}
