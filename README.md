# adventofcode2014

My solutions to the Advent of Code 2024 puzzles in Rust 🦀

## Requirements

* [Rust](https://www.rust-lang.org/) 1.56.1 or later

## Running the tests

### All puzzles for each day

```sh
cargo test
```

#### With slow tests

```sh
cargo test --features slow
```

#### With tests for utility functions

```sh
cargo test --features utils
```

### Both puzzles for one day

```sh
cargo test day_01 --all-features
```

### Single puzzle

```sh
cargo test day_01_part_1 --all-features
```