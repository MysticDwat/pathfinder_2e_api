pub fn input_ancestry() -> &(&str, u8, u8, Size){
    let mut input_buffer: String = String::new();

    println!("Please Enter Ancestry Number:\n\
            1. Dwarf\n\
            2. Elf\n\
            3. Gnome\n\
            4. Goblin\n\
            5. Halfling\n\
            6. Human\
    ");

    loop {
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
    }
}

pub fn input_background() -> &(&str, (Ability, Ability), &str, Skill, &str) {
    let mut input_buffer: String = String::new();

    println!("Please Enter Background Number:\n\
        1. Farmhand\
    ");

    loop {
        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).expect("failed to readline");

        if let Ok(input_number) = input_buffer.trim().parse::<u8>() {
            match input_number {
                1 => break &FARMHAND,
                _ => ()
            };
        }

        println!("Invalid input, please try again.");
    }
}

pub fn input_abilities() -> Vec<Ability> {
    let mut boosts = Vec::<Ability>::new();
    let mut free_boosts: Vec<Ability> = Ability::iter().collect();
    let mut ancestry_boosts = free_boosts.clone();

    boosts.push(class.key_ability);

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

    boosts
}