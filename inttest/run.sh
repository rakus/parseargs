#!/bin/bash
#
# Run test from this directory
#

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1


cd "$script_dir" || exit 1

for tst in test-*.sh; do
    if ! ./"$tst"; then
        exit 1
    fi
done

