// Zack Ross
// zmr462
// 11215196

// encounter module

use crate::monsters;
use monsters::Monster;

use indexmap::IndexMap;


/// Represents an encounter for task 3
pub struct Encounter<'a> {
    /// The number of times a monster is used in an encounter mapped to its
    /// reference
    usage_map: IndexMap<&'a Monster, u32>,
    rating: u32
}

impl<'a> Encounter<'a> {
    /// Returns an empty encounter.
    pub fn new() -> Self {
        Encounter {
            usage_map: IndexMap::new(),
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
        *self.usage_map.entry(monster).or_insert(0) += 1;
        self.rating += monster.rating();
    }

    /// Sorts the encounter by monster initiative.
    pub fn sort(&mut self) {
        self.usage_map.sort_by(|m0, _, m1, _| m0.cmp_init(m1))
    }

    /// Prints out the encounter.
    pub fn print(&self) {
        for (mon, usage) in self.usage_map.iter() {
            println!("    {}: init {}, Armour {}, Attack {}, Challenge {}",
            if *usage == 1 { 
                mon.name().to_string()
            } else { 
                format!("{} {}s", usage, mon.name())
            },
            mon.init(), mon.armour(), mon.attack(), mon.rating() * usage
            );
        }
        println!("  total challenge rating: {}\n", self.rating);
    }
}