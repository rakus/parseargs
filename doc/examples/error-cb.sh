#!/bin/sh

error_callback() { echo "You did something stupid!"; }

eval "$(parseargs -n error-cb.sh -e error_callback -o '' -- "$@")" || exit 1

echo "OK"

