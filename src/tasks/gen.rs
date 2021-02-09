// Zack Ross
// zmr462
// 11215196

// module containing parts of the main program that involve generating an encounter.


extern crate encounters;
use encounters::encounter::Encounter;
use encounters::monsters::Monsters;

use std::io;
use std::io::Write;

/// Valid user input options
pub enum Input {
    Num(u32),
    Quit
}

/// Prompts user for a party capability. Returns `Input` if it is a valid
/// user input; otherwise it returns `None`.
/// 
/// Valid inputs include:
/// * str value of `"Q"` or `"q"`
/// * a `u32` integer
pub fn user_input() -> Option<Input> {
    print!("Enter party capability: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("could not read input");
    input = input.trim().to_string();

    // input case handling
    if input == "Q" || input == "q" {
        Some(Input::Quit)
    }
    else {
        match input.parse::<u32>() {
            Ok(n) => Some(Input::Num(n)),
            Err(_) => None,
        }
    }
}

/// Perform an encounter based on the specified options.
/// 
/// * `cr` - party challenge rating
/// * `mons` - reference to the monster roster
/// * `repeats` - whether or not to allow repeated monsters in an encounter.
pub fn encounter(cr: u32, monsters: &Monsters, repeats: bool) {
    if !repeats {
        if cr > monsters.total_rating() {
            println!("Party capability exceeds the max challenge rating of the roster!");
            return
        }
    }
    let mut mons_rating = 0;
    for monster in monsters.iter(repeats) {
        if mons_rating >= cr {
            break;
        }
        monster.print();
        mons_rating += monster.rating();
    }
    println!("  total challenge rating: {}\n", mons_rating);
}

/// Perform an encounter based on the specified options. This function uses
/// The `Encounter` struct to generate an encounter for task 3.
/// 
/// * `cr` - party challenge rating
/// * `mons` - reference to the monster roster
/// * `repeats` - whether or not to allow repeated monsters in an encounter.
pub fn encounter_struct(cr: u32, monsters: &Monsters, repeats: bool) {
    if !repeats {
        if cr > monsters.total_rating() {
            println!("Party capability exceeds the max challenge rating of the roster!");
            return
        }
    }
    let mut enc = Encounter::new();
    for monster in monsters.iter(repeats) {
        if enc.rating() >= cr {
            break;
        }
        enc.add(monster);
    }
    enc.sort();
    enc.print();
}