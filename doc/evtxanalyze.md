# Command-Line Help for `evtxanalyze`

This document contains the help content for the `evtxanalyze` command-line program.

**Command Overview:**

* [`evtxanalyze`↴](#evtxanalyze)
* [`evtxanalyze pstree`↴](#evtxanalyze-pstree)
* [`evtxanalyze sessions`↴](#evtxanalyze-sessions)
* [`evtxanalyze session`↴](#evtxanalyze-session)

## `evtxanalyze`

crate provide functions to analyze evtx files

**Usage:** `evtxanalyze [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `pstree` — generate a process tree
* `sessions` — display sessions
* `session` — display one single session

###### **Options:**

* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



## `evtxanalyze pstree`

generate a process tree

**Usage:** `evtxanalyze pstree [OPTIONS] <EVTX_FILE>`

###### **Arguments:**

* `<EVTX_FILE>` — Name of the evtx file to parse

###### **Options:**

* `-U`, `--username <USERNAME>` — display only processes of this user (case insensitive regex search)
* `-F`, `--format <FORMAT>` — output format

  Default value: `csv`

  Possible values: `json`, `markdown`, `csv`, `latex`, `dot`




## `evtxanalyze sessions`

display sessions

**Usage:** `evtxanalyze sessions [OPTIONS] <EVTX_FILES_DIR>`

###### **Arguments:**

* `<EVTX_FILES_DIR>` — Names of the evtx files to parse

###### **Options:**

* `--include-anonymous` — include anonymous sessions



## `evtxanalyze session`

display one single session

**Usage:** `evtxanalyze session <EVTX_FILES_DIR> <SESSION_ID>`

###### **Arguments:**

* `<EVTX_FILES_DIR>` — Names of the evtx files to parse
* `<SESSION_ID>` — Session ID



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

