// Zack Ross
// zmr462
// 11215196

pub mod monster;
use monster::Monster;

use rand::{thread_rng, Rng};
use std::fs::File;
use std::io;
use std::io::ErrorKind as IoError;
use std::io::prelude::*;


/// A roster of Monsters
#[derive(Debug)]
pub struct Monsters {
    values: Vec<Monster>
}

impl Monsters {
    /// Returns a `Result` that contains a new roster of Monsters from a given 
    /// file if the operation was successful; otherwise it contains an io error. 
    /// 
    /// * `fname` - the name of a file to read from
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
            values: Vec::with_capacity(size)
        };

        while reader.read_line(&mut buf)? > 0 {
                let line = buf.trim_end();
                let monster = match Monster::parse(&line) {
                    Ok(s) => s,
                    Err(_) => return Err(io::Error::from(IoError::InvalidData))
                };
                monsters.values.push(monster);
                buf.clear();
        }
        Ok(monsters)
    }
    
    /// Returns the number of monsters in the roster.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns an iterator over the roster of Monsters.
    /// 
    /// * `repeats` - whether or not monsters can be repeated
    pub fn iter(&self, repeats: bool) -> MonsterIter {
        MonsterIter::create(&self, repeats)
    }
}


/// Iterator structure for monsters
/// 
/// This struct allows for monster to be removed without without effecting the
/// original roster of monsters.
pub struct MonsterIter<'a,> {
    /// Represents whether or not monsters can be repeated.
    repeats: bool,
    /// Vector of references to Monsters.
    monsters: Vec<&'a Monster>,
}

impl<'a> MonsterIter<'a,> {
    /// Returns an new Iterator over monsters.
    /// 
    /// * `mons` - the Monsters struct to iterate over
    /// * `repeats` - whether or not monsters can be repeated
    fn create(mons: &'a Monsters, repeats: bool) -> Self {
        MonsterIter {
            repeats: repeats,
            monsters: mons.values.iter().collect()
        }
    }
}

impl<'a> Iterator for MonsterIter<'a,> {
    type Item = &'a Monster;

    fn next(&mut self) -> Option<&'a Monster> { 
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.monsters.len());
        let monster = match self.monsters.get(index) {
            Some(monster) => Some(*monster),
            None => None
        };
        if !self.repeats {
            self.monsters.swap_remove(index);
        }
        monster
    }
}