#!/bin/sh
#
# Test with invalid UTF-8 arguments.
#
# As of today, parseargs can't work with invalid UTF-8, so it should detect
# that and error out.
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test


if [ -n "$IS_MSYS" ] || [ -n "$IS_CYGWIN" ]; then
    echo "Skipped on Windows"
elif [ "$TEST_SHELL" = "yash" ]; then
    echo "Skipped with shell Yash, as it can't handle invalid UTF-8"
else


    inv_1="$(printf '\303\050')"
    inv_2="$(printf '\240\241')"
    inv_3="$(printf '\342\050\241')"
    inv_4="$(printf '\342\202\050')"
    inv_5="$(printf '\360\050\214\274')"
    inv_6="$(printf '\360\220\050\274')"
    inv_7="$(printf '\360\050\214\050')"

    test_pa 'test "$1" = "$inv_1"' -o "n:name=name" -- "$inv_1"
    test_pa 'test "$1" = "$inv_2"' -o "n:name=name" -- "$inv_2"
    test_pa 'test "$1" = "$inv_3"' -o "n:name=name" -- "$inv_3"
    test_pa 'test "$1" = "$inv_4"' -o "n:name=name" -- "$inv_4"
    test_pa 'test "$1" = "$inv_5"' -o "n:name=name" -- "$inv_5"
    test_pa 'test "$1" = "$inv_6"' -o "n:name=name" -- "$inv_6"
    test_pa 'test "$1" = "$inv_7"' -o "n:name=name" -- "$inv_7"

    test_pa 'test "$name" = "$inv_1"' -o "n:name=name" -- -n "$inv_1"
    test_pa 'test "$name" = "$inv_2"' -o "n:name=name" -- -n "$inv_2"
    test_pa 'test "$name" = "$inv_3"' -o "n:name=name" -- -n "$inv_3"
    test_pa 'test "$name" = "$inv_4"' -o "n:name=name" -- -n "$inv_4"
    test_pa 'test "$name" = "$inv_5"' -o "n:name=name" -- -n "$inv_5"
    test_pa 'test "$name" = "$inv_6"' -o "n:name=name" -- -n "$inv_6"
    test_pa 'test "$name" = "$inv_7"' -o "n:name=name" -- -n "$inv_7"

    test_pa 'test "$name" = "$inv_1"' -o "n:name=name" -- -n"$inv_1"
    test_pa 'test "$name" = "$inv_2"' -o "n:name=name" -- -n"$inv_2"
    test_pa 'test "$name" = "$inv_3"' -o "n:name=name" -- -n"$inv_3"
    test_pa 'test "$name" = "$inv_4"' -o "n:name=name" -- -n"$inv_4"
    test_pa 'test "$name" = "$inv_5"' -o "n:name=name" -- -n"$inv_5"
    test_pa 'test "$name" = "$inv_6"' -o "n:name=name" -- -n"$inv_6"
    test_pa 'test "$name" = "$inv_7"' -o "n:name=name" -- -n"$inv_7"


    test_pa 'test "$name" = "$inv_1"' -o "n:name=name" -- --name "$inv_1"
    test_pa 'test "$name" = "$inv_2"' -o "n:name=name" -- --name "$inv_2"
    test_pa 'test "$name" = "$inv_3"' -o "n:name=name" -- --name "$inv_3"
    test_pa 'test "$name" = "$inv_4"' -o "n:name=name" -- --name "$inv_4"
    test_pa 'test "$name" = "$inv_5"' -o "n:name=name" -- --name "$inv_5"
    test_pa 'test "$name" = "$inv_6"' -o "n:name=name" -- --name "$inv_6"
    test_pa 'test "$name" = "$inv_7"' -o "n:name=name" -- --name "$inv_7"

    test_pa 'test "$name" = "$inv_1"' -o "n:name=name" -- --name="$inv_1"
    test_pa 'test "$name" = "$inv_2"' -o "n:name=name" -- --name="$inv_2"
    test_pa 'test "$name" = "$inv_3"' -o "n:name=name" -- --name="$inv_3"
    test_pa 'test "$name" = "$inv_4"' -o "n:name=name" -- --name="$inv_4"
    test_pa 'test "$name" = "$inv_5"' -o "n:name=name" -- --name="$inv_5"
    test_pa 'test "$name" = "$inv_6"' -o "n:name=name" -- --name="$inv_6"
    test_pa 'test "$name" = "$inv_7"' -o "n:name=name" -- --name="$inv_7"


    # The following will always result in an error exit
    # The parseargs arguments must always be valid UTF-8.
    test_pa_code 'exit 1' -n "X$(printf '\303\050')Y" -o "n:name=name" --
    test_pa_code 'exit 1' -a "X$(printf '\303\050')Y" -o "n:name=name" --
    test_pa_code 'exit 1' -e "X$(printf '\303\050')Y" -o "n:name=name" --
    test_pa_code 'exit 1' -r "X$(printf '\303\050')Y" -o "n:name=name" --
    test_pa_code 'exit 1' -s "X$(printf '\303\050')Y" -o "n:name=name" --
    test_pa_code 'exit 1' -o "n:name=n$(printf '\303\050')ame" --

fi

end_test
