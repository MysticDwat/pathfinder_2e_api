// crates
use std::collections::HashMap;
use strum_macros::Display;

// modules
use crate::{ Ability, Proficiency, AbilityStruct};

// skills in pf2e
#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize, Clone, Display)]
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
    pub skill_modifiers: HashMap<Skill, (Ability, Proficiency)>,
    pub free_skills: u8,
}

impl SkillStruct {
    // function to create new skillstruct
    pub fn new() -> Self {
        let mut skill_modifiers = HashMap::<Skill, (Ability, Proficiency)>::new();

        skill_modifiers.insert(Skill::Acrobatics, (Ability::Dexterity, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Arcana, (Ability::Intelligence, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Athletics, (Ability::Strength, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Crafting, (Ability::Intelligence, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Deception, (Ability::Charisma, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Diplomacy, (Ability::Charisma, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Intimidation, (Ability::Charisma, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Medicine, (Ability::Wisdom, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Nature, (Ability::Wisdom, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Occultism, (Ability::Intelligence, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Performance, (Ability::Charisma, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Religion, (Ability::Wisdom, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Society, (Ability::Intelligence, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Stealth, (Ability::Dexterity, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Survival, (Ability::Wisdom, Proficiency::Untrained));
        skill_modifiers.insert(Skill::Thievery, (Ability::Dexterity, Proficiency::Untrained));

        Self {
            skill_modifiers,
            free_skills: 0,
        }
    }

    // function to get skill mod
    pub fn get_modifier(&self, skill: &Skill, abilities: &AbilityStruct, level: &u8) -> i8 {
        let (ability, proficiency): &(Ability, Proficiency) = self.skill_modifiers.get(skill).unwrap();
        abilities.get(ability) + proficiency.get_modifier(*level as i8)
    }

    pub fn get(&self, skill: &Skill) -> (Ability, Proficiency) {
        *self.skill_modifiers.get(skill).unwrap()
    }

    pub fn get_mut(&mut self, skill: &Skill) -> &mut (Ability, Proficiency) {
        self.skill_modifiers.get_mut(skill).unwrap()
    }

    // MAKE SURE THAT FREE SKILLS WORK PROPERLY IF A SKILL STARTS ABOVE TRAINED.
    pub fn train(&mut self, skill: &Skill, proficiency: &Proficiency) {
        if let Some((_, p)) = self.skill_modifiers.get_mut(skill) {
            if *p < *proficiency {
                *p = *proficiency;
                return;
            }
    
            self.free_skills += 1;
            return;
        }

        self.skill_modifiers.insert(skill.clone(), (Ability::Intelligence, *proficiency));
    }
}