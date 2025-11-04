# 🚀 jaoc - Jeremy's Advent of Code Scaffolder

`jaoc` is a small command-line tool to quickly set up and manage a multi-bin
Rust project for solving [Advent of Code](https://adventofcode.com/) puzzles.

It's designed to get you from no code to solving Day 1, letting you focus on
the puzzle logic instead of boilerplate.

## Features

* **Fast Compile Times:** Generates
  a [multi-bin](https://www.google.com/search?q=https://doc.rust-lang.org/cargo/reference/cargo-targets.html%23binaries)
  project structure. Each day is its own binary, so `cargo run --bin day05` only
  compiles Day 5.
* **Zero Boilerplate:** The `jaoc start` command generates a template with a
  simple `aoc_main!` macro. You just write your `part1` and `part2` functions.
* **Full Workflow Automation:** A `jaoc next` command scaffolds the next day,
  downloads the input, and updates your config in one go.
* **Built-in Runner:** A simple `jaoc run <day>` command replaces the long
  `cargo run --bin...`.
* **Live Reloading:** `jaoc watch <day>` automatically re-runs your code every
  time you save.
* **Automatic Downloading:** `jaoc download <day>` fetches your personal puzzle
  input from AoC (requires session token).
* **Stateful Config:** `jaoc new` creates a `.jaoc.toml` to track your year and
  progress.

## Prerequisites

Before you can use `jaoc`, you'll need:

1. **Rust & Cargo:** (Install via [rustup](https://rustup.rs/))
2. **`cargo-generate`:** This is used by the `new` command.
   ```bash
   cargo install cargo-generate
   ```

## Installation

You can install `jaoc` directly from crates.io

```bash
cargo install jaoc
```

Or, you can download it from git

```bash
cargo install --git https://github.com/jviguy/jaoc.git.
```

-----

## The `jaoc` Workflow 🤩

This is the ideal workflow for using the tool.

### 1\. Create a New Project

Run `jaoc new` with your desired project name and the year.

```bash
# Usage: jaoc new <project-name> <year>
jaoc new my-aoc-2025 2025
```

This creates a new directory `my-aoc-2025`, initializes `day01`, and creates a
`.jaoc.toml` file to track your progress.

### 2\. Enter the Project

All other commands must be run from *inside* the newly created project folder.

```bash
cd my-aoc-2025
```

### 3\. Add Your Session Cookie (One-Time Setup)

To use the automatic download feature, you need to tell `jaoc` your session
cookie.

1. Log in to the [Advent of Code website](https://adventofcode.com).

2. Open your browser's developer tools (F12).

3. Go to the `Application` (Chrome) or `Storage` (Firefox) tab.

4. Find the `session` cookie for `adventofcode.com`.

5. Create a file named `.env` in your project's root.

6. Add your cookie to it like this:

   ```text
   # In ./.env
   AOC_SESSION=your_long_session_cookie_value_here
   ```

- This isn't truly one time, these cookies are said to last a month so
  replace it if requests start to fail.

### 4\. Code, Run, and Watch

Start `jaoc watch` to automatically re-run your code on every save.

```bash
# Watch Day 1, Part 1, using the example input
jaoc watch 1 --part 1 --example
```

Now, open `src/bin/day01.rs` in your editor. Every time you save the file, your
terminal will re-compile and re-run your solution instantly.

### 5\. Run on the Real Input

Once your example works, run it on the real input (which `jaoc new` or
`jaoc download` already fetched for you).

```bash
# Run Part 1 on the real input
jaoc run 1 --part 1
```

### 6\. Move to the Next Day

When you're ready for the next puzzle, just run `jaoc next`.

```bash
jaoc next
```

This command automatically:

1. Reads your `.jaoc.toml` and sees you last finished Day 1.
2. Runs the scaffold command for Day 2 (creates `src/bin/day02.rs` and data
   files).
3. Runs the download command for Day 2 (fetches `data/inputs/day02.txt`).
4. Updates your `.jaoc.toml` to set `last_day = 2`.

Now you're ready to `jaoc watch 2 --example` and repeat the cycle\!

-----

## Command Reference

| Command         | Arguments                              | Description                                                       |
|:----------------|:---------------------------------------|:------------------------------------------------------------------|
| `jaoc new`      | `<name> <year>`                        | Creates a new project, scaffolds Day 1, and creates `.jaoc.toml`. |
| `jaoc next`     |                                        | Scaffolds and downloads the *next* day based on `.jaoc.toml`.     |
| `jaoc run`      | `<day>` `[--part <1 2>]` `[--example]` | Runs the specified day/part. A wrapper for `cargo run`.           |
| `jaoc watch`    | `<day>` `[--part <1 2>]` `[--example]` | Runs the specified day/part and re-runs on file changes.          |
| `jaoc start`    | `<day>`                                | Scaffolds and downloads for a given day.                          |
| `jaoc download` | `<day>`                                | **Manual:** Only downloads the input for a given day.             |
| `jaoc regen`    | `<year>` `<day>`                       | **Utility:** Creates a new `.jaoc.toml` if you deleted yours.     |

-----

## Solution File Example

Your `jaoc start` or `jaoc next` command generates a file like this. You just
fill in the logic.

```rust
// In src/bin/day01.rs
// The crate name "my_aoc_2025" matches your project's name
use my_aoc_2025::aoc_main;

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