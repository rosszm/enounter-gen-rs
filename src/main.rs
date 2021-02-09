// Zack Ross 
// zmr462
// 11215196

mod monsters;
use monsters::Monsters;

use std::env;
use std::io;
use std::io::Write;

fn encounter(monsters: Monsters) {
    print!("Enter party capability: ");
    io::stdout().flush().unwrap();      // we can't fix this problem!

    let mut i = String::new();
    io::stdin().read_line(&mut i).unwrap();
    let cr: u32 = i.trim().parse::<u32>().unwrap_or_default();
    
    task1a(monsters, cr);
}

fn task1a(monsters: Monsters, cr: u32) {
    let mut mons_rating = 0;
    for monster in monsters.iter(true) {
        if mons_rating >= cr {
            break;
        }
        monster.print();
        mons_rating += monster.rating()
    }
    println!("  total challenge rating: {}\n", mons_rating);
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


