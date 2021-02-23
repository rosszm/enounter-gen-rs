// Zack Ross
// zmr462
// 11215196

// module containing parts of the main program that involve generating an encounter.

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
