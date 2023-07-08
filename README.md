# DFIR Toolkit

## Installation

```bash
cargo install dfir-toolkit
```

# Tools

### evtx2bodyfile

yet to be come

### mactime2

Replacement for `mactime`

#### Changes to original `mactime`

 - no implicit conversion of timestamp to local date/time
 - possibility of explicit timezone correction
 - other datetime format (RFC3339) which always includes the timezone offset
 - faster

#### Usage

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

### mft2bodyfile

yet to be come

### pol_export

Exporter for Windows Registry Policy Files

#### Usage

```bash
USAGE:
    pol_export <POLFILE>

ARGS:
    <POLFILE>    Name of the file to read

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

#### More information

 - <https://docs.microsoft.com/en-us/previous-versions/windows/desktop/policy/registry-policy-file-format>