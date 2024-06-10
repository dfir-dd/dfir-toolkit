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

  Possible values: `true`, `false`

* `-v`, `--verbose` — Increase logging verbosity
* `-q`, `--quiet` — Decrease logging verbosity



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

