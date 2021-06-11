// Zack Ross 
// zmr462
// 11215196

// main program for task 2b of encounters project.


extern crate encounters;
use std::fs::File;
use encounters::logger::EncounterLogger;
use encounters::encounter::Encounter;
use encounters::monsters::Monsters;

mod io;
use io::Input;
use io::user_input;

use std::env;


/// Perform an encounter based on the specified options. Returns the encounter
/// 
/// * `cr` - party challenge rating
/// * `mons` - reference to the monster roster
pub fn encounter(cr: u32, monsters: &Monsters) -> Encounter {
    let mut enc = Encounter::new();
    for monster in monsters.iter() {
        if enc.rating() >= cr {
            break;
        }
        enc.add(monster);
    }
    println!("{}", enc);
    enc
}

/// performs the task 2b encounter loop
/// * `monsters` - a roster of monsters.
/// 
/// ### Pre-Conditions:
/// - `monsters` must not be empty
pub fn task2b(monsters: &Monsters, log: &mut Option<EncounterLogger>) {
    assert!(monsters.len() > 0, "Cannot generate an encounter with an empty roster!");
    loop {
        match user_input() {
            Some(Input::Num(cr)) => {
                if cr > monsters.total_rating() {
                    println!("Party capability exceeds the max challenge rating of the roster!");
                    return
                }
                let enc = encounter(cr, monsters);

                if let Some(logger) = log {
                    match logger.log(enc){
                        Ok(_) => (),
                        Err(_) => eprintln!("Could not log encounter!")
                    }
                }
            },
            Some(Input::Quit) => {
                if let Some(logger) = log {
                    match logger.log_summary() {
                        Ok(_) => (),
                        Err(_) => eprintln!("Could not log summary!")
                    }
                }
                std::process::exit(0)
            },
            None => println!("please enter a positive integer ('Q' or 'q' to quit)")
        };   
    }
}

// main program
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let fname = args[1].as_str();
        let mut logger: Option<EncounterLogger> = None;
        if args.len() > 2 {
            match args[2].as_str() {
                "-f" => {
                    let f= File::create(&args[3]).expect("Could not create log file!");
                    logger = Some(EncounterLogger::new(f));
                },
                _ => ()
            }
        }

        match Monsters::from(fname) {
            Ok(ms) => { 
                println!("Read {} monsters.", ms.len());
                task2b(&ms, &mut logger);
            },
            Err(e) => println!("Reading {} failed: {}", fname, e),
        }
    } else {
        println!("Usage: {} <monsters-file-name>", args[0])
    }
}
