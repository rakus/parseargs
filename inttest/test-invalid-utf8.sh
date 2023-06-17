#!/bin/bash
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

test_pa_code 'exit 1;' -o "n:name=name" -- "$(printf '\xc3\x28')"
test_pa_code 'exit 1;' -o "n:name=name" -- "$(printf '\xa0\xa1')"
test_pa_code 'exit 1;' -o "n:name=name" -- "$(printf '\xe2\x28\xa1')"
test_pa_code 'exit 1;' -o "n:name=name" -- "$(printf '\xe2\x82\x28')"
test_pa_code 'exit 1;' -o "n:name=name" -- "$(printf '\xf0\x28\x8c\xbc')"
test_pa_code 'exit 1;' -o "n:name=name" -- "$(printf '\xf0\x90\x28\xbc')"
test_pa_code 'exit 1;' -o "n:name=name" -- "$(printf '\xf0\x28\x8c\x28')"

test_pa_code 'exit 1;' -n "X$(printf '\xc3\x28')Y" -o "n:name=name" --
test_pa_code 'exit 1;' -a "X$(printf '\xc3\x28')Y" -o "n:name=name" --
test_pa_code 'exit 1;' -e "X$(printf '\xc3\x28')Y" -o "n:name=name" --
test_pa_code 'exit 1;' -r "X$(printf '\xc3\x28')Y" -o "n:name=name" --
test_pa_code 'exit 1;' -s "X$(printf '\xc3\x28')Y" -o "n:name=name" --
test_pa_code 'exit 1;' -o "n:name=n$(printf '\xc3\x28')ame" --

end_test
