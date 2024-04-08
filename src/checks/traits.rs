// trait for all modifiers, like attributes, skills, saves, etc
pub trait Modifier {
    // function to get dc of modifier
    pub fn get_difficulty_class(
        attribute_modifier: i16,   // attribute modifier
        proficiency_bonus: i16,    // proficiency bonus
        circumstance_bonus: i16,   // highest circumstance bonus
        circumstance_penalty: i16, // highest circumstance penalty 
        status_bonus: i16,         // highest status bonus
        status_penalty: i16,       // highest status penalty
        item_bonus: i16,           // highest item bonus
        item_penalty: i16,         // highest item_penalty
    ) -> i16 {
        // add up modifiers and add 10 for dc
        attribute_modifier + proficiency_bonus + circumstance_bonus + circumstance_penalty 
        + status_bonus + status_penalty + item_bonus + item_penalty + 10
    }
}