## How to use
The project structure is as follows:
```console
.
├── Cargo.lock
├── Cargo.toml
└── src
   ├── bin
   │  ├── day1.rs
   │  └── get_input.rs
   ├── lib.rs
   └── main.rs
```
To run the program for a particular day, run `cargo run --bin day{day}`
To retrieve the puzzle input for a particular day and write the output to a `input.txt` in the current directory,
run `cargo run --bin get_input {day}`

## Credits
The aoc library functions are taken from [tjdeveries](https://github.com/tjdevries/advent_of_code/tree/master/2021)'s AOC 2021 repo
