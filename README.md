# Advent of Code Solutions

This repository contains my solutions to the [Advent of Code](https://adventofcode.com/) programming puzzles.

## Project Structure

- `src/` - Contains the Rust implementation of solutions
  - `main.rs` - Main entry point with a command-line interface for running solutions
  - `y2024/` - Solutions for year 2024, organized by day
- `input/` - Contains the puzzle input files
  - `dayXX.txt` - Real input for each day
  - `dayXX.example` - Example input from the problem descriptions
- `days/` - Additional solution files

## Implementation Details

The solutions are primarily implemented in Rust, with some days solved in Python. The repository uses a modular structure with separate files for each day's solution.

### Technologies Used

- **Rust** with the following dependencies:
  - `clap` - Command-line argument parsing with derive features
  - `ndarray` - N-dimensional arrays
  - `num` - Numeric types and mathematical operations
  - `regex` - Regular expressions
  - `rustc-hash` - High-performance hashing

## How to Run

To run a specific day's solution, use:

```bash
cargo run -- input/dayXX.txt
```

Replace `XX` with the day number (e.g., `01`, `02`, etc.).

To run with example input:

```bash
cargo run -- input/dayXX.example
```

## Implementation Approach

The main program automatically detects which day to run based on the input filename. The `Day` enum in `main.rs` maps day numbers to their respective solver functions.

## Solutions

Solutions are organized by day and include implementations for:

- Day 1-11 (Rust)
- Day 19 (Rust)
- Additional days (Python)

## License

This project is for personal learning and practice with the Advent of Code challenges.
