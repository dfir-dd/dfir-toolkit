#!/bin/bash

cargo build

cat >README.md <<'EOF'

# DFIR Toolkit

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
  - [ ] [`ts2date`](https://github.com/janstarke/ts2date)
  - [ ] [`usnjrnl_dump`](https://github.com/janstarke/usnjrnl)

# Overview of timelining tools

<img src="https://github.com/dfir-dd/dfir-toolkit/blob/master/doc/images/tools.svg?raw=true">

# Installation

```bash
cargo install dfir-toolkit
```

# Tools
EOF

for B in $(cd src/bin; echo *); do
    cargo run --bin $B -- --markdown-help >>README.md
done