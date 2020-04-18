#!/bin/bash -e

cd "$(dirname "$0")/.."

# Colors.
GREEN="$(tput setaf 2)"
YELLOW="$(tput setaf 3)"
RED="$(tput setaf 1)"
RESET="$(tput sgr0)"

# Check formatting.
function check_cargo_fmt {
    if ! cargo fmt -- --check; then
        printf "${YELLOW}HINT:${RESET} Run\n\n    scripts/format_all.sh\n\nbefore committing.\n"
        exit 1
    fi
}

# Check version numbers are in sync.
function get_version {
    sed -nr 's/version = "(.*)"/\1/p' "$1/Cargo.toml" | head -n1
}
function check_versions_match {
    VERSION1="$(get_version $1)"
    VERSION2="$(get_version $2)"
    if [[ "$VERSION1" != "$VERSION2" ]]; then
        echo "${RED}ERROR:${RESET} $1 version ($VERSION1) and $2 version ($VERSION2) don't match!\n"
        exit 1
    fi
}
check_versions_match mafia mafia-bin

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
    cargo run -- version
)

printf "\nTests ${GREEN}PASSED${RESET}.\n"