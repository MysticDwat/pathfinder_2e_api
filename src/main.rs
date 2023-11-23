//crates
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

//modules
mod keyword;
mod storage;

//use crate and module
use keyword::Keyword;
use storage::serialize;
use serde::Deserialize;
use rmps::Deserializer;

fn main() {
    let mut buf = Vec::new();
    let val = Keyword::new(0, "test".into(), "test keyword for msgpack".into());
    serialize(&val, &mut buf);
    println!("{:?}",buf);
    let de_val = deserialize!(Keyword, buf);
    println!("{:?}",de_val);
    println!("{:?}",val);
}
