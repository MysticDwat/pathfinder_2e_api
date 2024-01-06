// pf2e core rulebook related templates

pub mod ancestries {
    use crate::Size;

    // Ancestry Templates (Name, Speed, Hit Points, Size)
    pub const DWARF:    (&str, u8, u8, Size) = ("Dwarf",    25, 10, Size::Medium);
    pub const ELF:      (&str, u8, u8, Size) = ("Elf",      30, 6,  Size::Medium);
    pub const GNOME:    (&str, u8, u8, Size) = ("Gnome",    25, 8,  Size::Small);
    pub const GOBLIN:   (&str, u8, u8, Size) = ("Goblin",   25, 6,  Size::Small);
    pub const HALFLING: (&str, u8, u8, Size) = ("Halfling", 25, 6,  Size::Small);
    pub const HUMAN:    (&str, u8, u8, Size) = ("Human",    25, 10, Size::Medium);
}

pub mod backgrounds {
    use crate::{ Ability, Skill };
    
    // Background Templates (Name, Ability Boosts, Skill Feat, Skill, Lore Skill)
    pub const FARMHAND: (&str, (Ability, Ability), &str, Skill, &str) 
    = ("Farmhand", (Ability::Consitution, Ability::Wisdom), "Assurance", Skill::Athletics, "Farming");
}