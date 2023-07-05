#!/bin/sh
#
# Test parseargs basic functionallities
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

if shell_supports_arrays; then
    if [ "$TEST_SHELL" != "zsh" ]; then
        test_pa 'test $1 = kirk -a "${team[0]}" = spock -a "${team[1]}" = bones' -r team -- kirk -- spock bones
    else
        test_pa 'test $1 = kirk -a "${team[1]}" = spock -a "${team[2]}" = bones' -r team -- kirk -- spock bones
    fi
else
    echo "Skipped '--remainder' tests: Not supported with $TEST_SHELL and/or mode $PARSEARGS_SHELL"
fi

end_test

