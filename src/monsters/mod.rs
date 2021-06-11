// Zack Ross
// zmr462
// 11215196

// monsters module


mod monster;
pub use monster::Monster;

mod iter;
pub use iter::RandRepIterator;
pub use iter::RandNonRepIterator;

use std::fs::File;
use std::io;
use std::io::ErrorKind as IoError;
use std::io::prelude::*;


/// A roster of Monsters
#[derive(Debug)]
pub struct Monsters {
    values: Vec<Monster>,
    rating: u32
}

impl Monsters {
    /// Returns a `Result` that contains a new roster of Monsters from a given 
    /// file if the operation was successful; otherwise it contains an io error. 
    /// 
    /// * `fname` - the name of a file to read from
    /// 
    /// ### Pre-Conditions:
    /// - `fname` must be a valid file to succeed.
    /// 
    /// ### Post-Conditions:
    /// - returns a `Monsters` structure containing all the `Monster` from the 
    /// file if valid.
    pub fn from(fname: &str) -> Result<Monsters, io::Error> {
        let file =  File::open(fname)?;
        let mut reader = io::BufReader::new(file);
        let mut buf = String::new();
        
        reader.read_line(&mut buf)?;
        let size: usize =  match buf.trim_end().parse() {
            Ok(s) => s,
            Err(_) => return Err(io::Error::from(IoError::InvalidData))
        };
        buf.clear();
        let mut monsters = Monsters {
            values: Vec::with_capacity(size),
            rating: 0
        };

        while reader.read_line(&mut buf)? > 0 {
                let line = buf.trim_end();
                let monster = match Monster::parse(&line) {
                    Ok(s) => s,
                    Err(_) => return Err(io::Error::from(IoError::InvalidData))
                };
                monsters.rating += monster.rating();
                monsters.values.push(monster);
                buf.clear();
        }
        Ok(monsters)
    }
    
    /// Returns the number of monsters in the roster.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns the total challenge rating of all monsters in the roster.
    pub fn total_rating(&self) -> u32 {
        self.rating
    }

    /// Returns an iterator over the roster of Monsters that can visit a single
    /// monster only once.
    /// 
    pub fn iter(&self) -> RandNonRepIterator {
        RandNonRepIterator::create(&self)
    }

    /// Returns an iterator over the roster of Monsters that can visit a single
    /// monster more than once. 
    /// 
    /// Since repetitions are allowed, this iterator will be generate monsters 
    /// infinitely.
    pub fn iter_with_repeats(&self) -> RandRepIterator {
        RandRepIterator::create(&self)
    }
}