#!/bin/sh

eval "$(parseargs -n example.sh -o 'l#long_output,o=outfile' -- "$@")" || exit 1

if [ -n "$long_output" ]; then
    echo "Long output is enabled"
fi
echo "Output file: '$outfile'"
echo "Arguments: $*"
