#!/bin/sh

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
export PATH="$script_dir/../../target/debug:$script_dir/../../target/release:$PATH"

eval "$(parseargs -n required.sh -o 'o=*out_file' -- "$@")" || exit 1

echo "Output file: $out_file"

