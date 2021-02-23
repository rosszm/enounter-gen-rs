// Zack Ross
// zmr462
// 11215196

// monster iter module


use crate::monsters::Monster;
use crate::monsters::Monsters;

use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;


/// Iterator structure for monsters. This structure iterates over randomly
/// selected monsters.
/// 
/// This iterator does not support multiple visits of a single monster.
/// 
/// This struct allows for monster to be removed without without effecting the
/// original roster of monsters.
pub struct RandNonRepIterator<'a,> {
    /// Vector of references to Monsters.
    monsters: Vec<&'a Monster>,
}

impl<'a> RandNonRepIterator<'a,> {
    /// Returns an new Iterator over monsters.
    /// 
    /// * `mons` - the Monsters struct to iterate over
    pub fn create(mons: &'a Monsters) -> Self {
        RandNonRepIterator {
            monsters: mons.values.iter().collect()
        }
    }
}

impl<'a> Iterator for RandNonRepIterator<'a,> {
    type Item = &'a Monster;

    fn next(&mut self) -> Option<&'a Monster> { 
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.monsters.len());
        let monster = match self.monsters.get(index) {
            Some(monster) => Some(*monster),
            None => None
        };
        self.monsters.swap_remove(index);
        monster
    }
}


/// Iterator structure for monsters. This structure iterates over randomly 
/// selected monsters. 
/// 
/// This iterator supports multiple visits of a single monster.
pub struct RandRepIterator<'a> {
    /// Reference to a vector of monsters
    monsters: &'a Monsters
}

impl<'a> RandRepIterator<'a,> {
    /// Returns an new Iterator over monsters.
    /// 
    /// * `mons` - the Monsters struct to iterate over
    pub fn create(mons: &'a Monsters) -> Self {
        RandRepIterator {
            monsters: mons
        }
    }
}

impl<'a> Iterator for RandRepIterator<'a,> {
    type Item = &'a Monster;

    fn next(&mut self) -> Option<&'a Monster> { 
        let mut rng = thread_rng();
        self.monsters.values.choose(&mut rng)
    }
}