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
* `-C`, `--color <DISPLAY_COLORS>` — highlight interesting content using colors

  Default value: `auto`

  Possible values: `auto`, `always`, `never`

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

  Possible values: `true`, `false`

* `-v`, `--verbose` — Increase logging verbosity
* `-q`, `--quiet` — Decrease logging verbosity



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

