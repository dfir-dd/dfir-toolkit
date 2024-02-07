# Command-Line Help for `evtxscan`

This document contains the help content for the `evtxscan` command-line program.

**Command Overview:**

* [`evtxscan`↴](#evtxscan)

## `evtxscan`

Find time skews in an evtx file

**Usage:** `evtxscan [OPTIONS] <EVTX_FILE>`

###### **Arguments:**

* `<EVTX_FILE>` — name of the evtx file to scan

###### **Options:**

* `-S`, `--show-records` — display also the contents of the records befor and after a time skew
* `-N`, `--negative-tolerance <NEGATIVE_TOLERANCE>` — negative tolerance limit (in seconds): time skews to the past below this limit will be ignored

  Default value: `5`
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

