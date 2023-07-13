#!/bin/sh

eval "$(parseargs -n verbosity.sh -o 'v:verbose+verbosity' -- "$@")" || exit 1

echo "Verbosity: $verbosity"

