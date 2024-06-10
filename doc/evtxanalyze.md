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

* `-v`, `--verbose` — Increase logging verbosity
* `-q`, `--quiet` — Decrease logging verbosity



## `evtxanalyze pstree`

generate a process tree

**Usage:** `evtxanalyze pstree [OPTIONS] <EVTX_FILE>`

###### **Arguments:**

* `<EVTX_FILE>` — Name of the evtx file to parse (should be the path to "Security.evtx")

###### **Options:**

* `-U`, `--username <USERNAME>` — display only processes of this user (case insensitive regex search)
* `-F`, `--format <FORMAT>` — output format

  Default value: `csv`

  Possible values: `json`, `markdown`, `csv`, `latex`, `dot`




## `evtxanalyze sessions`

display sessions

**Usage:** `evtxanalyze sessions [OPTIONS] <EVTX_FILES_DIR>`

###### **Arguments:**

* `<EVTX_FILES_DIR>` — Names of the evtx files directory to parse. Be aware that this tool assumes some file names. If you renamed the files, session analysis wil not work correctly

###### **Options:**

* `--include-anonymous` — include anonymous sessions

  Possible values: `true`, `false`




## `evtxanalyze session`

display one single session

**Usage:** `evtxanalyze session <EVTX_FILES_DIR> <SESSION_ID>`

###### **Arguments:**

* `<EVTX_FILES_DIR>` — Names of the evtx files directory to parse. Be aware that this tool assumes some file names. If you renamed the files, session analysis wil not work correctly
* `<SESSION_ID>` — Session ID



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

