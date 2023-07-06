#!/bin/sh
#
# Test option, argument and error callbacks.
#
# SC2016: Expressions don't expand in single quotes -- needed for assertions for test_pa
# shellcheck disable=SC2016  # assertions for test_pa

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

set_debug()
{
    # shellcheck disable=SC2034
    debug=$1
}

set_file()
{
    # shellcheck disable=SC2034
    file=$1
}

set_verbosity()
{
    # shellcheck disable=SC2034
    verbosity=$1
}

set_argument()
{
    # shellcheck disable=SC2034
    argument="$argument:$1"
}

err_cb()
{
    echo "err_cb called"
}


test_pa 'test -z "$debug"' -o "d:debug#set_debug()" --
test_pa 'test $debug = true' -o "d:debug#set_debug()" -- -d
test_pa 'test $debug = true' -o "d:debug#set_debug()" -- --debug

# callback allows multiple usage of the option. First set debug than reset it
test_pa 'test -z "$debug"' -o "d:debug#set_debug()" -- -d --debug=false

test_pa 'test -z "$verbosity"' -o "v+set_verbosity()" --
test_pa 'test $verbosity = 1' -o "v+set_verbosity()" -- -v
test_pa 'test $verbosity = 3' -o "v+set_verbosity()" -- -vvv

test_pa 'test -z "$file"' -o "f=set_file()" --
test_pa 'test $file = "hallo.txt"' -o "f=set_file()" -- -f hallo.txt

# test argument callback
test_pa 'test $argument = ":one:two:three"' -a set_argument -- one two three

# test error callback
if (eval "$(parseargs -e err_cb -- --unknown 2>/dev/null)") | grep -q "err_cb called"; then
    ok "error callback called"
else
    failed "error callback NOT called"
fi

test_pa_errmsg 127 "ERROR: Function 'set_unknown' does not exist." -o "l:long#set_unknown()" --

end_test
