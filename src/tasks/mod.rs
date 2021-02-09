// Zack Ross
// zmr462
// 11215196

// modules containing all the task functions to be used in the main program.

extern crate encounters;
use encounters::monsters::Monsters;

mod gen;
use gen::Input;
use gen::{user_input, encounter, encounter_struct};


/// performs the task 1a encounter
/// * `monsters` - a roster of monsters.
pub fn task1a(monsters: &Monsters) {
    match gen::user_input() {
        Some(Input::Num(cr)) => {
            gen::encounter(cr, monsters, true);
        },
        _ => println!("please enter a positive integer")
    };
}

/// performs the task 1b encounter
/// * `monsters` - a roster of monsters.
pub fn task1b(monsters: &Monsters) {
    match user_input() {
        Some(Input::Num(cr)) => {
            encounter(cr, monsters, false);
        },
        _ => println!("please enter a positive integer")
    };
}

/// performs the task 2a encounter loop
/// * `monsters` - a roster of monsters.
pub fn task2a(monsters: &Monsters) {
    loop {
        match user_input() {
            Some(Input::Num(cr)) => {
                encounter(cr, monsters, true);
            },
            Some(Input::Quit) => std::process::exit(0),
            None => println!("please enter a positive integer ('Q' or 'q' to quit)")
        };   
    }
}

/// performs the task 2b encounter loop
/// * `monsters` - a roster of monsters.
pub fn task2b(monsters: &Monsters) {
    loop {
        match user_input() {
            Some(Input::Num(cr)) => {
                encounter(cr, monsters, false);
            },
            Some(Input::Quit) => std::process::exit(0),
            None => println!("please enter a positive integer ('Q' or 'q' to quit)")
        };   
    }
}

/// performs the task 3 encounter loop
/// * `monsters` - a roster of monsters.
pub fn task3(monsters: &Monsters) {
    loop {
        match user_input() {
            Some(Input::Num(cr)) => {
                encounter_struct(cr, monsters, true);
            },
            Some(Input::Quit) => std::process::exit(0),
            None => println!("please enter a positive integer ('Q' or 'q' to quit)")
        };   
    }
}