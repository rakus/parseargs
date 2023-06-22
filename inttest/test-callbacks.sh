#!/bin/sh
#
# FILE: test-callbacks.sh
#
# ABSTRACT:
#
# AUTHOR: Ralf Schandl
#
# CREATED: 2023-06-16
#

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

(

set_debug()
{
    debug=$1
}


eval "$($PA_EXE -s$PA_SHELL_OPT -o "d#set_debug()" -- -d)"
test -n "$debug"

) && ok '$PA_EXE -s$PA_SHELL_OPT -o "d#set_debug()" -- -d' || failed '$PA_EXE -s$PA_SHELL_OPT -o "d#set_debug()" -- -d'




end_test
