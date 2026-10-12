# aoclp_solutions

My solutions to [Advent of Code](https://adventofcode.com/) puzzles in Rust 🦀

## Requirements

* [Rust](https://www.rust-lang.org/) 1.91.1 or later

## Setting your session cookie

* Get your AoC session cookie by following [these instructions](https://github.com/nuxeh/aocf/blob/HEAD/cookie.md)
* Save it in a file named `.env` at the repository root:

```dotenv
AOCLP_AOC_SESSION=<COOKIE>
```

## Finding solutions

If `--year` is not specified, it defaults to the latest year that has solutions.

### All solutions in a year

```shell
cargo run -- --year 2024
```

### Only a specific day

```shell
cargo run -- --year 2024 --day 1
```

### Only a specific part

```shell
cargo run -- --year 2024 --day 1 --part 1
```
