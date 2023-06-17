#!/bin/bash
#
#

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

export PATH="$script_dir/../target/debug:$script_dir/../target/release:$PATH"
#echo "Calling $(command -v parseargs)"

# Called when '--help' is given. Support for this is enabled in parseargs
# with the option '-H'
show_help()
{
    echo "$script_name [OPTIONS] [FILE...]"
    echo " -l, --long                 detailed output"
    echo " -o FILE, --out-file FILE   write to given file"
    echo " -v                         increases verbosity"
    echo " FILE...                    files to process"
}

eval "$(parseargs -n "$script_name" -H -o 'l:long#detailed,o:out-file=out_file,v+verbosity' -- "$@")"

echo "Detailed:   '$detailed'"
echo "Out-File:   '$out_file'"
echo "Verbosity:  $verbosity"
echo "Args:       $(printf "'%s' " "$@")"


