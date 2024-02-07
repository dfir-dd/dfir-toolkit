# Command-Line Help for `ts2date`

This document contains the help content for the `ts2date` command-line program.

**Command Overview:**

* [`ts2date`↴](#ts2date)

## `ts2date`

replaces UNIX timestamps in a stream by a formatted date

**Usage:** `ts2date [OPTIONS] [INPUT_FILE] [OUTPUT_FILE]`

###### **Arguments:**

* `<INPUT_FILE>` — name of the file to read (default from stdin)

  Default value: `-`
* `<OUTPUT_FILE>` — name of the file to write (default to stdout)

  Default value: `-`

###### **Options:**

* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence
* `-f`, `--from-timezone <SRC_ZONE>` — name of offset of source timezone (or 'list' to display all possible values

  Default value: `UTC`
* `-t`, `--to-timezone <DST_ZONE>` — name of offset of destination timezone (or 'list' to display all possible values

  Default value: `UTC`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

