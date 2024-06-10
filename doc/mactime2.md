# Command-Line Help for `mactime2`

This document contains the help content for the `mactime2` command-line program.

**Command Overview:**

* [`mactime2`↴](#mactime2)

## `mactime2`

Replacement for `mactime`

**Usage:** `mactime2 [OPTIONS]`

IMPORTANT

Note that POSIX specifies that all UNIX timestamps are UTC timestamps. It is
up to you to ensure that the bodyfile only contains UNIX timestamps that
comply with the POSIX standard.

###### **Options:**

* `-b <INPUT_FILE>` — path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped)

  Default value: `-`
* `-F`, `--format <OUTPUT_FORMAT>` — output format, if not specified, default value is 'txt'

  Possible values: `csv`, `txt`, `json`, `elastic`

* `-d` — output as CSV instead of TXT. This is a conveniance option, which is identical to `--format=csv` and will be removed in a future release. If you specified `--format` and `-d`, the latter will be ignored

  Possible values: `true`, `false`

* `-j` — output as JSON instead of TXT. This is a conveniance option, which is identical to `--format=json` and will be removed in a future release. If you specified `--format` and `-j`, the latter will be ignored

  Possible values: `true`, `false`

* `-t`, `--to-timezone <DST_ZONE>` — name of offset of destination timezone (or 'list' to display all possible values

  Default value: `UTC`
* `--strict` — strict mode: do not only warn, but abort if an error occurs

  Possible values: `true`, `false`

* `-v`, `--verbose` — Increase logging verbosity
* `-q`, `--quiet` — Decrease logging verbosity



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

