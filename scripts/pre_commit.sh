#!/bin/bash -e

cd "$(dirname "$0")/../.."

# Colors.
GREEN="$(tput setaf 2)"
YELLOW="$(tput setaf 3)"
RED="$(tput setaf 1)"
RESET="$(tput sgr0)"

# Check formatting.
function check_cargo_fmt {
    if ! cargo fmt -- --check; then
        CD_COMMAND="cd $PWD"
        printf "${YELLOW}HINT:${RESET} Run\n\n    ${CD_COMMAND} && cargo fmt\n\nbefore committing.\n"
        exit 1
    fi
}

# Test mafia.
(
    cd mafia
    check_cargo_fmt
    cargo test
)

# Test mafia-bin.
(
    cd mafia-bin
    check_cargo_fmt
    cargo run
)

printf "\nPresubmits ${GREEN}PASSED${RESET}.\n"
