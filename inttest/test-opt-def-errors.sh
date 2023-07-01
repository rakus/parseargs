#!/bin/sh
#
# Test parseargs basic functionallities
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "ddebug" --

test_pa_errmsg 1 "parseargs: Error parsing option definition" -o ":#debug" --
test_pa_errmsg 1 "parseargs: Error parsing option definition" -o ":d#debug" --
test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "d#" --
test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "d#debug(" --
test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "d#debug)" --
test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "d#debug,d" --

test_pa_errmsg 1 "parseargs: Duplicate definition of option '-d'" -o "d#debug,d#dancing" --
test_pa_errmsg 1 "parseargs: Duplicate definition of option '--debug'" -o "debug#debug,debug#dancing" --
test_pa_errmsg 1 "parseargs: Duplicate usage of variable/function 'debug'" -o "d#debug,x#debug" --
test_pa_errmsg 1 "parseargs: Duplicate usage of variable/function 'debug'" -o "d#debug,x#debug()" --
test_pa_errmsg 1 "parseargs: Duplicate usage of variable/function 'mode'" -o "c#mode,m#mode=move" --
test_pa_errmsg 1 "parseargs: Duplicate usage of variable/function 'mode'" -o "c#mode=copy,m#mode" --
test_pa_errmsg 1 "parseargs: Duplicate value 'copy' for mode 'mode'" -o "c#mode=copy,m#mode=copy" --


#test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "d#debug=" --

# Incomplete
# e.g.: test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "c=mode=copy" --

end_test
