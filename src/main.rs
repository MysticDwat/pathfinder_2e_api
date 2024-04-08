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

// modules
use checks::check::flat_check;

// classes

// functions

fn main() {
    println!("{:?}", flat_check(10))
}
