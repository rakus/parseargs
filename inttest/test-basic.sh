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

test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -a
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -b
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -c
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -d
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -e
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -f
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -g
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -h
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -i
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -j
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -k
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -l
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -m
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -n
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -o
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -p
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -q
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -r
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -s
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -t
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -u
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -v
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -w
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -x
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -y
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -z
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -0
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -1
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -2
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -3
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -4
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -5
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -6
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -7
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -8
test_pa 'test "$opt" = true' -o "a:b:c:d:e:f:g:h:i:j:k:l:m:n:o:p:q:r:s:t:u:v:w:x:y:z:0:1:2:3:4:5:6:7:8:9#opt" -- -9


end_test

