
# Playing with GitHub Actions using Parseargs

**A command line option parser for shell scripts.**

---
:warning: This is in an early state of
development and not suitable for serious use.

Have a look at section TODO below.

Blahhh

---

Parseargs parses given shell script parameters based on a given
option definition. Depending on the given options shell code is
generated that can be evaluated using the shells `eval` builtin.

## A Simple Example

The following is a fragment for a shell script that supports
the options
* `-l` & `--long` to produce some long, more detailed output
* `-o FILE` & `--out-file FILE` to set the file to write to

Additionally multiple input files can be given as arguments.

```bash
#!/bin/sh
script_name="$(basename "$0")"
eval "$(parseargs -n "$script_name" -o 'l:long#details,o:out-file=output' )"
if [ -n "$details"] ; then
    echo "Long detailed output requested"
fi
echo "Output file: '$output'"
echo "Arguments: $*"
```

Parseargs parses the given options and creates shell code to set
variables. It also prints error messages and exits the script on
unknown options.

To investigate the generated code just call parseargs from the
command line.

The [Tutorial](https://rakus.github.io/parseargs/) explains all features of
Parseargs.

## Building

Build is controlled by a Makefile.  Run `make help` to get a help on the
available targets.

First step is to install `cargo get` and `cargo generate-rpm`. Run `make setup`
for this.

If the man-page and the tutorial should be build (required for RPM),
Asciidoctor must be installed.  See [install
page](https://docs.asciidoctor.org/asciidoctor/latest/install/). For syntax
highlighting in the tutorial "Pygments" is used. See [this
page](https://docs.asciidoctor.org/asciidoctor/latest/syntax-highlighting/pygments/).


## TODO

This tool is in an early state and there are areas that need further
improvements

* Improve tutorial and man page
* Error messages and error handling.
* Gracefully handle arguments with invalid UTF-8 chars. Today it just error exits.
* To many `clone()` calls -- most likely a general Rust newbie problem.

