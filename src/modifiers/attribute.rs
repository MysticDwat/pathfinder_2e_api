// enum to handle attribute types
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}