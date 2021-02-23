Zack Ross\
zmr462\
11215196

# Exercise 5

This exercise is build off the work for ex4.

This archive includes:

- README this file!
- Discuss.txt
- monsters.db

- src/lib.rs -- the monsters library
- src/tasks/ -- directory containing the main programs for reach task
- Cargo.toml -- the cargo imports


Compile Instructions
--------------------
In the `./` directory of this archive, use following to build the program:

`cargo build`

Then use:

`cargo run --bin $TASK $FILE`

or

`./target/debug/$TASK $FILE`

Where `$FILE` is the name/path of a monster db file, and `$TASK` is one of the 
following:
 - `task1a` 
 - `task1b` 
 - `task2a` 
 - `task2b` 
 - `task3`

### Example Usage:
`./target/debug/task1a monsters.db`

will run the task 1a binary using the roster from the file `monsters.db`.