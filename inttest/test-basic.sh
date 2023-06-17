#!/bin/bash
#
# Test parseargs basic functionallities
#
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

test_pa 'test $debug = true' -o "d:debug#debug" -- -d
test_pa 'test $debug = true' -o "d:debug#debug" -- --debug
test_pa 'test $debug = true' -o "d:debug#debug" -- --debug=true
test_pa 'test $debug = true' -o "d:debug#debug" -- --debug=True
test_pa 'test $debug = true' -o "d:debug#debug" -- --debug=TRUE
test_pa 'test $debug = true' -o "d:debug#debug" -- --debug=yes
test_pa 'test $debug = true' -o "d:debug#debug" -- --debug=yeS
test_pa 'test $debug = true' -o "d:debug#debug" -- --debug=YES
test_pa 'test -z "$debug"' -o "d:debug#debug" -- --debug=false
test_pa 'test -z "$debug"' -o "d:debug#debug" -- --debug=würzelpfrümpf

test_pa 'test $file = text' -o "f:file=file" -- -f text
test_pa 'test $file = text' -o "f:file=file" -- -ftext
test_pa 'test $file = text' -o "f:file=file" -- --file text
test_pa 'test $file = text' -o "f:file=file" -- --file=text

test_pa 'test "$file" = "test file"' -o "f:file=file" -- -f 'test file'
test_pa 'test "$file" = "test file"' -o "f:file=file" -- -f'test file'
test_pa 'test "$file" = "test file"' -o "f:file=file" -- --file 'test file'
test_pa 'test "$file" = "test file"' -o "f:file=file" -- --file='test file'

test_pa 'test $verbosity -eq 0' -o "v:verbose+verbosity" --
test_pa 'test $verbosity -eq 1' -o "v:verbose+verbosity" -- -v
test_pa 'test $verbosity -eq 3' -o "v:verbose+verbosity" -- -vvv

test_pa 'test $verbosity -eq 1' -o "v:verbose+verbosity" -- --verbose
test_pa 'test $verbosity -eq 3' -o "v:verbose+verbosity" -- -vv --verbose
test_pa 'test $verbosity -eq 17' -o "v:verbose+verbosity" -- --verbose=17
test_pa 'test $verbosity -eq 1 && test $1 = 17' -o "v:verbose+verbosity" -- --verbose 17


expect='test $debug = true && test "$long" = true && test "$file" = filename'
test_pa "$expect" -o "d#debug,l#long,f=file" -- -d -l -f filename
test_pa "$expect" -o "d#debug,l#long,f=file" -- -dl -f filename
test_pa "$expect" -o "d#debug,l#long,f=file" -- -dlf filename
test_pa "$expect" -o "d#debug,l#long,f=file" -- -dlffilename

test_pa 'test "$mode" = copy' -o "c:copy#mode=copy,m:move#mode=move" -- -c
test_pa 'test "$mode" = copy' -o "c:copy#mode=copy,m:move#mode=move" -- --copy
test_pa 'test "$mode" = move' -o "c:copy#mode=copy,m:move#mode=move" -- -m
test_pa 'test "$mode" = move' -o "c:copy#mode=copy,m:move#mode=move" -- --move

test_pa_errmsg 1 "parseargs: Options are mutual exclusive: -c/--copy, -m/--move" -o "c:copy#mode=copy,m:move#mode=move" -- -cm

test_pa_errmsg 1 "parseargs: One of the following options is required: -c/--copy, -m/--move" -o "c:copy#*mode=copy,m:move#mode=move" --
test_pa_errmsg 1 "parseargs: One of the following options is required: -c/--copy, -m/--move" -o "c:copy#mode=copy,m:move#*mode=move" --

test_pa_errmsg 1 "parseargs: Unknown option: -D" -o "d#debug" -- -D

end_test

