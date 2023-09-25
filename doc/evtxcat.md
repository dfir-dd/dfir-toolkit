# Command-Line Help for `evtxcat`

This document contains the help content for the `evtxcat` command-line program.

**Command Overview:**

* [`evtxcat`↴](#evtxcat)

## `evtxcat`

Display one or more events from an evtx file

**Usage:** `evtxcat [OPTIONS] <EVTX_FILE>`

###### **Arguments:**

* `<EVTX_FILE>` — Name of the evtx file to read from

###### **Options:**

* `--min <MIN>` — filter: minimal event record identifier
* `--max <MAX>` — filter: maximal event record identifier
* `-i`, `--id <ID>` — show only the one event with this record identifier
* `-T`, `--display-table` — don't display the records in a table format
* `-F`, `--format <FORMAT>` — output format

  Default value: `xml`

  Possible values: `json`, `xml`

* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

