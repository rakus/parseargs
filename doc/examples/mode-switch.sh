#!/bin/sh

eval "$(parseargs -n mode-switch.sh -o 'c:copy#mode=copy,m:move#mode=move' -- "$@")" || exit 1

echo "Mode: $mode"

