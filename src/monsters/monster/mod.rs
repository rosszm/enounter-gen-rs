// Zack Ross
// zmr462
// 11215196

use std::num::ParseIntError;


/// Represent a Monster.
#[derive(Debug)]
pub struct Monster {
    name: String,
    init: i32,
    arm: u32,
    atk: u32,
    cr: u32,
}

impl Monster {
    /// Prints out the monster
    pub fn print(&self) {
        println!("    {}: init {}, Armour {}, Attack {}, Challenge {}", 
    self.name, self.init, self.arm, self.atk, self.cr);
    }
    
    /// Create a new Monster from a space separated string. Returns an error
    /// if the string data cannot be parsed.
    /// 
    /// * `s` - a string containing the monster data.
    pub fn parse(s: &str) -> Result<Monster, ParseIntError> {
        let vec: Vec<&str> = s.split(' ').collect();
        Ok(Monster {
            name: vec[0].to_string(),
            init: vec[1].parse()?,
            arm: vec[2].parse()?,
            atk: vec[3].parse()?,
            cr: vec[4].parse()?,
        })
    }

    /// Returns the challenge rating of the monster.
    pub fn rating(&self) -> u32 {
        self.cr
    }
}
