
# Parseargs

**A command line option parser for shell scripts.**

---
:warning: This is in an early state of
development and not suitable for serious use.

Have a look at section TODO below.

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
#!/bin/bash
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

## Option definition

Work in progress...

The supported options are defined with the parseargs option
`-o`/`--options`. Multiple option definitions can be given
separated by commas. Using `-o ''` means, that the script
doesn't support any options.

Single Option Definition:

```
<OPTIONS><TYPE><ATTRIBUTE><TARGET>[=ASSIGN]
```
`OPTIONS` is a colon-separated list of option characters and strings
For single character options (like 'l' for '-l') alphanumeric chars
are supported.
String options (like 'long' for '--long') must start with a alphanumeric
followed by alphanumeric, '_' or '-'.

`TYPE` is either
* `#` for flags (boolean options)
* `=` for options that require an additional argument
* `+` for counting options (counts the occurrences on the command line)

'ATTRIBUTE' is either a '*' to mark an option as required or
a '?' to mark it as a help option. **'?' NOT IMPLEMENTED YET**

`TARGET` is either a plain variable name or if followed by `()` the
name of a function to call.

`ASSIGN` is only possible with a flag option (`#`). If the option
is given, the value is assigned to the target. Multiple options with the
same variable are possible in this case.

## TODO

This tool is in an early state and there are areas that need further
improvements

* Improve README.
* Consider writing a tutorial.
* Error messages and error handling.
* Improve parsing and parsing error messages.
* Gracefully handle arguments with invalid UTF-8 chars. Today it just error exits.
* Detect duplicate option definitions (e.g. 'd#debug,d#details').
* To many `clone()` calls -- most likely a general Rust newbie problem.

