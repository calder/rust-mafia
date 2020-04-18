#!/bin/bash -ex

cd "$(dirname "$0")/.."

# Colors.
RED="$(tput setaf 1)"
GREEN="$(tput setaf 2)"
RESET="$(tput sgr0)"

# Check formatting.
cargo fmt -- --check \
    || { printf "\n${RED}ERROR${RESET}: Run \`cargo fmt\` before committing.\n"; exit 1; }

# Make sure tests pass under both Bazel and Cargo.
bazel test //...
cargo test

# Make sure binary runs under both Bazel and Cargo.
bazel run //:mafia_bin
cargo run

printf "\nPresubmits ${GREEN}PASSED${RESET}.\n"
