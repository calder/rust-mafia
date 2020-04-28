# ⚖️ Mafia

[![Documentation](https://docs.rs/mafia/badge.svg)](https://docs.rs/mafia) [![Latest Version](https://img.shields.io/crates/v/mafia.svg)](https://crates.io/crates/mafia) [![Build Status](https://travis-ci.org/calder/mafia.svg?branch=master)](https://travis-ci.org/calder/mafia) [![Coverage Status](https://coveralls.io/repos/github/calder/mafia/badge.svg?branch=master)](https://coveralls.io/github/calder/mafia?branch=master)

**Rust implementation of the classic party game [Mafia](https://en.wikipedia.org/wiki/Mafia_(party_game)).**



## Installation

TODO



## Usage

TODO



## TODO

[ ] Basic gameplay
    [X] Test infrastructure
    [X] Advance phases
    [X] Objectives
    [X] Town
    [X] Faction actions
    [X] Mafia
    [X] Individual actions
    [X] Cop
    [X] Doctor
    [ ] Day
    [X] Action amending
    [ ] Log visibility
    [ ] Action validity checking
        [ ] Player has action
        [ ] Target is alive
        [ ] Doctors can't protect themselves
        [ ] Faction action chain of command
[ ] Basic server
    [ ] State updates
    [ ] Log updates
    [ ] Auth
    [ ] Log visibility
    [ ] Client input



## ❤️ Contributing

Contributions are very welcome! See the [issue tracker](https://github.com/calder/rust-mafia/issues) if you're looking to get involved but don't know where to start.

### Setup

```sh
# Install Rust.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Set up pre-commit hooks.
scripts/setup_git_hooks.sh
```

### Running tests

To run all tests:

```sh
scripts/test_all.sh
```

To run a specific test:

```sh
cd mafia
cargo test player
```

### Code of conduct

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
