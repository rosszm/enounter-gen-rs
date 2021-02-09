// Zack Ross
// zmr462
// 11215196

// monster module

use std::num::ParseIntError;
use std::cmp::{Ord, Ordering};


/// Represent a Monster.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd)]
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

    /// Returns the name of the monster.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the initiative of the monster.
    pub fn init(&self) -> i32 {
        self.init
    }

    /// Returns the armour stat of the monster.
    pub fn armour(&self) -> u32 {
        self.arm
    }

    /// Returns the attack stat of the monster.
    pub fn attack(&self) -> u32 {
        self.atk
    }

    /// Returns the challenge rating of the monster.
    pub fn rating(&self) -> u32 {
        self.cr
    }

    /// Compares two monsters based on initiative. Allows monsters to be sorted
    /// by init.
    /// * `other` - another monster
    pub fn cmp_init(&self, other: &Self) -> Ordering {
        (self.init).cmp(&other.init)
    }

    /// Compares two monsters based on name. Allows monsters to be sorted by 
    /// name. This is the default comparison for `Ord`.
    /// * `other` - another monster
    pub fn cmp_name(&self, other: &Self) -> Ordering {
        (self.name).cmp(&other.name)
    }
}

// default comparison is by unique name
impl Ord for Monster {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_name(other)
    }
}
