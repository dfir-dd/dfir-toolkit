# DFIR Toolkit

# Table of contents

- [Installation](#installation)
- [Overview of timelining tools](#overview-of-timelining-tools)
- [Tools](#tools)
  - [x] [`evtx2bodyfile`](#evtx2bodyfile)
  - [x] [`evtx2analyze`](#evtx2analyze)
  - [x] [`evtxscan`](#evtxscan)
  - [x] [`evtxcat`](#evtxcat)
  - [x] [`evtxls`](#evtxls)
  - [ ] [`es4forensics`](https://github.com/janstarke/es4forensics)
  - [ ] [`hivescan`](https://github.com/janstarke/nt_hive2)
  - [ ] [`ipgrep`](https://github.com/janstarke/ipgrep)
  - [ ] [`lnk2bodyfile`](https://github.com/janstarke/lnk2bodyfile)
  - [x] [`mactime2`](#mactime2)
  - [ ] [`mft2bodyfile`](https://github.com/janstarke/mft2bodyfile)
  - [ ] [`ntdsextract2`](https://github.com/janstarke/ntdsextract2)
  - [x] [`pol_export`](#pol_export)
  - [ ] [`procbins`](https://github.com/janstarke/procbins)
  - [ ] [`regdump`](https://github.com/janstarke/nt_hive2)
  - [ ] [`regls`](https://github.com/janstarke/regls)
  - [ ] [`regview`](https://github.com/janstarke/regview)
  - [ ] [`ts2date`](https://github.com/janstarke/ts2date)
  - [ ] [`usnjrnl_dump`](https://github.com/janstarke/usnjrnl)

# Overview of timelining tools

<img src="https://github.com/janstarke/dfir-toolkit/blob/master/doc/images/tools.svg">

# Installation

```bash
cargo install dfir-toolkit
```

# Tools

## `evtx2bodyfile`

### Usage

```
Usage: evtx2bodyfile [OPTIONS] [EVTX_FILES]...

Arguments:
  [EVTX_FILES]...  names of the evtx files

Options:
  -J, --json        output json for elasticsearch instead of bodyfile
  -S, --strict      fail upon read error
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -h, --help        Print help
  -V, --version     Print version
```

### Example

```shell
# convert to bodyfile only
evtx2bodyfile Security.evtx >Security.bodyfile

# create a complete timeline
evtx2bodyfile *.evtx | mactime2 -d -b >evtx_timeline.csv
```

## `evtxanalyze`

Analyze evtx files

### Usage

```
Usage: evtxanalyze [OPTIONS] <COMMAND>

Commands:
  pstree    generate a process tree
  sessions  display sessions
  session   display one single session
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -h, --help        Print help
```

## `evtxscan`

Finds time skews in an evtx file

### Example

<img src="https://github.com/janstarke/evtxtools/blob/master/doc/img/evtxscan1.png?raw=true">

<img src="https://github.com/janstarke/evtxtools/blob/master/doc/img/evtxscan2.png?raw=true">

### Usage

```
Find time skews in an evtx file

Usage: evtxscan [OPTIONS] <EVTX_FILE>

Arguments:
  <EVTX_FILE>  name of the evtx file to scan

Options:
  -S, --show-records                             display also the contents of the records befor and after a time skew
  -N, --negative-tolerance <NEGATIVE_TOLERANCE>  negative tolerance limit (in seconds): time skews to the past below this limit will be ignored [default: 5]
  -h, --help                                     Print help
  -V, --version                                  Print version
```

## `evtxcat`

Display one or more events from an evtx file

### Example

<img src="https://github.com/janstarke/evtxtools/blob/master/doc/img/evtxls.png?raw=true">

### Usage
```

Usage: evtxcat [OPTIONS] <EVTX_FILE>

Arguments:
  <EVTX_FILE>  Name of the evtx file to read from

Options:
      --min <MIN>        filter: minimal event record identifier
      --max <MAX>        filter: maximal event record identifier
  -i, --id <ID>          show only the one event with this record identifier
  -T, --display-table    don't display the records in a table format
  -F, --format <FORMAT>  [default: xml] [possible values: json, xml]
  -h, --help             Print help
  -V, --version          Print version

```

## `evtxls`

Display one or more events from an evtx file

### Usage 

```
Usage: evtxls [OPTIONS] [EVTX_FILES]...

Arguments:
  [EVTX_FILES]...
          Name of the evtx files to read from

Options:
  -d, --delimiter <DELIMITER>
          use this delimiter instead of generating fixed space columns

  -i, --include <INCLUDED_EVENT_IDS>
          List events with only the specified event ids, separated by ','

  -x, --exclude <EXCLUDED_EVENT_IDS>
          Exclude events with the specified event ids, separated by ','

  -c, --colors
          highlight interesting content using colors

  -f, --from <NOT_BEFORE>
          hide events older than the specified date (hint: use RFC 3339 syntax)

  -t, --to <NOT_AFTER>
          hide events newer than the specified date (hint: use RFC 3339 syntax)

  -r, --regex <HIGHLIGHT>
          highlight event data based on this regular expression

  -s, --sort <SORT_ORDER>
          sort order
          
          [default: storage]

          Possible values:
          - storage:   don't change order, output records as they are stored
          - record-id: sort by event record id
          - time:      sort by date and time

  -b, --base-fields <DISPLAY_SYSTEM_FIELDS>
          display fields common to all events. multiple values must be separated by ','
          
          [default: event-id event-record-id]

          Possible values:
          - event-id:            The identifier that the provider used to identify the event
          - event-record-id:     The record number assigned to the event when it was logged
          - activity-id:         A globally unique identifier that identifies the current activity. The events that are published with this identifier are part of the same activity
          - related-activity-id: A globally unique identifier that identifies the activity to which control was transferred to. The related events would then have this identifier as their ActivityID identifier
          - process-id:          The ID of the process that created the event

  -B, --hide-base-fields
          don't display any common event fields at all. This corresponds to specifying '--base-fields' without any values (which is not allowed, that's why there is this flag)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## mactime2

Replacement for `mactime`

### Changes to original `mactime`

 - no implicit conversion of timestamp to local date/time
 - possibility of explicit timezone correction
 - other datetime format (RFC3339) which always includes the timezone offset
 - faster

### Usage

```
Usage: mactime2 [OPTIONS]

Options:
  -v, --verbose...                More output per occurrence
  -q, --quiet...                  Less output per occurrence
  -b <INPUT_FILE>                 path to input file or '-' for stdin (files ending with .gz will be treated as being gzipped) [default: -]
  -f, --from-timezone <SRC_ZONE>  name of offset of source timezone (or 'list' to display all possible values
  -t, --to-timezone <DST_ZONE>    name of offset of destination timezone (or 'list' to display all possible values
      --strict                    strict mode: do not only warn, but abort if an error occurs
  -F, --format <OUTPUT_FORMAT>    output format, if not specified, default value is 'txt' [possible values: csv, txt, json, elastic]
  -d                              output as CSV instead of TXT. This is a conveniance option, which is identical to `--format=csv` and will be removed in a future release.
                                  If you specified `--format` and `-d`, the latter will be ignored
  -j                              output as JSON instead of TXT. This is a conveniance option, which is identical to `--format=json` and will be removed in a future release.
                                  If you specified `--format` and `-j`, the latter will be ignored
  -h, --help                      Print help information
  -V, --version                   Print version information

```

## mft2bodyfile

yet to be come

## pol_export

Exporter for Windows Registry Policy Files

### Usage

```bash
USAGE:
    pol_export <POLFILE>

ARGS:
    <POLFILE>    Name of the file to read

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

### More information

 - <https://docs.microsoft.com/en-us/previous-versions/windows/desktop/policy/registry-policy-file-format>