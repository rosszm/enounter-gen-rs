// Zack Ross
// zmr462
// 11215196

// encounter module

use std::fmt;

use crate::monsters;
use monsters::Monster;


/// Represents an encounter for task 3
pub struct Encounter<'a> {
    /// The number of times a monster is used in an encounter mapped to its
    /// reference
    monsters: Vec<&'a Monster>,
    rating: u32
}

impl<'a> Encounter<'a> {
    /// Returns an empty encounter.
    pub fn new() -> Self {
        Encounter {
            monsters: Vec::new(),
            rating: 0
        }
    }

    /// Returns the total challenge rating of the encounter.
    pub fn rating(&self) -> u32 {
        self.rating
    }

    /// Adds a monster to the encounter.
    /// 
    /// * `monster` - a reference to the monster to be added
    /// 
    /// ### Pre-Conditions:
    /// - `monster` is a valid monster reference
    /// 
    /// ### Post-Conditions:
    /// - reference to the monster is added to this encounter
    /// - rating of the monster is added to the total encounter rating
    pub fn add(&mut self, monster: &'a Monster) {
        self.monsters.push(monster);
        self.rating += monster.rating();
    }

    /// Returns a vector of sorted encounter monsters.
    /// 
    /// ### Pre-Conditions:
    /// - encounter monsters can be sorted
    /// - `Monster` implements `Ord`
    fn sorted(&self) -> Vec<&'a Monster> {
        let mut sorted = self.monsters.clone();
        sorted.sort_by(|m0, m1| m0.cmp(m1));
        sorted
    }

    /// Returns an iterator over the monster of the encounter and their usage.
    pub fn iter(&self) -> EncounterIterator {
        let mons = self.sorted();  
        EncounterIterator::create(mons)
    }
}

/// Allows the Encounter to be displayed as a string.
impl<'a> fmt::Display for Encounter<'a> {
    /// Format the Encounter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut enc_str = String::from("");
        for (mon, usage) in self.iter() {
            enc_str +=
                format!("    {}: init {}, Armour {}, Attack {}, Challenge {}\n",
                    if usage == 1 { 
                        mon.name().to_string()
                    } else { 
                        format!("{} {}s", usage, mon.name())
                    },
                    mon.init(), mon.armour(), mon.attack(), mon.rating() * usage
                ).as_str();
        }
        enc_str += format!("  total challenge rating: {}\n", self.rating).as_str();

        write!(f, "{}", enc_str)
    }
}


/// Iterator structure for an encounter
/// 
/// This struct is used to iterate through the monster in an encounter and 
/// their usage within that encounter.
pub struct EncounterIterator<'a> {
    monsters: Vec<&'a Monster>,
    idx: isize,
    cur: Option<&'a Monster>,
}

impl<'a> EncounterIterator<'a> {
    /// Returns an iterator over the monsters in an encounter.
    /// 
    /// * `mons` - the vector of sorted monsters
    fn create(mons: Vec<&'a Monster>) -> Self {
        EncounterIterator {
            monsters: mons,
            idx: -1,
            cur: None,
        }
    }
}
impl<'a> Iterator for EncounterIterator<'a,> {
    /// A monster reference and the number of times it is used in the
    /// encounter.
    type Item = (&'a Monster, u32);

    /// Get the next monster-usage pair of the iterator
    fn next(&mut self) -> Option<Self::Item> { 
        let mut usage = 1;
        loop {
            self.idx += 1;
            match self.monsters.get(self.idx as usize) {
                Some(next) => {
                    match self.cur {
                        Some(cur) => {
                            if cur == *next {
                                usage += 1;
                            }
                            else {
                                self.cur = Some(*next);
                                break Some((cur, usage));
                            }
                        },
                        None => {
                            self.cur = Some(*next);
                        }
                    }
                },
                None => {
                    break match self.cur {
                        Some(cur) => {
                            self.cur = None;
                            Some((cur, usage))
                        },
                        None => None
                    }
                }
            };
        }
    }
}