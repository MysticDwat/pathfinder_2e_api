// proficiency enum
#[derive(Debug, PartialEq, PartialOrd, Deserialize, Serialize, Clone, Copy)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary
}

// proficiency functions
impl Proficiency {
    // get proficiency mod
    pub fn get_modifier (&self, level: i8) -> i8{
        match self {
            Proficiency::Untrained => 0,
            Proficiency::Trained   => 2 + level,
            Proficiency::Expert    => 4 + level,
            Proficiency::Master    => 6 + level,
            Proficiency::Legendary => 8 + level
        }
    }
}