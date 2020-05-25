# ⚖️ Mafia

[![Documentation](https://docs.rs/mafia/badge.svg)](https://docs.rs/mafia) [![Latest Version](https://img.shields.io/crates/v/mafia.svg)](https://crates.io/crates/mafia) [![Build Status](https://travis-ci.org/calder/mafia.svg?branch=master)](https://travis-ci.org/calder/mafia) [![Coverage Status](https://coveralls.io/repos/github/calder/mafia/badge.svg?branch=master)](https://coveralls.io/github/calder/mafia?branch=master)

**Rust implementation of the classic party game [Mafia](https://en.wikipedia.org/wiki/Mafia_(party_game))**



## Installation

[Install Rust](https://rustup.rs/), then:

```sh
cargo install mafia-bin
```



## Usage

Host a game:

```sh
# Create a basic game template:
mafia init

# Edit auth.ron and setup.ron to your heart's content.

# When you're ready, start the game:
mafia host
```

Join a game:

```sh
socat - TCP:<address>:<port>
Auth("<password>")
```



## Rules

### Players

Each player has a number of **attributes**. Attributes can be:
* **Stacked:** A Godfather is `[Member("Mafia"), Bulletproof, Appears(Good)]`.
* **Composed:** Stone is `Uses(1, Bulletproof)`. Doctors apply `Phases(1, Bulletproof)`.

#### Attributes

|    | Attribute | Description |
|----|-----------|-------------|
| ❌ | `Appears(Alignment)` | Shows up as `Alignment` to investigations. |
| ✔️ | `Dead` | Player is dead. |
| ✔️ | `Has(Action)` | Player can use `Action`. |
| ✔️ | `Member(Faction)` | Player belongs to `Faction`. |
| ✔️ | `Phases(N, Attribute)` | Attribute expires after `N` phases. |
| ❌ | `Uses(N, Attribute)` | Attribute expires after `N` uses. |

#### Actions

|    | Action | Description |
|----|--------|-------------|
| ✔️ | `Investigate(Player)` | Investigate a player's alignment. |
| ✔️ | `Kill(Player)` | Kill a player. |
| ✔️ | `Protect(Player)` | Protect a player from kills. |
| ❌ | `Recruit(Player)` | Recruit a player into your faction. |
| ❌ | `Roleblock(Player)` | Block a player from using abilities. |

#### Placeholders

|    | Action | Description |
|----|--------|-------------|
| ❌ | `$PLAYER` | Any player. |
| ❌ | `$OTHER_PLAYER` | Any player besides the player using the action. |

#### Core roles

|    | Role | Attributes |
|----|------|------------|
| ✔️ | Town member | `Member("Town")` |
| ✔️ | Mafia member | `Member("Mafia")` |
| ✔️ | Cop | `Has(Investigate("$PLAYER"))` |
| ✔️ | Doctor | `Has(Protect("$OTHER_PLAYER"))` |

#### Common roles

|    | Role | Attributes |
|----|------|------------|
| ❌ | Busdriver | `Has(Busdrive("$PLAYER", "$PLAYER"))` |
| ❌ | Cult member | `Member("Cult")` |
| ❌ | Roleblocker | `Has(Roleblock("$PLAYER"))` |
| ❌ | Stone | `Uses(1, Bulletproof)` |



### Factions

**Factions** are groups of players with a common objective. Each faction has:
* An objective.
* An list of faction abilities.
* An alignment: `Good`, `Neutral`, or `Evil`.
* Membership `Hidden` or `Visible` to other players in the faction.

#### Core factions

|    | Faction | Objective | Alignment | Abilities | Membership |
|----|---------|-----------|-----------|-----------|------------|
| ✔️ | Town | `Eliminate(Evil)` | `Good` | `[]` | `Hidden` |
| ✔️ | Mafia | `AchieveMajority` | `Evil` | `[Kill("$PLAYER")]` | `Visible` |

#### Common factions

|    | Faction | Objective | Alignment | Abilities | Membership |
|----|---------|-----------|-----------|-----------|------------|
| ❌ | Cult | `AchieveMajority` | `Evil` | `[]` | `Visible` |
| ✔️ | Mason | `Eliminate(Evil)` | `Good` | `[]` | `Visible` |
| ✔️ | Survivor | `Survive` | `Neutral` | `[]` | `Visible` |

#### Objectives

|    | Objective | Description |
|----|-----------|-------------|
| ✔️ | `Eliminate(Alignment)` | Eliminate all players of a given alignment. |
| ✔️ | `EliminateFaction(Faction)` | Eliminate all players of a given faction. |
| ✔️ | `Majority` | Outnumber all other surviving players. |
| ✔️ | `Survive` | Survive until the end of the game. |



## TODO

* [ ] Basic gameplay
    * [X] Test infrastructure
    * [X] Advance phases
    * [X] Objectives
    * [X] Town
    * [X] Faction actions
    * [X] Mafia
    * [X] Individual actions
    * [X] Cop
    * [X] Doctor
    * [X] Day
    * [X] Action amending
    * [X] Log visibility
    * [ ] Action validity checking
        * [X] Player has action
        * [X] Doctors can't protect themselves
        * [X] Faction action chain of command
        * [ ] Faction leader can only order faction members.
* [X] Basic server
    * [X] Auth
    * [X] Client input
    * [X] Persistence
    * [X] Liveness updates
    * [X] Log updates
    * [X] Log visibility

Pre-intro:
* [X] Allow "..." as a shortcut for "Use(...)"
* [ ] Time travel
* [ ] Documentation

Punted:
* [ ] Remove trailing commas and add spaces in server responses
* [ ] Vote count updates
* [ ] Better setup & auth ergonomics



## ❤️ Contributing

Contributions are very welcome! If you're looking for a place to start:
* Submit a pull request adding a new role idea to this README.
* Submit a pull request implementing a role idea on this README.

### Setup

```sh
# Install Rust.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Set up pre-commit hooks.
scripts/setup_git_hooks.sh
```

### Running tests

Run all tests:

```sh
scripts/test_all.sh
```

Run a specific test:

```sh
cd mafia
cargo test basic_game
```

Run tests with verbose logging:

```sh
cd mafia-bin
env RUST_LOG=debug cargo test
```



### Running games

Run the `mafia` binary:

```sh
cd mafia-bin
cargo run -- help
```



### Adding features

1.  Implement feature.

2.  Add a test:

    ```sh
    cd mafia
    cp -r test_basic_game test_foo
    rm test_foo/out.*

    # Edit these files to your liking:
    #   test_foo/in.actions.ron: Player actions throughout the game.
    #   test_foo/in.setup.ron:   Initial game setup.

    # Generate expected outputs.
    env REGENERATE_GOLDENFILES=1 cargo test foo

    # Inspect outputs:
    #   test_foo/out.*.*.ron:     Game state at the beginning of each phase.
    #   test_foo/out.*.*_log.ron: Events that occurred during each phase.
    ```

3.  If you're satisfied, commit your changes and send out a pull request!



### Code of conduct

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
