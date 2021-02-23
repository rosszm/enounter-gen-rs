// Zack Ross
// zmr462
// 11215196

// encounter module

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
    pub fn add(&mut self, monster: &'a Monster) {
        self.monsters.push(monster);
        self.rating += monster.rating();
    }

    /// Sorts the encounter by monster initiative.
    pub fn sort(&mut self) {
        self.monsters.sort_by(|m0, m1| m0.cmp(m1))
    }

    /// Prints out the encounter.
    pub fn print(&self) {
        let mut usage = 1;
        let mut previous: Option<&&Monster> = None;
        for cur in self.monsters.iter() {
            usage = match previous {
                Some(prev) => {
                    if cur == prev {
                        usage + 1
                    }
                    else {
                        self.print_monster(*cur, usage);
                        1
                    }
                },
                None => {
                    self.print_monster(*cur, usage);
                    1
                }
            };
            previous = Some(cur);
        }
        println!("  total challenge rating: {}\n", self.rating);
    }

    /// Prints out a single monster
    fn print_monster(&self, mon: &Monster, usage: u32) {
        println!("    {}: init {}, Armour {}, Attack {}, Challenge {}",
            if usage == 1 { 
                mon.name().to_string()
            } else { 
                format!("{} {}s", usage, mon.name())
            },
            mon.init(), mon.armour(), mon.attack(), mon.rating() * usage
        );
    }
}