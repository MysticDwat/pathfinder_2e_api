// enum to handle proficiency ranks
#[derive(Debug)]
pub enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary
}

impl Proficiency {
    // function to get proficiency bonus
    pub fn get_bonus (&self, level: i16) -> i16{
        match self {
            Proficiency::Untrained => 0,
            Proficiency::Trained   => 2 + level,
            Proficiency::Expert    => 4 + level,
            Proficiency::Master    => 6 + level,
            Proficiency::Legendary => 8 + level
        }
    }
}