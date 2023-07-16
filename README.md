
# Playing with GitHub Actions using Parseargs

**A command line option parser for shell scripts.**

---

:warning: This is in an early state of development.
Have a look at section [TODO](#todo) below.

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

To investigate the generated code just call Parseargs from the
command line.

The [Tutorial] explains all features of Parseargs.

## Building

Build is controlled by a Makefile.  Run `make help` to get a help on the
available targets.

### Prerequisites

Some additional tools are needed for building and testing Parseargs.

**Basics**

* cargo get - To extract info from `Cargo.toml` (`cargo install cargo-get`).
* ShellCheck - Linter for shell code. Used in tests (install using your package manager).

**Documentation**

* Asciidoctor - Used to build tutorial and man page. See [Asciidoctor install page].
* Pygments - Syntax highlighting of shell code in tutorial. See [Pygments install page].

**Package Build**

* cargo generate-rpm - Pure Rust RPM builder. `cargo install cargo-generate-rpm`
* cargo deb - Pure Rust Debian package builder. `cargo install cargo-deb`

## TODO

This tool is in an early state and there are areas that need further
improvements

* Improve error messages from option definition parser.
* Gracefully handle arguments with invalid UTF-8 chars. Today it just error exits.
* To many `clone()` calls -- most likely a general Rust newbie problem.


[Tutorial]: https://rakus.github.io/parseargs/
[Asciidoctor install page]: https://docs.asciidoctor.org/asciidoctor/latest/install/
[Pygments install page]: https://docs.asciidoctor.org/asciidoctor/latest/syntax-highlighting/pygments/
