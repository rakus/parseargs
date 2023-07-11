
# Shell Script Tests

This directory contains various scripts to test Parseargs in real live shell environments.

All scripts starting with `test-` are called with various shells to validate that the generate code is really correctly evaluated by that shell.

Scripts starting with `s-test-` are standalone programs and will either use different shells internally or just textual check Parseargs output.

## Execution

To run all tests just execute `./run.sh`.
This script will run the `test-*.sh` scripts with multiple shells.
Then it runs the scripts `s-test-*.sh`.

If installed the following shells are used for testing:

* bash
* ksh
* zsh
* mksh
* pdksh
* dash
* sh

