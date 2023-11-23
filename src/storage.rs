use super::serde::Serialize;
use super::rmps::Serializer;

pub fn serialize<T: Serialize>(val: &T, buf: &mut Vec<u8>) {
    val.serialize(&mut Serializer::new(buf)).unwrap();
}

#[macro_export]
macro_rules! deserialize {
    ($T: ty, $buf: expr) => {
        <$T>::deserialize(&mut Deserializer::new(&$buf[..])).unwrap()
    };
}