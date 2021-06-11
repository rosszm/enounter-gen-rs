# encounter-gen-rs

Generates a DnD-like monster encounter from a provided set of monsters 
and their stats based on the capability or level of a party.

This was created throughout a number of exercises as part of CMPT 470
Winter 2021 Term.

# Usage

The following can be used to build and compile the exercises:

## Compile & Run

In the `./` directory of the exercise, use following to build the program:

`cargo build`

Then use:

`cargo run --bin $TASK $FILE`

or

`./target/debug/$TASK $FILE`

Where `$FILE` is the name/path of a monster db file, and `$TASK` is one of the 
following:
 - `task1a` -- single iteration, allows repeats of monsters
 - `task1b` -- single iteration, does not allow repeats of monsters
 - `task2a` -- multiple encounters, allows repeats of monsters
 - `task2b` -- multiple encounters, does not allow repeats of monsters
 - `task3` -- the same as `task2a`; all tasks now use the `Encounter` struct.



### Example Usage:
`./target/debug/task1a monsters.txt`

will run the task 1a binary using the roster from the file `monsters.txt`.

## Input Format

Monsters must have the following information and stats associated with them:

| Name      | Initiative | Armour | Attack | Challenge Rating |
|-----------|:----------:|:------:|:------:|:----------------:|
| example_1 |         -1 |     17 |      9 |               10 |
| example_2 |          2 |     15 |      4 |                1 |


The Input file for the monsters should be a plain text file
with each field delineated by a space, `' '` character, like such:

```text
monster_1 -1 17 9 10
monster_2 2 15 4 1
```


# Repository Contents

This repository includes:
- `src/lib.rs` -- the monsters library
- `src/encounter/` -- module containing the encounter representation
- `src/logger/` -- module containing structs, and function related to file 
    logging
- `src/monsters/` -- module containing the representation of the collection of
    monsters, and the actions that can be performed on them.
- `src/tasks/` -- directory containing the main programs for reach task
- `Cargo.toml` -- the cargo imports