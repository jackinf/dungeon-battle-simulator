<div align="center">

# Dungeon Battle Simulator

### An ASCII dungeon simulator built with Rust and async concurrency

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-000000?style=for-the-badge&logo=rust&logoColor=white)
![Crossterm](https://img.shields.io/badge/Crossterm-000000?style=for-the-badge&logoColor=white)
[![Repo](https://img.shields.io/badge/GitHub-jackinf%2Fdungeon--battle--simulator-181717?style=for-the-badge&logo=github&logoColor=white)](https://github.com/jackinf/dungeon-battle-simulator)

</div>

## Overview

Dungeon Battle Simulator is a terminal-based ASCII dungeon game written in Rust. It renders a live grid where a player and randomly moving monsters share a dungeon, with an event log displayed beneath the map. The project is a study in async concurrency: it uses the Tokio runtime to drive independent tasks for monster movement, rendering, player input, and event processing, all coordinating over shared state guarded by async mutexes.

## Features

- Real-time ASCII grid rendered in the terminal via `crossterm`.
- Concurrent task model powered by Tokio: separate async tasks handle monster movement, map rendering, and keyboard input.
- Shared state coordinated with `Arc<Mutex<...>>` and an `mpsc` channel for event delivery.
- Arrow-key player movement with a quit key (`q`).
- Randomly spawned monsters with randomized health and attack stats.
- A random event system supporting trap and treasure encounters.
- A bounded event log that keeps the most recent in-game messages.

## Tech Stack

| Area | Technology |
| --- | --- |
| Language | Rust (edition 2021) |
| Async runtime | [Tokio](https://tokio.rs/) (full features) |
| Terminal UI | [crossterm](https://crates.io/crates/crossterm) |
| Randomness | [rand](https://crates.io/crates/rand) |

## Getting Started

### Prerequisites

- A recent [Rust toolchain](https://www.rust-lang.org/tools/install) (with Cargo).
- A terminal that supports raw-mode key events.

### Installation

```bash
git clone https://github.com/jackinf/dungeon-battle-simulator.git
cd dungeon-battle-simulator
cargo build
```

### Running

```bash
cargo run
```

Use the arrow keys to move the player around the dungeon, and press `q` to quit.

## Project Structure

```
dungeon-battle-simulator/
├── Cargo.toml          # Crate manifest and dependencies
└── src/
    ├── main.rs         # Entry point; sets up Tokio tasks and shared state
    ├── dungeon.rs      # Dungeon grid, rendering, input handling, game loop
    ├── entities.rs     # Player/monster entities and spawning logic
    ├── event.rs        # Random trap/treasure event generation
    └── map.txt         # Sample ASCII map layout
```
