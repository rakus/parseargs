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
#test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "d#debug=" --

# Incomplete
# e.g.: test_pa_errmsg 1 "parseargs: Error parsing option definition" -o "c=mode=copy" --

end_test
