#!/bin/sh

set_long() { echo "set_long($1)"; }
set_outfile() { echo "set_outfile($1)"; }
set_verbosity() { echo "set_verbosity($1)"; }

eval "$(parseargs -n option-cb.sh -o 'l:long#set_long(),o=set_outfile(),v+set_verbosity()' -- "$@")" || exit 1

echo "Arguments: $*"
