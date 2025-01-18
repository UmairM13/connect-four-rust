# Connect Four in Rust

A simple implementation of the Connect Four game using a Functional approach. Players can either play against each other or challenge an AI opponent. The game also includes a timed mode for player moves against the AI.

---

## Features

- **Player vs Player Mode**: Two players can play on the same machine.
- **Player vs AI Mode**: A player can compete against a randomly moving AI.
- **Timed Player vs AI Mode**: The player has 5 seconds to make a move, adding a time-based challenge when playing against the AI.
- **Terminal-based UI**: The game is played in the terminal, providing an interactive and responsive user interface.

---

## Requirements

To run this project, you'll need:

- Rust 1.50 or higher
- `cargo` (Rust's package manager and build system)

You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

---

## Setup and Installation

1. Clone the repository
2. Navigate into the project directory:
   ` cd connect-four-rust`
3. Build the project:
   ` cargo build --release`
4. Run the game:
   `cargo run`

---

### Gameplay

1. After starting the game, choose the desired game mode by entering the corresponding number (1, 2, or 3).
2. For Player vs Player, two players will alternately choose columns.
3. For Player vs AI, the player will input their column choice, and the AI will make its move automatically.
4. For Player vs Harder AI, the player must input their column within 5 seconds. If the player doesn’t input a valid column in time, the game ends.
5. Enter a column (1-7) to drop a piece in the corresponding column of the grid.
6. Enter 0 to exit the game at any time.

---

## Future improvements

- Enhance AI using advanced strategies like Minimax or Monte Carlo Tree Search.
- Implement a save/load feature for game state persistance.
- Add a web interface
