// crates
use dwat20::Die;

// modules
use crate::{
    checks::check_result::SuccessDegree, 
    creatures::creature::Creature,
    modifiers::{
        attribute::Attribute,
        modifier_type:: {
            ArmorType,
            ModifierType,
        },
    }
};

// function to resolve all checks
pub fn check(
    difficulty_class: i16,     // check dc the roll must beat
    modifier: i16,             // modifier to roll
) -> SuccessDegree {
    // get raw roll to check for nat 20 or nat 1
    let raw_roll: i16 = Die::new(20).roll() as i16;

    let roll: i16 = raw_roll + modifier; // add modifier to roll

    let nat_20_check: bool = raw_roll == 20; // nat 20 upgrades check result
    let nat_1_check: bool = raw_roll == 1;   // nat 1 downgrades check result

    let critical_success_check: bool = roll >= difficulty_class + 10;                  // crit success conditions
    let success_check: bool = roll >= difficulty_class;                                // success conditions
    let failure_check: bool = roll < difficulty_class && roll > difficulty_class - 10; // failure conditions
    let critical_failure_check: bool = roll <= difficulty_class - 10;                  // crit failure conditions

    // crit success if
    if critical_success_check            // dc is beaten by 10 or more
    || (nat_20_check && success_check) { // or success and nat 20
        SuccessDegree::CriticalSuccess

    // success if
    } else if success_check                      // dc is beaten or met
    || (nat_20_check && failure_check)           // or failure and nat 20
    || (nat_1_check && critical_success_check) { // or crit success and nat 1
        SuccessDegree::Success

    // failure if
    } else if failure_check                     // below dc by less than 10
    || (nat_20_check && critical_failure_check) // or crit failure and nat 20
    || (nat_1_check && success_check) {         // or success and nat 1
        SuccessDegree::Failure

    // crit failure if below dc by 10 or greater or failure and nat 1
    } else {
        SuccessDegree::CriticalFailure
    }
}

// function to resolve flat checks; a check without any modifiers
pub fn flat_check(difficulty_class: i16) -> SuccessDegree {
    check(difficulty_class, 0,)
}

// function to resolve attacks
pub fn attack(attacker: &Creature, defender: &Creature) -> SuccessDegree {
    check(defender.get_armor_class(), attacker.get_attribute_modifier(&Attribute::Strength))
}