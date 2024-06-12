#!/bin/sh
#
# Test parseargs with single byte
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"
script_file="$script_dir/$script_name"

. "$script_dir/_test.shinc"


check_single_byte()
{
    # print the bytes for the UTF-8 Euro sign and check whether this results in
    # a single character
    x="$(printf  '\342\202\254' | sed "s/./X/g")"
    if [ ${#x} = 1 ]; then
        return 1
    else
        return 0
    fi
}

if ! check_single_byte; then
    # set ISO-8859-1 if available
    SB_LOCALE=$(locale -a | grep 'iso88591$' | head -n1)
    if [ -n "$SB_LOCALE" ]; then
        echo "Setting LC_ALL to $SB_LOCALE"
        export LC_ALL="$SB_LOCALE"
    fi

    # If still no single byte char set -> exit
    if ! check_single_byte; then
        echo "No ISO-8859-1 character encoding available ... skipping"
        exit 0
    fi
    luit -encoding 'ISO 8859-1' "$script_file"
    exit $?
fi

start_test

if [ -n "$IS_WINDOWS" ]; then
    skip_test "on Windows"
elif [ "$TEST_SHELL" = "yash" ]; then
    skip_test "shell Yash, can't handle this"
fi

test_pa 'test "${#1}" = 3' -o "n:name=name" -- "$(printf '\342\202\254')"

test_pa 'test "$1" = "Ä"' -o "n:name=name" -- Ä
test_pa 'test "$name" = "Ä"' -o "n:name=name" -- -n Ä
test_pa 'test "$name" = "Ä"' -o "n:name=name" -- -nÄ
test_pa 'test "$name" = "Ä"' -o "n:name=name" -- --name Ä
test_pa 'test "$name" = "Ä"' -o "n:name=name" -- --name=Ä

test_pa 'test "$1" = "Ä'\''Ö"' -o "n:name=name" -- Ä\'Ö
test_pa 'test "$name" = "Ä'\''Ö"' -o "n:name=name" -- -n Ä\'Ö
test_pa 'test "$name" = "Ä'\''Ö"' -o "n:name=name" -- -nÄ\'Ö
test_pa 'test "$name" = "Ä'\''Ö"' -o "n:name=name" -- --name Ä\'Ö
test_pa 'test "$name" = "Ä'\''Ö"' -o "n:name=name" -- --name=Ä\'Ö

end_test

# vim:fileencoding=latin1

