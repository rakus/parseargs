#!/bin/bash

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
export PATH="$script_dir/../../target/debug:$script_dir/../../target/release:$PATH"

eval "$(parseargs -n remainder.sh -r support -o '' -- "$@")" || exit 1

echo "Main:    $*"
echo "Support: ${support[@]}"
