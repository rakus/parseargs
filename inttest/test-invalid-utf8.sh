#!/bin/bash
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

test_pa_code 'exit 1' -o "n:name=name" -- "$(printf '\303\050')"
test_pa_code 'exit 1' -o "n:name=name" -- "$(printf '\240\241')"
test_pa_code 'exit 1' -o "n:name=name" -- "$(printf '\342\050\241')"
test_pa_code 'exit 1' -o "n:name=name" -- "$(printf '\342\202\050')"
test_pa_code 'exit 1' -o "n:name=name" -- "$(printf '\360\050\214\274')"
test_pa_code 'exit 1' -o "n:name=name" -- "$(printf '\360\220\050\274')"
test_pa_code 'exit 1' -o "n:name=name" -- "$(printf '\360\050\214\050')"

test_pa_code 'exit 1' -n "X$(printf '\303\050')Y" -o "n:name=name" --
test_pa_code 'exit 1' -a "X$(printf '\303\050')Y" -o "n:name=name" --
test_pa_code 'exit 1' -e "X$(printf '\303\050')Y" -o "n:name=name" --
test_pa_code 'exit 1' -r "X$(printf '\303\050')Y" -o "n:name=name" --
test_pa_code 'exit 1' -s "X$(printf '\303\050')Y" -o "n:name=name" --
test_pa_code 'exit 1' -o "n:name=n$(printf '\303\050')ame" --

end_test
