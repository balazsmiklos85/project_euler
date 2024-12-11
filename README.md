# project_euler

My solutions to [Project Euler](https://projecteuler.net/) problems.

## Requirements

- Rust 1.83.0

## How to run?

The project is a workspace of multiple bin crates. Run `cargo run --bin problem<number>` to run a specific solution. The solutions are self-contained, they take no input, they work with the one data from the problem solution, and they produce the expected result.

Whenever there is an example case in the problem description, that case is added as a test. Run `cargo test` to run all tests.

