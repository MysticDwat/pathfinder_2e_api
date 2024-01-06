// crates
use std::collections::HashMap;

// modules
use crate::{ Ability, Proficiency};

// skills in pf2e
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone)]
pub enum Skill {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Lore(String), // lore skills store their field
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery
}

// struct to store creature skills
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SkillStruct {
    pub skills: HashMap<Skill, (Ability, Proficiency)>,
}

impl SkillStruct {
    // function to create new skillstruct
    pub fn new() -> Self {
        Self {
            skills: HashMap::<Skill, (Ability, Proficiency)>::new()
        }
    }

    // function to get skill mod
    pub fn get_skill_modifier(&self, skill: &Skill, ability_modifiers: &HashMap<Ability, i8>, level: &u8) -> i8 {
        let (ability, proficiency): &(Ability, Proficiency) = self.skills.get(skill).unwrap();
        ability_modifiers.get(ability).unwrap() + proficiency.get_modifier(*level as i8)
    }
}