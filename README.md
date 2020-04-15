# ⚖️ Mafia

[![Documentation](https://docs.rs/mafia/badge.svg)](https://docs.rs/mafia) [![Latest Version](https://img.shields.io/crates/v/mafia.svg)](https://crates.io/crates/mafia) [![Build Status](https://travis-ci.org/calder/mafia.svg?branch=master)](https://travis-ci.org/calder/mafia) [![Coverage Status](https://coveralls.io/repos/github/calder/mafia/badge.svg?branch=master)](https://coveralls.io/github/calder/mafia?branch=master)

**Rust implementation of the classic party game [Mafia](https://en.wikipedia.org/wiki/Mafia_(party_game)).**



## Installation

TODO



## Usage

TODO



## Contributing

Contributions are very welcome!

### Setup

```sh
# Install Rust.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install pre-commit hooks.
scripts/install_git_hooks.sh
```

### Running tests

To run all tests:

```sh
bazel test //...
```

To debug a specific test:

```sh
bazel test  --test_env=RUST_BACKTRACE=1 --test_output=streamed //src:lib_test
```

### Adding a new dependency

First, install [`cargo raze`](https://github.com/google/cargo-raze):

```sh
sudo apt install libssl-dev
cargo install cargo-raze
```

Then, add the dependency to `Cargo.tml` and run:
```sh
cd cargo
cargo raze
```

### Code of conduct

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
