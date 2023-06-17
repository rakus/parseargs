#!/bin/sh

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
export PATH="$script_dir/../../target/debug:$script_dir/../../target/release:$PATH"

eval "$(parseargs -n verbosity.sh -o 'v:verbose+verbosity' -- "$@")" || exit 1

echo "Verbosity: $verbosity"

