
# DFIR Toolkit
[![Crates.io](https://img.shields.io/crates/v/dfir-toolkit)](https://crates.io/crates/dfir-toolkit)
[![Crates.io (latest)](https://img.shields.io/crates/dv/dfir-toolkit)](https://crates.io/crates/dfir-toolkit)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/dfir-dd/dfir-toolkit/cargo_test.yml)
[![Codecov](https://img.shields.io/codecov/c/github/dfir-dd/dfir-toolkit)](https://app.codecov.io/gh/dfir-dd/dfir-toolkit)


# Table of contents

- [Installation](#installation)
- [Overview of timelining tools](#overview-of-timelining-tools)
- [Tools](#tools)
  - [x] [`cleanhive`](#cleanhive)
  - [x] [`evtx2bodyfile`](#evtx2bodyfile)
  - [x] [`evtxanalyze`](#evtxanalyze)
  - [x] [`evtxscan`](#evtxscan)
  - [x] [`evtxcat`](#evtxcat)
  - [x] [`evtxls`](#evtxls)
  - [x] [`es4forensics`](#es4forensics)
  - [x] [`hivescan`](#hivescan)
  - [x] [`ipgrep`](#ipgrep)
  - [ ] [`lnk2bodyfile`](https://github.com/janstarke/lnk2bodyfile)
  - [x] [`mactime2`](#mactime2)
  - [ ] [`mft2bodyfile`](https://github.com/janstarke/mft2bodyfile)
  - [ ] [`ntdsextract2`](https://github.com/janstarke/ntdsextract2)
  - [x] [`pol_export`](#pol_export)
  - [ ] [`procbins`](https://github.com/janstarke/procbins)
  - [x] [`regdump`](#regdump)
  - [ ] [`regls`](https://github.com/janstarke/regls)
  - [ ] [`regview`](https://github.com/janstarke/regview)
  - [x] [`ts2date`](#ts2date)
  - [ ] [`usnjrnl_dump`](https://github.com/janstarke/usnjrnl)

# Overview of timelining tools

<img src="https://github.com/dfir-dd/dfir-toolkit/blob/master/doc/images/tools.svg?raw=true">

# Installation

```bash
cargo install dfir-toolkit
```

To generate autocompletion scripts for your shell, invoke the tool with the `--autocomplete` option, e.g.

```bash
mactime2 --autocomplete bash | sudo tee /etc/bash_completion.d/mactime2
```

would install a autocompletion script in `/etc/bash_completion.d/mactime2`.

# Tools
# Command-Line Help for `cleanhive`

This document contains the help content for the `cleanhive` command-line program.

**Command Overview:**

* [`cleanhive`↴](#cleanhive)

## `cleanhive`

merges logfiles into a hive file

**Usage:** `cleanhive [OPTIONS] <HIVE_FILE>`

###### **Arguments:**

* `<HIVE_FILE>` — name of the file to dump

###### **Options:**

* `-L`, `--log <LOGFILES>` — transaction LOG file(s). This argument can be specified one or two times
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence
* `-O`, `--output <DST_HIVE>` — name of the file to which the cleaned hive will be written

  Default value: `-`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

# Command-Line Help for `es4forensics`

This document contains the help content for the `es4forensics` command-line program.

**Command Overview:**

* [`es4forensics`↴](#es4forensics)
* [`es4forensics create-index`↴](#es4forensics-create-index)
* [`es4forensics import`↴](#es4forensics-import)

## `es4forensics`

This crates provides structs and functions to insert timeline data into an elasticsearch index

**Usage:** `es4forensics [OPTIONS] --index <INDEX_NAME> --password <PASSWORD> <COMMAND>`

###### **Subcommands:**

* `create-index` — 
* `import` — 

###### **Options:**

* `--strict` — strict mode: do not only warn, but abort if an error occurs
* `-I`, `--index <INDEX_NAME>` — name of the elasticsearch index
* `-H`, `--host <HOST>` — server name or IP address of elasticsearch server

  Default value: `localhost`
* `-P`, `--port <PORT>` — API port number of elasticsearch server

  Default value: `9200`
* `--proto <PROTOCOL>` — protocol to be used to connect to elasticsearch

  Default value: `https`

  Possible values: `http`, `https`

* `-k`, `--insecure` — omit certificate validation

  Default value: `false`
* `-U`, `--username <USERNAME>` — username for elasticsearch server

  Default value: `elastic`
* `-W`, `--password <PASSWORD>` — password for authenticating at elasticsearch
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



## `es4forensics create-index`

**Usage:** `es4forensics create-index`



## `es4forensics import`

**Usage:** `es4forensics import [OPTIONS] [INPUT_FILE]`

###### **Arguments:**

* `<INPUT_FILE>` — path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped)

  Default value: `-`

###### **Options:**

* `--bulk-size <BULK_SIZE>` — number of timeline entries to combine in one bulk operation

  Default value: `1000`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

# Command-Line Help for `evtx2bodyfile`

This document contains the help content for the `evtx2bodyfile` command-line program.

**Command Overview:**

* [`evtx2bodyfile`↴](#evtx2bodyfile)

## `evtx2bodyfile`

creates bodyfile from Windows evtx files

**Usage:** `evtx2bodyfile [OPTIONS] [EVTX_FILES]...`

###### **Arguments:**

* `<EVTX_FILES>` — names of the evtx files

###### **Options:**

* `-F`, `--format <FORMAT>` — select output format

  Default value: `bodyfile`

  Possible values: `json`, `bodyfile`

* `-S`, `--strict` — fail upon read error
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

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

# Command-Line Help for `evtxls`

This document contains the help content for the `evtxls` command-line program.

**Command Overview:**

* [`evtxls`↴](#evtxls)

## `evtxls`

Display one or more events from an evtx file

**Usage:** `evtxls [OPTIONS] [EVTX_FILES]...`

###### **Arguments:**

* `<EVTX_FILES>` — Name of the evtx files to read from

###### **Options:**

* `-d`, `--delimiter <DELIMITER>` — use this delimiter instead of generating fixed space columns
* `-i`, `--include <INCLUDED_EVENT_IDS>` — List events with only the specified event ids, separated by ','
* `-x`, `--exclude <EXCLUDED_EVENT_IDS>` — Exclude events with the specified event ids, separated by ','
* `-c`, `--colors` — highlight interesting content using colors
* `-f`, `--from <NOT_BEFORE>` — hide events older than the specified date (hint: use RFC 3339 syntax)
* `-t`, `--to <NOT_AFTER>` — hide events newer than the specified date (hint: use RFC 3339 syntax)
* `-r`, `--regex <HIGHLIGHT>` — highlight event data based on this regular expression
* `-s`, `--sort <SORT_ORDER>` — sort order

  Default value: `storage`

  Possible values:
  - `storage`:
    don't change order, output records as they are stored
  - `record-id`:
    sort by event record id
  - `time`:
    sort by date and time

* `-b`, `--base-fields <DISPLAY_SYSTEM_FIELDS>` — display fields common to all events. multiple values must be separated by ','

  Default values: `event-id`, `event-record-id`

  Possible values:
  - `event-id`:
    The identifier that the provider used to identify the event
  - `event-record-id`:
    The record number assigned to the event when it was logged
  - `activity-id`:
    A globally unique identifier that identifies the current activity. The events that are published with this identifier are part of the same activity
  - `related-activity-id`:
    A globally unique identifier that identifies the activity to which control was transferred to. The related events would then have this identifier as their ActivityID identifier
  - `process-id`:
    The ID of the process that created the event

* `-B`, `--hide-base-fields` — don't display any common event fields at all. This corresponds to specifying '--base-fields' without any values (which is not allowed, that's why there is this flag)

  Default value: `false`
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

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
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence
* `-b` — output as bodyfile format



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

# Command-Line Help for `ipgrep`

This document contains the help content for the `ipgrep` command-line program.

**Command Overview:**

* [`ipgrep`↴](#ipgrep)

## `ipgrep`

search for IP addresses in text files

**Usage:** `ipgrep [OPTIONS] [FILE]...`

###### **Arguments:**

* `<FILE>`

###### **Options:**

* `-i`, `--include <INCLUDE>` — display only lines who match ALL of the specified criteria. Values are delimited with comma

  Possible values: `ipv4`, `ipv6`, `public`, `private`, `loopback`

* `-x`, `--exclude <EXCLUDE>` — hide lines who match ANY of the specified criteria. Values are delimited with comma

  Possible values: `ipv4`, `ipv6`, `public`, `private`, `loopback`

* `-I`, `--ignore-ips <IGNORE_IPS>` — ignore any of the specified IP addresses. Values are delimited with comma
* `-c`, `--colors` — highlight interesting content using colors
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

# Command-Line Help for `mactime2`

This document contains the help content for the `mactime2` command-line program.

**Command Overview:**

* [`mactime2`↴](#mactime2)

## `mactime2`

replacement for `mactime`

**Usage:** `mactime2 [OPTIONS]`

###### **Options:**

* `-b <INPUT_FILE>` — path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped)

  Default value: `-`
* `-F`, `--format <OUTPUT_FORMAT>` — output format, if not specified, default value is 'txt'

  Possible values: `csv`, `txt`, `json`, `elastic`

* `-d` — output as CSV instead of TXT. This is a conveniance option, which is identical to `--format=csv` and will be removed in a future release. If you specified `--format` and `-d`, the latter will be ignored
* `-j` — output as JSON instead of TXT. This is a conveniance option, which is identical to `--format=json` and will be removed in a future release. If you specified `--format` and `-j`, the latter will be ignored
* `-f`, `--from-timezone <SRC_ZONE>` — name of offset of source timezone (or 'list' to display all possible values

  Default value: `UTC`
* `-t`, `--to-timezone <DST_ZONE>` — name of offset of destination timezone (or 'list' to display all possible values

  Default value: `UTC`
* `--strict` — strict mode: do not only warn, but abort if an error occurs
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

# Command-Line Help for `pol_export`

This document contains the help content for the `pol_export` command-line program.

**Command Overview:**

* [`pol_export`↴](#pol_export)

## `pol_export`

Exporter for Windows Registry Policy Files

**Usage:** `pol_export [OPTIONS] <POLFILE>`

###### **Arguments:**

* `<POLFILE>` — Name of the file to read

###### **Options:**

* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

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
* `-I`, `--ignore-base-block` — ignore the base block (e.g. if it was encrypted by some ransomware)
* `-T`, `--hide-timestamps` — hide timestamps, if output is in reg format
* `-v`, `--verbose` — More output per occurrence
* `-q`, `--quiet` — Less output per occurrence



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

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

