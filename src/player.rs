// crates
use std::io;
use strum::IntoEnumIterator;

// modules
use crate::{
    Ancestry,
    Background,
    core::{
        ancestries::{HUMAN, DWARF, ELF, GNOME, GOBLIN, HALFLING},
        backgrounds::FARMHAND
    },
    Creature, ability::{Ability, self}, background
};

// player struct to store player data
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Player {
    pub ancestry:      Ancestry,   // stores ancestry data
    pub background:    Background, // stores background data
    pub creature_data: Creature,   // stores creature data
}

// player functions
impl Player {
    // function to create new player
    pub fn new() -> Self {
        let mut input_buffer: String = String::new();

        println!("Please Enter Ancestry Number:\n\
                1. Dwarf\n\
                2. Elf\n\
                3. Gnome\n\
                4. Goblin\n\
                5. Halfling\n\
                6. Human\
        ");

        let ancestry_template = loop {
            input_buffer.clear();
            io::stdin().read_line(&mut input_buffer).expect("failed to readline");

            if let Ok(input_number) = input_buffer.trim().parse::<u8>() {
                match input_number {
                    1 => break &DWARF,
                    2 => break &ELF,
                    3 => break &GNOME,
                    4 => break &GOBLIN,
                    5 => break &HALFLING,
                    6 => break &HUMAN,
                    _ => ()
                };
            }

            println!("Invalid input, please try again.");
        };

        println!("Please Enter Background Number:\n\
                1. Farmhand\
        ");

        let background = Background::template(loop {
            input_buffer.clear();
            io::stdin().read_line(&mut input_buffer).expect("failed to readline");

            if let Ok(input_number) = input_buffer.trim().parse::<u8>() {
                match input_number {
                    1 => break &FARMHAND,
                    _ => ()
                };
            }

            println!("Invalid input, please try again.");
        });

        let mut creature_data = Creature::new();
        let mut _abilties = &mut creature_data.abilities.ability_modifiers;

        let mut boosts = Vec::<Ability>::new();
        let mut free_boosts: Vec<Ability> = Ability::iter().collect();
        let mut ancestry_boosts = free_boosts.clone();

        println!("Please select two boosts:");
        
        while boosts.len() < 3 {
            for (i, ability) in ancestry_boosts.iter().enumerate() {
                println!("{:}. {:}", i + 1, ability);
            }

            let boost = loop {
                input_buffer.clear();
                io::stdin().read_line(&mut input_buffer).expect("failed to readline");
    
                if let Ok(input_number) = input_buffer.trim().parse::<usize>() {
                    let ability = ancestry_boosts.get(input_number - 1);
                    match ability {
                        Some(ability) => break *ability, 
                        _ => ()
                    };
                }
    
                println!("Invalid input, please try again.");
            };

            if let Some(pos) = ancestry_boosts.iter().position(|x| x == &boost) {
                ancestry_boosts.remove(pos);
            }

            boosts.push(boost);
        };

        println!("Please select four boosts:");
        
        while boosts.len() < 7 {
            for (i, ability) in free_boosts.iter().enumerate() {
                println!("{:}. {:}", i + 1, ability);
            }

            let boost = loop {
                input_buffer.clear();
                io::stdin().read_line(&mut input_buffer).expect("failed to readline");
    
                if let Ok(input_number) = input_buffer.trim().parse::<usize>() {
                    let ability = free_boosts.get(input_number - 1);
                    match ability {
                        Some(ability) => break *ability, 
                        _ => ()
                    };
                }
    
                println!("Invalid input, please try again.");
            };

            if let Some(pos) = free_boosts.iter().position(|x| x == &boost) {
                free_boosts.remove(pos);
            }

            boosts.push(boost);
        };

        println!("Please select one boost:\n\
                      1. {:}\n\
                      2. {:}\
            ", background.ability_boosts.0, background.ability_boosts.1);

        boosts.push(loop {
            input_buffer.clear();
            io::stdin().read_line(&mut input_buffer).expect("failed to readline");

            if let Ok(input_number) = input_buffer.trim().parse::<u8>() {
                match input_number {
                    1 => break background.ability_boosts.0,
                    2 => break background.ability_boosts.1,
                    _ => ()
                };
            }

            println!("Invalid input, please try again.");
        });

        println!("Please select one boost:");

        for (i, ability) in Ability::iter().enumerate() {
            println!("{:}. {:}", i + 1, ability);
        }

        boosts.push(loop {
            input_buffer.clear();
            io::stdin().read_line(&mut input_buffer).expect("failed to readline");

            if let Ok(input_number) = input_buffer.trim().parse::<u8>() {
                match input_number {
                    1 => break Ability::Strength,
                    2 => break Ability::Dexterity,
                    3 => break Ability::Consitution,
                    4 => break Ability::Intelligence,
                    5 => break Ability::Wisdom,
                    6 => break Ability::Charisma,
                    _ => ()
                };
            }

            println!("Invalid input, please try again.");
        });

        for boost in boosts {
            if let Some(modifier) = _abilties.get_mut(&boost) {
                *modifier += 1;
            }
        }

        Self {
            ancestry:      Ancestry::template(ancestry_template),
            background,
            creature_data,
        }
    }
}