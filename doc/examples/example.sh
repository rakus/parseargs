#!/bin/sh

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
export PATH="$script_dir/../../target/debug:$script_dir/../../target/release:$PATH"

eval "$(parseargs -n example.sh -o 'l#long_output,o=outfile' -- "$@")" || exit 1

if [ -n "$long_output" ]; then
    echo "Long output is enabled"
fi
echo "Output file: '$outfile'"
echo "Arguments: $*"
