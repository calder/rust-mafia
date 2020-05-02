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



## Data model



### Players

Instead of roles, player can have any number of attributes. Attributes can be:
* **Stacked:** A Godfather is `[Member("Mafia"), Bulletproof, Appears(Good)]`.
* **Modified:** Stone is `OneShot(Bulletproof)`. Doctors apply `OnePhase(Bulletproof)`.

Examples of traditional roles implemented via attributes:

#### Core roles

| Role | Attributes |
| ---- | ---------- |
| Mafia member | `Member("Mafia")` |
| Town member | `Member("Town")` |
| Cop | `Has(Investigate)` |
| Doctor | `Has(Protect)` |

#### Common roles

| Role | Attributes |
| ---- | ---------- |
| Cult member (aka Cultist) | `Member("Cult")` |
| Stone | `OneShot(Bulletproof)` |
| Double stone | `OneShot(OneShot(Bulletproof))` |

#### Attributes

| Attributes | Description |
| ---------- | ----------- |
| `Has(Ability)` | Player can use `Ability`. |
| `Dead` | Player is dead. |
| `Member(Faction)` | Player belongs to `Faction`. |

#### Abilities

| Ability | Description |
| ------- | ----------- |
| `Investigate` | Determine a player's alignment. |
| `Kill` | Kill a player. |
| `Protect` | Make a player temporarily immune to kills. |



### Factions

Factions are defined by:
* An alignment: `Good`, `Neutral`, or `Evil`.
* An objective.
* An optional list of faction abilities.
* Whether membership is `Hidden` or `Visible` to other players in the faction.

#### Core factions

| Faction | `objective` | `alignment` | `abilities` | `membership` |
|---------|-------------|-------------|-------------|--------------|
| Mafia | `AchieveMajority` | `Evil` | `[Kill]` | `Visible` |
| Town | `Eliminate(Evil)` | `Good` | None | `Hidden` |

#### Common factions

| Faction | `objective` | `alignment` | `abilities` | `membership` |
|---------|-------------|-------------|-------------|--------------|
| Survivor | `Survive` | `Good` | None | `Visible` |
| Mason | `Eliminate(Evil)` | `Good` | None | `Visible` |

#### Objectives

| Objective | Description |
| --------- | ----------- |
| `Eliminate(Alignment)` | Eliminate all players of a given alignment. |
| `EliminateFaction(Faction)` | Eliminate all players of a given faction. |
| `Majority` | Outnumber all other surviving players. |
| `Survive` | Survive until the end of the game. |

#### Objectives

| Objective | Description |
| --------- | ----------- |


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

Contributions are very welcome! See the [issue tracker](https://github.com/calder/rust-mafia/issues) if you're looking for a place to start.

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
