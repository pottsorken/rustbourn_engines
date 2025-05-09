# Rustbourn Engines 🚀 - Steel is the only law

[![Bevy](https://img.shields.io/badge/Bevy-0.15-blue)](https://bevyengine.org)
[![SpacetimeDB](https://img.shields.io/badge/SpacetimeDB-1.0.1-orange)](https://spacetimedb.com)

A 2D multiplayer base building game built with **Rust**, **Bevy**, and **SpacetimeDB**.

![logo](https://github.com/user-attachments/assets/a63a40e5-a345-4a69-b4d4-c90d0ea966a6)
![1746716967 263503](https://github.com/user-attachments/assets/2c759417-3848-4545-9910-8723520ca22b)

## Features ✨

- 🚀 Real-time multiplayer gameplay
- ⚔️ City-to-city combat system
- 🏗️ Persistent world state with SpacetimeDB

## Tech Stack 🛠️

- **Game Engine**: [Bevy](https://bevyengine.org) (Rust)
- **Database**: [SpacetimeDB](https://spacetimedb.com/home)
- **Networking**: SpacetimeDB's native networking
- **Physics**: No physics

## Installation 📥

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- SpacetimeDB CLI (for local development)

### Running Locally

   ```bash
   git clone https://github.com/pottsorken/rustbourn_engines.git
   cd rustbourn_engines

   #Start the SpacetimeDB server (in a separate terminal):
   spacetime start
   spacetime publish --project-path server <server-name-here>

   #Run the game client:
   cd client
   cargo run -- --ip 127.0.0.1 --port 3000 --clear
   ```
# Command Line Options
Option|	Description	|                            Default
--- | --- | ---
-i	   | Server host IP address	   |             127.0.0.1
-p	   | Server port number |	                    3000
-c    |  Clear spacetime authentication token  |  false

# Useful Commands

   ```bash
# Build for release:
cargo build --release
   ```

# Contributing 🤝

Do not contribute


# License 📜

This project has no license yet. Please ask for permission for any usage.
