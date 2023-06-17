#!/bin/sh

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
export PATH="$script_dir/../../target/debug:$script_dir/../../target/release:$PATH"

eval "$(parseargs -n mode-switch.sh -o 'c:copy#mode=copy,m:move#mode=move' -- "$@")" || exit 1

echo "Mode: $mode"

