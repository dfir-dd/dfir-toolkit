# Command-Line Help for `pf2bodyfile`

This document contains the help content for the `pf2bodyfile` command-line program.

**Command Overview:**

* [`pf2bodyfile`↴](#pf2bodyfile)

## `pf2bodyfile`

creates bodyfile from Windows Prefetch files

**Usage:** `pf2bodyfile [OPTIONS] [PREFETCH_FILES]...`

###### **Arguments:**

* `<PREFETCH_FILES>` — names of the prefetch files (commonly files with 'pf' extension in 'C:\Windows\Prefetch')

###### **Options:**

* `-I` — show not only the executed files, but all references files -- such as libraries -- as well

  Possible values: `true`, `false`

* `-v`, `--verbose` — Increase logging verbosity
* `-q`, `--quiet` — Decrease logging verbosity



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

