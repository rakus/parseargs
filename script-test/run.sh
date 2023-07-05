#!/bin/sh
#
# Run test from this directory
#

script_name="$(basename "$0")"
script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1

test_shells="bash ksh zsh pdksh mksh dash sh"

cd "$script_dir" || exit 1

run_tests()
{
    for tst in test-*.sh; do
        if ! $TEST_SHELL "$tst"; then
            exit 1
        fi
    done
}

test_with_shell()
{
    unset TEST_SHELL
    unset PARSEARGS_SHELL

    TEST_SHELL="$1"
    shift

    if command -v "$TEST_SHELL" >/dev/null 2>&1; then
        for ts in "$@"; do
            PARSEARGS_SHELL="$ts"

            echo
            echo "Testing with $TEST_SHELL (Mode: $PARSEARGS_SHELL)"
            echo "============================================================"

            export TEST_SHELL
            export PARSEARGS_SHELL

            run_tests
        done
    fi
}


get_supported_shell_dialects()
{
    bn_sh=$(basename "$1")

    case $bn_sh in
        bash*)
            echo "bash dash"
            ;;
        ksh*|mksh*)
            echo "ksh dash"
            ;;
        zsh*)
            echo "zsh dash"
            ;;
        dash*)
            echo "dash"
            ;;
        *)
            # Default: no native dialect
            ;;
    esac
}

usage()
{
    echo >&2 "$script_name [OPTIONS]"
    echo >&2 "    -q   Quick test. Only test with first found shell of"
    echo >&2 "         [$test_shells]."
    echo >&2 "    -s shell(s)"
    echo >&2 "         Run tests with given shell(s). Multiple shells can be given comma"
    echo >&2 "         separated."
}

#---------[ MAIN ]-------------------------------------------------------------

while getopts ":qs:" o "$@"; do
    case $o in
        q) quick=true
            ;;
        s) test_shells="$(echo "$OPTARG" | tr ',' ' ')"
            ;;
        *)
            usage
            exit 1
            ;;
    esac
done
shift $((OPTIND-1))

for sh in $test_shells; do
    if command -v "$sh" >/dev/null 2>&1; then
        shells_tested="$shells_tested $sh"

        native_dialect=$(get_supp_shell_dialects "$sh")
        if [ -n "$native_dialect" ]; then
            # shellcheck disable=SC2086 # native_dialect _should_ split
            test_with_shell "$sh" $native_dialect sh
        else
            test_with_shell "$sh" sh
        fi
        [ -n "$quick" ] && break
    else
        shells_not_found="$shells_not_found $sh"
    fi
done

echo "Tested shells:    $shells_tested"
if [ -n "$shells_not_found" ]; then
    echo "Shells not found: $shells_not_found"
fi
echo

