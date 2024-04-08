// crates
use std::io;
use strum::IntoEnumIterator;

// modules
use crate::{
    Ability,
    AbilityStruct,
    Ancestry,
    Background,
    Class,
    Creature,
    Proficiency,
    SaveThrow,
    SaveThrowStruct,
    Size,
    Skill, 
    SkillStruct, creature,
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

        let ancestry = Ancestry::template(ancestry_template);
        let background = Background::template(background_template);
        let class = Class::new();

        let mut abilities = AbilityStruct::new();

        Self::generate_abilities(&mut abilities, &background, &class);

        let maximum_hit_points = ancestry.hit_points + class.hit_points + abilities.get(&Ability::Consitution) as u8;
        let speed = ancestry.speed;
        let size = ancestry.size;

        let mut save_throws = SaveThrowStruct::new();
        save_throws.train(&SaveThrow::Fortitude, &class.save_throws.0);
        save_throws.train(&SaveThrow::Reflex,    &class.save_throws.1);
        save_throws.train(&SaveThrow::Will,      &class.save_throws.2);

        let mut skills = SkillStruct::new();
        skills.train(&background.skill, &Proficiency::Trained);
        skills.train(&background.lore_skill, &Proficiency::Trained);
        
        skills.free_skills = class.free_skills;

        for (skill, (_, proficiency)) in class.skills.iter() {
            skills.train(skill, proficiency);
        }

        let creature_data = Creature {
            level: 1,
            name: String::from("Player"),
            size,
            speed,
            abilities,
            maximum_hit_points,
            save_throws,
            skills,
            perception: (Ability::Wisdom, class.perception),
        };

        Self {
            ancestry,
            background,
            creature_data,
            class,
        }
    }

    fn generate_abilities(abilities: &mut AbilityStruct, background: &Background, class: &Class) {
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
            *abilities.get_mut(&boost) += 1;
        }
    }
}