# ⚖️ Mafia

[![Documentation](https://docs.rs/mafia/badge.svg)](https://docs.rs/mafia) [![Latest Version](https://img.shields.io/crates/v/mafia.svg)](https://crates.io/crates/mafia) [![Build Status](https://travis-ci.org/calder/mafia.svg?branch=master)](https://travis-ci.org/calder/mafia) [![Coverage Status](https://coveralls.io/repos/github/calder/mafia/badge.svg?branch=master)](https://coveralls.io/github/calder/mafia?branch=master)

**Rust implementation of the classic party game [Mafia](https://en.wikipedia.org/wiki/Mafia_(party_game)).**



## Installation

[Install Rust](https://rustup.rs/), then:

```sh
cargo install mafia-bin
```



## Usage

Host a game:

```sh
mafia host
```

Join a game:

```sh
telnet <address> <port>
Auth(<passcode>)
```



## Roles

Instead of first class roles, players have any number of attributes. Examples:

| Role | Attributes |
| ---- | ---------- |
| Mafia goon | `Member("Mafia") + Can(Vote)` |
| Townie | `Member("Town") + Can(Vote)` |
| Cop | `Member("Town") + Can(Vote) + Can(Investigate)` |
| Doctor | `Member("Town") + Can(Vote) + Can(Protect)` |



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
    * [ ] Log visibility
    * [ ] Action validity checking
        * [ ] Player has action
        * [ ] Target is alive
        * [ ] Doctors can't protect themselves
        * [ ] Faction action chain of command
* [ ] Basic server
    * [ ] State updates
    * [ ] Log updates
    * [ ] Auth
    * [ ] Log visibility
    * [ ] Client input



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



### Running games

Run the `mafia` binary:

```sh
cd mafia-bin
cargo run -- help
```



### Adding features

1. Implement feature.

2. Add a test:

```sh
cd mafia
cp -r test_basic_game test_foo
rm test_foo/out.*

# Edit these files to your heart's content:
#   test_foo/in.actions.ron: Player actions throughout the game.
#   test_foo/in.setup.ron:   Initial game setup.

# Generate expected outputs.
env REGENERATE_GOLDENFILES=1 cargo test foo

# Inspect outputs:
#   test_foo/out.*.*.ron:     Game state at the beginning of each phase.
#   test_foo/out.*.*_log.ron: Events up through the end of each phase.
```

3. If you're satisfied, commit your changes and send out a pull request!



### Code of conduct

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
