#!/bin/sh

eval "$(parseargs -n long-opt.sh -o 'l:long#long_output,o:out-file=outfile' -- "$@")" || exit 1

if [ -n "$long_output" ]; then
    echo "Long output is enabled"
fi
echo "Output file: '$outfile'"
echo "Arguments: $*"


