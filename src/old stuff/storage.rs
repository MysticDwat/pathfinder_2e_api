use std::fs::File;
use std::io::prelude::*;
use crate::{ Serialize, Serializer };

fn serialize<T: Serialize>(val: &T, buf: &mut Vec<u8>) {
    val.serialize(&mut Serializer::new(buf)).unwrap();
}

#[macro_export]
macro_rules! deserialize {
    ($buf: expr, $T: ty) => {
        <$T>::deserialize(&mut Deserializer::new(&$buf[..])).unwrap()
    };
}

pub fn serialize_to_file<T: Serialize>(val: &T, file_path: &str){
    let mut buf: Vec<u8> = Vec::new();
    serialize(val, &mut buf);

    let mut file = File::create(file_path).unwrap();
    let _ = file.write_all(&buf);
}

pub fn file_to_buf(file_path: &str, buf: &mut Vec<u8>) {
    let mut file = File::open(file_path).unwrap();
    let _ = file.read_to_end(buf);
}