#!/bin/bash

eval "$(parseargs -n remainder.sh -r support -o '' -- "$@")" || exit 1

echo "Main:    $*"
echo "Support: ${support[@]}"
