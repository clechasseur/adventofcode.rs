# aoclp_solutions

My solutions to Advent of Code puzzles in Rust 🦀

## Requirements

* [Rust](https://www.rust-lang.org/) 1.74.1 or later
* [aocf](https://crates.io/crates/aocf) CLI

## Setting your session cookie

* Get your AoC session cookie by following [these instructions](https://github.com/nuxeh/aocf/blob/HEAD/cookie.md)
* Save it locally by running:

```shell
aocf set-cookie <COOKIE>
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

## Running tests

The tests are only for the helpers.

```shell
cargo test
```
