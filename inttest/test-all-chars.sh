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

echo "Normal chars"
test_pa 'test "$opt" = true' -o 'A#opt' -- -A
test_pa 'test "$opt" = true' -o 'B#opt' -- -B
test_pa 'test "$opt" = true' -o 'C#opt' -- -C
test_pa 'test "$opt" = true' -o 'D#opt' -- -D
test_pa 'test "$opt" = true' -o 'E#opt' -- -E
test_pa 'test "$opt" = true' -o 'F#opt' -- -F
test_pa 'test "$opt" = true' -o 'G#opt' -- -G
test_pa 'test "$opt" = true' -o 'H#opt' -- -H
test_pa 'test "$opt" = true' -o 'I#opt' -- -I
test_pa 'test "$opt" = true' -o 'J#opt' -- -J
test_pa 'test "$opt" = true' -o 'K#opt' -- -K
test_pa 'test "$opt" = true' -o 'L#opt' -- -L
test_pa 'test "$opt" = true' -o 'M#opt' -- -M
test_pa 'test "$opt" = true' -o 'N#opt' -- -N
test_pa 'test "$opt" = true' -o 'O#opt' -- -O
test_pa 'test "$opt" = true' -o 'P#opt' -- -P
test_pa 'test "$opt" = true' -o 'Q#opt' -- -Q
test_pa 'test "$opt" = true' -o 'R#opt' -- -R
test_pa 'test "$opt" = true' -o 'S#opt' -- -S
test_pa 'test "$opt" = true' -o 'T#opt' -- -T
test_pa 'test "$opt" = true' -o 'U#opt' -- -U
test_pa 'test "$opt" = true' -o 'V#opt' -- -V
test_pa 'test "$opt" = true' -o 'W#opt' -- -W
test_pa 'test "$opt" = true' -o 'X#opt' -- -X
test_pa 'test "$opt" = true' -o 'Y#opt' -- -Y
test_pa 'test "$opt" = true' -o 'Z#opt' -- -Z
test_pa 'test "$opt" = true' -o 'a#opt' -- -a
test_pa 'test "$opt" = true' -o 'b#opt' -- -b
test_pa 'test "$opt" = true' -o 'c#opt' -- -c
test_pa 'test "$opt" = true' -o 'd#opt' -- -d
test_pa 'test "$opt" = true' -o 'e#opt' -- -e
test_pa 'test "$opt" = true' -o 'f#opt' -- -f
test_pa 'test "$opt" = true' -o 'g#opt' -- -g
test_pa 'test "$opt" = true' -o 'h#opt' -- -h
test_pa 'test "$opt" = true' -o 'i#opt' -- -i
test_pa 'test "$opt" = true' -o 'j#opt' -- -j
test_pa 'test "$opt" = true' -o 'k#opt' -- -k
test_pa 'test "$opt" = true' -o 'l#opt' -- -l
test_pa 'test "$opt" = true' -o 'm#opt' -- -m
test_pa 'test "$opt" = true' -o 'n#opt' -- -n
test_pa 'test "$opt" = true' -o 'o#opt' -- -o
test_pa 'test "$opt" = true' -o 'p#opt' -- -p
test_pa 'test "$opt" = true' -o 'q#opt' -- -q
test_pa 'test "$opt" = true' -o 'r#opt' -- -r
test_pa 'test "$opt" = true' -o 's#opt' -- -s
test_pa 'test "$opt" = true' -o 't#opt' -- -t
test_pa 'test "$opt" = true' -o 'u#opt' -- -u
test_pa 'test "$opt" = true' -o 'v#opt' -- -v
test_pa 'test "$opt" = true' -o 'w#opt' -- -w
test_pa 'test "$opt" = true' -o 'x#opt' -- -x
test_pa 'test "$opt" = true' -o 'y#opt' -- -y
test_pa 'test "$opt" = true' -o 'z#opt' -- -z
test_pa 'test "$opt" = true' -o '0#opt' -- -0
test_pa 'test "$opt" = true' -o '1#opt' -- -1
test_pa 'test "$opt" = true' -o '2#opt' -- -2
test_pa 'test "$opt" = true' -o '3#opt' -- -3
test_pa 'test "$opt" = true' -o '4#opt' -- -4
test_pa 'test "$opt" = true' -o '5#opt' -- -5
test_pa 'test "$opt" = true' -o '6#opt' -- -6
test_pa 'test "$opt" = true' -o '7#opt' -- -7
test_pa 'test "$opt" = true' -o '8#opt' -- -8
test_pa 'test "$opt" = true' -o '9#opt' -- -9
test_pa 'test "$opt" = true' -o '!#opt' -- -!
test_pa 'test "$opt" = true' -o '$#opt' -- -$
test_pa 'test "$opt" = true' -o ',#opt' -- -,
test_pa 'test "$opt" = true' -o '.#opt' -- -.
test_pa 'test "$opt" = true' -o '/#opt' -- -/
test_pa 'test "$opt" = true' -o '@#opt' -- -@
test_pa 'test "$opt" = true' -o '[#opt' -- -\[   # zsh needs escape here
test_pa 'test "$opt" = true' -o ']#opt' -- -]
test_pa 'test "$opt" = true' -o '^#opt' -- -^
test_pa 'test "$opt" = true' -o '_#opt' -- -_
test_pa 'test "$opt" = true' -o '{#opt' -- -\{
test_pa 'test "$opt" = true' -o '}#opt' -- -\}
test_pa 'test "$opt" = true' -o '~#opt' -- -~

echo
echo "Chars needing backslash escape on command line"
test_pa 'test "$opt" = true' -o '"#opt' -- -\"
test_pa 'test "$opt" = true' -o '&#opt' -- -\&
test_pa 'test "$opt" = true' -o "'#opt" -- -\'
test_pa 'test "$opt" = true' -o '(#opt' -- -\(
test_pa 'test "$opt" = true' -o ')#opt' -- -\)
test_pa 'test "$opt" = true' -o '<#opt' -- -\<
test_pa 'test "$opt" = true' -o '>#opt' -- -\>
test_pa 'test "$opt" = true' -o '*#opt' -- -\*
test_pa 'test "$opt" = true' -o '?#opt' -- -\?
test_pa 'test "$opt" = true' -o '|#opt' -- -\|
test_pa 'test "$opt" = true' -o ';#opt' -- -\;

echo
echo "Chars needing backslash escape in definition"
test_pa 'test "$opt" = true' -o '\##opt' -- -#
test_pa 'test "$opt" = true' -o '\%#opt' -- -%
test_pa 'test "$opt" = true' -o '\+#opt' -- -+
test_pa 'test "$opt" = true' -o '\:#opt' -- -:
test_pa 'test "$opt" = true' -o '\=#opt' -- -=
test_pa 'test "$opt" = true' -o '\\#opt' -- -\\

end_test
