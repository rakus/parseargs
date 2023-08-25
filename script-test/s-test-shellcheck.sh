#!/bin/sh
#
# Check parseargs output with shellcheck
#
# No check with ZSH, as this is not supported by shellcheck.
# Test with DASH, only when supported by shellcheck.
#

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

# shellcheck source=_test.shinc
. "${script_dir}/_test.shinc"

start_test

# check whether shellcheck is available at all
if ! command -v shellcheck >/dev/null; then
    echo "Skipped as shellcheck is not available"
    exit 0
fi

# check whether shellchek does support the dash shell
sc_support_dash=true
if ! echo "echo" | shellcheck -s dash - >/dev/null 2>&1; then
    printf "\033[01;33mWARNING: shellcheck does not support 'dash' -- dash will not be tested.\033[0m\n"
    sc_support_dash=
fi

# Run shellcheck on generated code
# Ignore "SC2034 - ... appears unused"
# $1: the shell  if the shell param to shellcheck should be different than the
#     shell use: 'shell/shell-for-shellcheck' (e.g. 'sh/dash', generates code
#     for 'sh' and shellcheck checks with the 'dash' rules)
# $*: parseargs options/arguments
sc_shell_code()
{
    shell="${1%%/*}"
    sc_shell="${1##*/}"
    shift
    if [ -z "$sc_support_dash" ] && [ "$sc_shell" = "dash" ]; then
        return
    fi
    if parseargs -s "$shell" "$@" | shellcheck -s"$sc_shell" -fgcc -eSC2034 -; then
        ok "shellcheck($sc_shell): parseargs -s $shell $*"
    else
        failed "shellcheck($sc_shell): parseargs -s $shell $*"
    fi
}

# check with bash, ksh and sh
# Also test sh output with bash, ksh and dash shellcheck rules
check_shells()
{
    for shell in bash ksh sh sh/bash sh/ksh sh/dash; do
        sc_shell_code $shell "$@"
    done
}

# check arrays with bash and ksh, but NOT sh or dash
check_shell_array()
{
    for shell in bash ksh; do
        sc_shell_code $shell "$@"
    done
}

check_shells --
check_shells -- in1.txt in2.txt
check_shells -- in1.txt -- in2.txt
check_shells -- in1.txt -- -x

check_shells -o 'd#debug,f=file,v+verbose' --
check_shells -o 'd#debug,f=file,v+verbose' -- in1.txt in2.txt
check_shells -o 'd#debug,f=file,v+verbose' -- in1.txt -- in2.txt
check_shells -o 'd#debug,f=file,v+verbose' -- in1.txt -- -x

check_shells -o 'd#debug,f=file,v+verbose' -- -d -f file -vvv in1.txt in2.txt
check_shells -io 'd#debug,f=file,v+verbose' -- -d -f file -vvv in1.txt in2.txt

check_shells -a arg_cb -hvo 'd#debug(),f=file(),v+verbose()' -- -v -d -v -f file -vvv in1.txt in2.txt

check_shells -o 'c#mode=copy,m#mode=move,d#mode=drop_it' -- -c
check_shells -o 'c#mode=copy,m#mode=move,d#mode=drop_it' -- -m
check_shells -o 'c#mode=copy,m#mode=move,d#mode=drop_it' -- -d
check_shells -o 'c#mode=copy,n#mode=' -- -n

check_shell_array -r team -o 'd#debug,f=file,v+verbose' -- Kirk -- Spock Bones
check_shell_array -ir team -o 'd#debug,f=file,v+verbose' -- Kirk -- Spock Bones

end_test
