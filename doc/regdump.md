# Command-Line Help for `regdump`

This document contains the help content for the `regdump` command-line program.

**Command Overview:**

* [`regdump`↴](#regdump)

## `regdump`

parses registry hive files and prints a bodyfile

**Usage:** `regdump [OPTIONS] <HIVE_FILE>`

###### **Arguments:**

* `<HIVE_FILE>` — name of the file to dump

###### **Options:**

* `-L`, `--log <LOGFILES>` — transaction LOG file(s). This argument can be specified one or two times
* `-b`, `--bodyfile` — print as bodyfile format

  Possible values: `true`, `false`

* `-I`, `--ignore-base-block` — ignore the base block (e.g. if it was encrypted by some ransomware)

  Possible values: `true`, `false`

* `-T`, `--hide-timestamps` — hide timestamps, if output is in reg format

  Possible values: `true`, `false`

* `-v`, `--verbose` — Increase logging verbosity
* `-q`, `--quiet` — Decrease logging verbosity



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

