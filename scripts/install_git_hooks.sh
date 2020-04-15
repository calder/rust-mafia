#!/bin/bash -e

cd "$(dirname "$0")/.."

# Colors
RED="$(tput setaf 1)"
GREEN="$(tput setaf 2)"
RESET="$(tput sgr0)"

mv ".git/hooks" ".git/hooks.bac$(ls -d .git/hooks.bac* 2> /dev/null | wc -l | tr -d ' ')"
ln -s "../scripts/git_hooks" ".git/hooks"

printf "Git hooks ${GREEN}INSTALLED${RESET}.\n"
