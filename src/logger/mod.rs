// Zack Ross
// zmr462
// 11215196

// logger module

use crate::encounter::Encounter;
use std::fs::File;
use std::io::Write;
use std::io;
use std::collections::HashMap;


/// The `EncounterLogger` structure represent a logger for encounters within
/// the monsters generator program.
#[derive(Debug)]
pub struct EncounterLogger {
    file: File,
    num_enc: u32,
    monsters: HashMap<String, (u32, u32)>,
}

impl<'a> EncounterLogger {
    /// Create a new encounter logger that logs the encounter to the given
    /// given file. If the file `fname` does not exist, a new file with the name
    /// `fname` will be created.
    /// - `fname` - the log file name
    pub fn new(file: File) -> Self {
        EncounterLogger {
            file: file,
            num_enc: 0,
            monsters: HashMap::new(),
        }
    }

    /// Logs a given encounter to the log file. Returns an error or how many
    /// bytes were written to the log file.
    ///  - `enc` - the encounter to log
    pub fn log(&mut self, enc: Encounter) -> Result<usize, io::Error> {
        for (mon, usage) in enc.iter() {
            self.monsters.entry(mon.name().to_string()).or_insert((usage, 0)).1 += 1;
        }
        self.num_enc += 1;
        self.file.write(format!("{}\n", enc).as_bytes())
    }

    /// Logs a summary of all previously logged encounters. Returns an error
    /// if unable to log the summary.
    pub fn log_summary(&mut self) -> Result<(), io::Error> {
        for (mon, (usage, encounters)) in self.monsters.iter() {
            self.file.write(format!("{} selected {} times in {} encounters.\n", 
            mon, usage, encounters).as_bytes())?;
        }
        self.file.write(format!("\n{} encounters, total", self.num_enc).as_bytes())?;
        Ok(())
    }
}