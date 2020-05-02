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

Instead of roles, player can have any number of attributes. Attributes can be:
* **Stacked:** A Godfather is `[Member("Mafia"), Bulletproof, Appears(Good)]`.
* **Composed:** Stone is `OneShot(Bulletproof)`. Doctors apply `OnePhase(Bulletproof)`.

Examples of traditional roles implemented via attributes:

#### Core roles

|    | Role | Attributes |
|----|------|------------|
| ✔️ | Town member | `Member("Town")` |
| ✔️ | Mafia member | `Member("Mafia")` |
| ✔️ | Cop | `Has(Investigate)` |
| ✔️ | Doctor | `Has(Protect)` |

#### Common roles

|    | Role | Attributes |
|----|------|------------|
| ❌ | Cult member (aka Cultist) | `Member("Cult")` |
| ❌ | Roleblocker | `Has(Roleblock)` |
| ❌ | Stone | `OneShot(Bulletproof)` |

#### Attributes

|    | Attributes | Description |
|----|------------|-------------|
| ❌ | `Appears(Alignment)` | Shows up as `Alignment` to investigations. |
| ✔️ | `Dead` | Player is dead. |
| ✔️ | `Has(Ability)` | Player can use `Ability`. |
| ✔️ | `Member(Faction)` | Player belongs to `Faction`. |
| ✔️ | `OnePhase(Attribute)` | Attribute expires after one phase. |
| ❌ | `OneShot(Attribute)` | Attribute expires after one use. |

#### Abilities

|    | Ability | Description |
|----|---------|-------------|
| ✔️ | `Investigate` | Determine a player's alignment. |
| ✔️ | `Kill` | Kill a player. |
| ✔️ | `Protect` | Temporarily make a player immune to kills. |
| ❌ | `Roleblock` | Temporarily prevent a player from using actions. |



### Factions

Factions are defined by:
* An objective.
* An list of faction abilities.
* An alignment: `Good`, `Neutral`, or `Evil`.
* Whether membership is `Hidden` or `Visible` to other players in the faction.

#### Core factions

|    | Faction | Objective | Alignment | Abilities | Membership |
|----|---------|-----------|-----------|-----------|------------|
| ✔️ | Town | `Eliminate(Evil)` | `Good` | `[]` | `Hidden` |
| ✔️ | Mafia | `AchieveMajority` | `Evil` | `[Kill]` | `Visible` |

#### Common factions

|    | Faction | Objective | Alignment | Abilities | Membership |
|----|---------|-----------|-----------|-----------|------------|
| ✔️ | Survivor | `Survive` | `Good` | `[]` | `Visible` |
| ✔️ | Mason | `Eliminate(Evil)` | `Good` | `[]` | `Visible` |

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
        * [ ] Player has action
        * [ ] Target is alive
        * [ ] Doctors can't protect themselves
        * [ ] Faction action chain of command
* [ ] Basic server
    * [X] Auth
    * [ ] Client input
    * [ ] State updates
    * [ ] Log updates
    * [ ] Log visibility



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
