#!/bin/sh

set_argument() { echo "set_argument($1)"; }

eval "$(parseargs -n args-cb.sh -a set_argument -o '' -- "$@")" || exit 1
