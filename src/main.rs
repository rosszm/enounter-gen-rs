// Zack Ross 
// zmr462
// 11215196

// main program for encounters project.


extern crate encounters;
use encounters::monsters::Monsters;

mod tasks;

use std::env;


// main program
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let fname = args[2].as_str();
        match Monsters::from(fname) {
            Ok(ms) => { 
                println!("Read {} monsters.", ms.len());
                // perform task based on command input
                match args[1].as_str() {
                    "1a" => tasks::task1a(&ms),
                    "1b" => tasks::task1b(&ms),
                    "2a" => tasks::task2a(&ms),
                    "2b" => tasks::task2b(&ms),
                    "3" => tasks::task3(&ms),
                    _ => println!("Task {} does not exist", args[1]),
                }
            },
            Err(e) => println!("Reading {} failed: {}", fname, e),
        }
    } else {
        println!("Usage: {} <task> <monsters-file-name>", args[0])
    }
}


