// Zack Ross 
// zmr462
// 11215196

// main program for task 2b of encounters project.


extern crate encounters;
use encounters::monsters::Monsters;

mod io;
use io::Input;
use io::user_input;

use std::env;


/// Perform an encounter based on the specified options.
/// 
/// * `cr` - party challenge rating
/// * `mons` - reference to the monster roster
pub fn encounter(cr: u32, monsters: &Monsters) {
    if cr > monsters.total_rating() {
        println!("Party capability exceeds the max challenge rating of the roster!");
        return
    }
    let mut mons_rating = 0;
    for monster in monsters.iter() {
        if mons_rating >= cr {
            break;
        }
        monster.print();
        mons_rating += monster.rating();
    }
    println!("  total challenge rating: {}\n", mons_rating);
}

/// performs the task 2b encounter loop
/// * `monsters` - a roster of monsters.
pub fn task2b(monsters: &Monsters) {
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
                task2b(&ms);
            },
            Err(e) => println!("Reading {} failed: {}", fname, e),
        }
    } else {
        println!("Usage: {} <monsters-file-name>", args[0])
    }
}
