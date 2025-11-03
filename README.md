# 🚀 jaoc - Jeremy's Advent of Code Scaffolder

`jaoc` is a small command-line tool to quickly set up and manage a multi-bin Rust project for solving [Advent of Code](https://adventofcode.com/) puzzles.

It's designed to get you from a new year to solving Day 1 in just three commands, letting you focus on the puzzle logic instead of boilerplate.

## Features

* **Fast Compile Times:** Generates a [multi-bin](https://www.google.com/search?q=https://doc.rust-lang.org/cargo/reference/cargo-targets.html%23binaries) project structure. Each day is its own binary, so `cargo run --bin day05` only compiles Day 5.
* **Zero Boilerplate:** The `jaoc start` command generates a template with a simple `aoc_main!` macro. You just write your `part1` and `part2` functions.
* **Automatic Scaffolding:** `jaoc new` sets up the entire project for a given year, and `jaoc start` creates the binary and data files for a given day.
* **Flexible Parsing:** The `part1` and `part2` functions are independent and both receive the raw input string, allowing for different parsing logic if a puzzle requires it.

## Prerequisites

Before you can use `jaoc`, you'll need:

1.  **Rust & Cargo:** (Install via [rustup](https://rustup.rs/))
2.  **`cargo-generate`:** This is used by the `new` command.
    ```bash
    cargo install cargo-generate
    ```

## Installation

You can install `jaoc` directly from this repository (once you've pushed it to GitHub):

```bash
# Recommended: Install from your git repo
cargo install --git https://github.com/jviguy/jaoc.git
```

Or, you can clone this repo and install it locally:

```bash
git clone https://github.com/jviguy/jaoc.git
cd jaoc
cargo install --path .
```

## Workflow & Usage

This is the entire workflow, from starting a new year to running your first solution.

### 1\. Create a New Year Project

Run `jaoc new` with the desired year. This will use `cargo generate` to clone the template and name it `aoc_YYYY`.

```bash
# This creates a new folder named "aoc_2025"
jaoc new 2025
```

### 2\. Enter the Project

All other commands must be run from *inside* the newly created project folder.

```bash
cd aoc_2025
```

### 3\. Start a New Day

Run `jaoc start` with the day number. This scaffolds all the files you need.

```bash
# Scaffolds:
# 1. ./src/bin/day01.rs      (Your solution file)
# 2. ./data/inputs/day01.txt   (For your real input)
# 3. ./data/examples/day01.txt (For the example input)
jaoc start 1
```

### 4\. Write Your Solution

Open the generated file, `src/bin/day01.rs`. You will see two empty functions, `part1` and `part2`, ready for your logic.

```rust
// In src/bin/day01.rs
use aoc_2025::aoc_main; // This name comes from your Cargo.toml

// --- Part 1 ---
fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Your logic
    Ok(input.lines().count().to_string())
}

// --- Part 2 ---
fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Your (maybe different) logic
    Ok("Part 2 not done".to_string())
}

// The macro handles all arg parsing, file reading, and printing
aoc_main!(1, part1, part2);
```

### 5\. Run Your Code

Use `cargo run --bin <day_name>` to run your solution.

```bash
# Run Part 1 with the example input
# (Paste the example from the AoC website into data/examples/day01.txt)
cargo run --bin day01 -- 1 --example

# Run Part 1 with the real input
# (Paste your puzzle input into data/inputs/day01.txt)
cargo run --bin day01 -- 1

# Run Part 2 with the real input
cargo run --bin day01 -- 2
```