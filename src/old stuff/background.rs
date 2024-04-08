//modules
use crate:: { Skill, Ability };

// background struct to store background data
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Background {
    pub name:           String,             // name of background
    pub ability_boosts: (Ability, Ability), // ability boosts granted by background
    pub skill_feat:     String,             // skill feat granted by background
    pub skill:          Skill,              // trained prereq skill granted by background
    pub lore_skill:     Skill,              // related lore skill granted by background
}

// background functions
impl Background {
    // function to create background struct from template
    pub fn template(template: &(&str, (Ability, Ability), &str, Skill, &str)) -> Self {
        Self {
            name:           template.0.to_string(),
            ability_boosts: template.1,
            skill_feat:     template.2.to_string(),
            skill:          template.3.clone(),
            lore_skill:     Skill::Lore(template.4.to_string()),
        }
    }
}