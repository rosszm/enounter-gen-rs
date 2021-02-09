// Zack Ross 
// zmr462
// 11215196


extern crate encounters;
use encounters::monsters::Monsters;

use std::env;
use std::io;
use std::io::Write;

fn encounter(ms: Monsters) {
    print!("Enter party capability: ");
    io::stdout().flush().unwrap();      // we can't fix this problem!

    let mut i = String::new();
    io::stdin().read_line(&mut i).unwrap();
    let cr: u32 = i.trim().parse::<u32>().unwrap_or_default();
    ms.encounter(cr);
}

// main program
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let fname = args[1].as_str();
        match Monsters::from(fname) {
            Ok(ms) => { println!("Read {} monsters.", ms.len());
                        encounter(ms)
                      },
            Err(e) => println!("Reading {} failed: {}", fname, e),
        }
    } else {
        println!("Usage: {} <monsters-file-name>", args[0])
    }
}


