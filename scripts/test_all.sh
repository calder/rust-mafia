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

# Get the version of a subpackage.
function get_version {
    sed -nr 's/version = "(.*)"/\1/p' "$1/Cargo.toml" | head -n1
}

# Check that 2 subpackages versions are in sync.
function check_versions_match {
    VERSION1="$(get_version $1)"
    VERSION2="$(get_version $2)"
    if [[ "$VERSION1" != "$VERSION2" ]]; then
        echo "${RED}ERROR:${RESET} $1 version ($VERSION1) != $2 version ($VERSION2)"
        exit 1
    fi
}

# Run tests for a given crate.
function test_crate() {
    CRATE="$1"
    (
        cd "$CRATE"
        check_cargo_fmt
        cargo test
    )
}

# Check versions.
check_versions_match mafia mafia-bin

# Run tests.
test_crate mafia
test_crate mafia-bin

printf "\nTests ${GREEN}PASSED${RESET}.\n"
