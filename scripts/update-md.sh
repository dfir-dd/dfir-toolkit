#!/bin/bash

cargo build

cat >README.md <<'EOF'

# DFIR Toolkit

<img align="right" width="64px" src="https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/images/fuchs_blau_q.png?raw=true" />

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

<img src="https://raw.githubusercontent.com/dfir-dd/dfir-toolkit/main/doc/images/tools.svg">

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
EOF

for B in $(cd src/bin; echo *); do
    cargo run --bin $B -- --markdown-help >>README.md
done