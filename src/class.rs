// crates
use std::collections::HashMap;

// modules
use crate::{ Ability, Proficiency, Skill };

// class struct to store class data
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Class {
    pub key_ability: Ability,
    pub hit_points: u8,
    pub class_dc: Proficiency,
    pub save_throws: (Proficiency, Proficiency, Proficiency),
    pub perception: Proficiency,
    pub skills: HashMap<Skill, (Ability, Proficiency)>,
    pub free_skills: u8,
    pub attacks: HashMap<String, Proficiency>,
    pub defenses: HashMap<String, Proficiency>,
}

impl Class {
    pub fn new() -> Self {
        let mut skills = HashMap::<Skill, (Ability, Proficiency)>::new();
        skills.insert(Skill::Athletics, (Ability::Strength, Proficiency::Trained));
        skills.insert(Skill::Acrobatics, (Ability::Dexterity, Proficiency::Trained));

        Self {
            key_ability: Ability::Strength,
            hit_points: 8,
            class_dc: Proficiency::Trained,
            save_throws: (Proficiency::Trained, Proficiency::Trained, Proficiency::Trained),
            perception: Proficiency::Trained,
            skills,
            free_skills: 3,
            attacks: HashMap::<String, Proficiency>::new(),
            defenses: HashMap::<String, Proficiency>::new(),
        }
    }
}