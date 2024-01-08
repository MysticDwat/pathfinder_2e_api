// crates
use std::io;
use strum::IntoEnumIterator;

// modules
use crate::{
    Ability,
    Ancestry,
    Background,
    Class,
    Creature,
    SaveThrow,
    Size,
    Skill, proficiency::{Proficiency, self},
};

// player struct to store player data
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    pub ancestry:      Ancestry,   // stores ancestry data
    pub background:    Background, // stores background data
    pub creature_data: Creature,   // stores creature data
    pub class:         Class,      // stores class data
}

// player functions
impl Player {
    // function to create new player
    pub fn new(
        ancestry_template: &(&str, u8, u8, Size), 
        background_template: &(&str, (Ability, Ability), &str, Skill, &str),
    ) -> Self {

        let mut input_buffer: String = String::new();

        let ancestry = Ancestry::template(ancestry_template);
        let background = Background::template(background_template);

        let class = Class::new();

        let mut creature_data = Creature::new();

        Self::generate_abilities(&mut creature_data, &background, &class);

        creature_data.maximum_hit_points = ancestry.hit_points + class.hit_points + *creature_data.abilities.ability_modifiers.get(&Ability::Consitution).unwrap() as u8;
        creature_data.speed = ancestry.speed;
        creature_data.size = ancestry.size;

        creature_data.saves.save_throw_modifiers.insert(SaveThrow::Fortitude, (Ability::Consitution, class.save_throws.0));
        creature_data.saves.save_throw_modifiers.insert(SaveThrow::Reflex, (Ability::Dexterity, class.save_throws.1));
        creature_data.saves.save_throw_modifiers.insert(SaveThrow::Will, (Ability::Wisdom, class.save_throws.2));

        creature_data.perception.1 = class.perception;

        creature_data.skills.skill_modifiers.insert(background.skill.clone(), (Ability::Strength, Proficiency::Trained));
        creature_data.skills.skill_modifiers.insert(background.lore_skill.clone(), (Ability::Intelligence, Proficiency::Trained));
        
        let mut free_skills = class.free_skills;

        for (skill, (ability, proficiency)) in class.skills.iter() {
            if let Some((_, p)) = creature_data.skills.skill_modifiers.get(skill) {
                if p < proficiency {
                    creature_data.skills.skill_modifiers.insert(skill.clone(), (*ability, *proficiency));
                } else {
                    free_skills += 1;
                }
            } else {
                creature_data.skills.skill_modifiers.insert(skill.clone(), (*ability, *proficiency));
            }
        }

        Self {
            ancestry,
            background,
            creature_data,
            class,
        }
    }

    fn generate_abilities(creature_data: &mut Creature, background: &Background, class: &Class) {
        let abilties = &mut creature_data.abilities.ability_modifiers;
        let mut boosts = Vec::<Ability>::new();

        boosts.push(class.key_ability);
        boosts.push(background.ability_boosts.0);
        boosts.push(Ability::Strength);
        boosts.push(Ability::Strength);
        boosts.push(Ability::Dexterity);
        boosts.push(Ability::Strength);
        boosts.push(Ability::Dexterity);
        boosts.push(Ability::Consitution);
        boosts.push(Ability::Wisdom);

        for boost in boosts {
            if let Some(modifier) = abilties.get_mut(&boost) {
                *modifier += 1;
            }
        }
    }
}