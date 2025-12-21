#!/bin/bash
#
# Example that uses most of the features of parseargs
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
    echo " -d, --debug                enable debug via call to function set_debug"
    echo " -o FILE, --out-file FILE   sets a argument via a call to the function set_out_file"
    echo " -c, --copy                 set mode to copy"
    echo " -m, --mode                 set mode to move"
    echo " FILE...                    call to arg_cb"

    echo ""
    echo "Either --copy or --move required."
    echo ""
    echo "In this script parseargs mainly uses callbacks to shell functions to set"
    echo "options and parameter. Additional on error the function error_cb is called."
    echo ""

}

# Called when '-d' or '--debug' is given.
# $1 is either 'true' or ''
set_debug()
{
    if [ -n "$1" ]; then
        echo "Debug enabled"
    else
        echo "Debug disbled"
    fi

    debug=$1
}

# Called fo '-o' and '--out-file'
# $1 is the option argument
set_out_file()
{
    echo "Output file: '$1'"
    out_file=$1
}

# Called for each program argument
arg_cb()
{
    echo "Argument: '$1'"
}

# Called when a error was detected. Eg. a unknown option
err_cb()
{
    echo "You did something wrong"
    return 27
}

mode=
eval "$(parseargs -n "$script_name" -sbash -h -r remainder -a arg_cb -e err_cb -o 'd:debug#set_debug(),o:out-file=set_out_file(),c:copy#*mode=copy,m:move#mode=move' -- "$@")"

echo "Mode:          $mode"
echo "Output File:   $out_file"
echo "Debug enabled: $debug"
echo "REMAINDER:     ${remainder[*]:-<EMPTY>}"
