#!/bin/sh

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
export PATH="$script_dir/../../target/debug:$script_dir/../../target/release:$PATH"

error_callback() { echo "You did something stupid!"; }

eval "$(parseargs -n error-cb.sh -e error_callback -o '' -- "$@")" || exit 1

echo "OK"

