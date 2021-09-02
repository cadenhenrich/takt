# Takt
> A TUI Rust Kanban Board and Task Organizer :pencil:

## What is Takt?
Takt is a type of organizational utility called a [kanban
board](https://www.atlassian.com/agile/kanban). This means that Takt uses a
board>list>tile structure to organize itself and your work, much like
Atlassian's tool [Trello](https://trello.com/). Takt is majorly inspired by the
way Trello does things, and aims to recreate that method of organization and
ease of use therein.

## Installation

### Prerequisites
+ Cargo / rustup / something else that can compile rust
+ An internet connection

### Building
1. Clone this repository
2. Enter the following command into your command line:
```bash
cd /path/to/github/repository/
cargo build --release
mkdir -p ~/.local/bin/
cp target/release/takt ~/.local/bin/takt
```
