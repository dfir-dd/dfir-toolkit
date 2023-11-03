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

