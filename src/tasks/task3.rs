// Zack Ross 
// zmr462
// 11215196

// main program for task 3 of encounters project.


extern crate encounters;
use encounters::monsters::Monsters;
use encounters::encounter::Encounter;

mod io;
use io::Input;
use io::user_input;

use std::env;


/// Perform an encounter based on the specified options.
/// 
/// * `cr` - party challenge rating
/// * `mons` - reference to the monster roster
pub fn encounter(cr: u32, monsters: &Monsters) {
    let mut enc = Encounter::new();
    for monster in monsters.iter_with_repeats() {
        if enc.rating() >= cr {
            break;
        }
        enc.add(monster);
    }
    enc.sort();
    enc.print();
}

/// performs the task 3 encounter loop
/// * `monsters` - a roster of monsters.
pub fn task3(monsters: &Monsters) {
    loop {
        match user_input() {
            Some(Input::Num(cr)) => {
                encounter(cr, monsters);
            },
            Some(Input::Quit) => std::process::exit(0),
            None => println!("please enter a positive integer ('Q' or 'q' to quit)")
        };   
    }
}

// main program
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let fname = args[1].as_str();
        match Monsters::from(fname) {
            Ok(ms) => { 
                println!("Read {} monsters.", ms.len());
                task3(&ms);
            },
            Err(e) => println!("Reading {} failed: {}", fname, e),
        }
    } else {
        println!("Usage: {} <monsters-file-name>", args[0])
    }
}
