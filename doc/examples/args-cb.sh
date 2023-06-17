#!/bin/sh

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
export PATH="$script_dir/../../target/debug:$script_dir/../../target/release:$PATH"

set_argument() { echo "set_argument($1)"; }

eval "$(parseargs -n args-cb.sh -a set_argument -o '' -- "$@")" || exit 1
