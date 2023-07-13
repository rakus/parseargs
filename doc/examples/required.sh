#!/bin/sh

eval "$(parseargs -n required.sh -o 'o=*out_file' -- "$@")" || exit 1

echo "Output file: $out_file"

