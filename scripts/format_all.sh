#!/bin/bash -e

cd "$(dirname "$0")/.."

# Colors.
GREEN="$(tput setaf 2)"
RESET="$(tput sgr0)"

for package in mafia mafia-bin; do
    (
        printf "Formatting $package... "
        cd $package
        cargo fmt
        echo "Done."
    )
done

echo "Formatting ${GREEN}SUCCEEDED${RESET}."
