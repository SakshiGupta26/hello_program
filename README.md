🦀 Rust Mini Projects Collection

A collection of beginner-friendly Rust projects created while learning the Rust programming language.
Each folder contains an independent Rust project demonstrating different concepts like CLI tools, file handling, ownership, enums, pattern matching, and game logic.

📌 Projects Included
1. CLI_tool

A command-line utility built using Rust. Demonstrates:

Parsing user input

Basic CLI structure

Rust's standard library functions

2. Guess_word

A simple word-guessing game coded in Rust.
Covers:

Loops

String comparison

Handling user input with stdin

3. Tictac

A classic Tic-Tac-Toe game written in Rust.
Concepts used:

Structs

Game loops

2D board logic

Pattern checking (win/draw)

4. Todo_rust

A Todo List application with CSV file storage.
Implements:

serde for serialization

Reading/writing CSV files

File I/O

Struct-based task management (Task)

5. guessing_game

A number-guessing game (based on Rust official book).
Shows:

Random number generation

Parsing integers

Conditions and loops

6. src (Main Project)

This is the primary Rust project in the root folder.
Recent update includes changes in:

src/main.rs


Features:

Core Rust logic

Improvements based on version control tracking

📁 Repository Structure
hello_program/
│
├── CLI_tool/              # CLI application
├── Guess_word/            # Word guessing game
├── Tictac/                # Tic Tac Toe game
├── Todo_rust/             # Todo application (CSV based)
├── guessing_game/         # Number guessing game
├── src/                   # Root Rust project code
│   └── main.rs
│
├── Cargo.toml
├── Cargo.lock
└── .gitignore

🚀 How to Run Any Project

Navigate into a project folder:

cd Project_Name


Then run:

cargo run


Example:

cd Todo_rust
cargo run
📦 Requirements

Rust (latest stable version)
Install Rust using:

https://rustup.rs

🎯 Purpose of This Repository

This repo documents the journey of learning Rust step-by-step.
Each project focuses on mastering one or more concepts like:

Ownership & Borrowing

Structs & Enums

Traits

Iterators

File handling

Error handling

Game logic

CLI development

🤝 Contributions

This is a personal learning repository, but improvements and suggestions are welcome!
