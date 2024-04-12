// enum to store type of modifier
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ModifierType {
    ArmorClass(Armor),
    Attack(Attack),
    ClassDC(Class),
    Perception,
    SaveThrow(SaveThrow),
    Skill(Skill),
    SpellAttack(Tradition),
    SpellDC(Tradition),
}

// enum to store armor types
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Armor {
    Unarmored,
    Light,
    Medium,
    Heavy,
}

// enum to store attack types
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Attack {
    Melee,
    Ranged,
}

// enum to store Class 
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Class {
    Bard,
    Cleric,
    Druid,
    Fighter,
    Ranger,
    Rogue,
    Witch,
    Wizard,
}

// enum to store save throw types
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SaveThrow {
    Fortitude,
    Reflex,
    Will
}

// enum to store skill types
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Theivery,
}

// enum to store magic traditions
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tradition {
    Arcane,
    Divine,
    Occult,
    Primal,
}