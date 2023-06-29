#!/bin/bash
#
# Run test from this directory
#

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1


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
    unset __PARSEARGS_TEST_SHELL__

    TEST_SHELL="$1"

    if command -v "$TEST_SHELL" >/dev/null 2>&1; then
        __PARSEARGS_TEST_SHELL__="$2"

        echo
        echo "Testing with $TEST_SHELL (Mode: $__PARSEARGS_TEST_SHELL__)"
        echo "============================================================"

        export TEST_SHELL
        export __PARSEARGS_TEST_SHELL__

        run_tests
    fi
}


#---------[ MAIN ]-------------------------------------------------------------

test_with_shell bash bash
test_with_shell ksh ksh
test_with_shell zsh zsh
test_with_shell pdksh ksh
test_with_shell mksh ksh
test_with_shell dash sh

