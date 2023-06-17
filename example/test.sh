#!/bin/bash
#
# FILE: test.sh
#
# ABSTRACT:
#
# AUTHOR: Ralf Schandl
#
# CREATED: 2023-06-14
#

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

export PATH="$script_dir/../target/debug:$script_dir/../target/release:$PATH"

show_help()
{
    echo "$script_name [OPTIONS] [FILE...]"
    echo " -l, --long                 detailed output"
    echo " -o FILE, --out-file FILE   write to given file"
}

echo "Calling $(command -v parseargs)"

details=
output=
cmd="$(parseargs -n "$script_name" -H -o 'l:long#details,o:out-file=output,0#nul_sep' -- "$@" )"
echo "---------"
echo "$cmd"
echo "---------"
eval "$cmd"
if [ -n "$details" ] ; then
    echo "Long detailed output requested"
fi
echo "Output file: '$output'"
echo "Nul sep: '$nul_sep'"
echo "Arguments: $*"

exit 38
