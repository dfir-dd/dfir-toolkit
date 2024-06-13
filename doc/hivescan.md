# Command-Line Help for `hivescan`

This document contains the help content for the `hivescan` command-line program.

**Command Overview:**

* [`hivescan`↴](#hivescan)

## `hivescan`

scans a registry hive file for deleted entries

**Usage:** `hivescan [OPTIONS] <HIVE_FILE>`

###### **Arguments:**

* `<HIVE_FILE>` — name of the file to scan

###### **Options:**

* `-L`, `--log <LOGFILES>` — transaction LOG file(s). This argument can be specified one or two times
* `-v`, `--verbose` — Increase logging verbosity
* `-q`, `--quiet` — Decrease logging verbosity
* `-b` — output as bodyfile format

  Possible values: `true`, `false`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

