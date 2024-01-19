
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
  - [x] [`cleanhive`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/cleanhive.md)
  - [x] [`evtx2bodyfile`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/evtx2bodyfile.md)
  - [x] [`evtxanalyze`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/evtxanalyze.md)
  - [x] [`evtxscan`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/evtxscan.md)
  - [x] [`evtxcat`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/evtxcat.md)
  - [x] [`evtxls`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/evtxls.md)
  - [x] [`es4forensics`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/es4forensics.md)
  - [x] [`hivescan`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/hivescan.md)
  - [x] [`ipgrep`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/ipgrep.md)
  - [x] [`lnk2bodyfile`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/lnk2bodyfile.md)
  - [x] [`mactime2`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/mactime2.md)
  - [ ] [`mft2bodyfile`](https://github.com/janstarke/mft2bodyfile)
  - [ ] [`ntdsextract2`](https://github.com/janstarke/ntdsextract2)
  - [x] [`pol_export`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/pol_export.md)
  - [ ] [`procbins`](https://github.com/janstarke/procbins)
  - [x] [`regdump`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/regdump.md)
  - [ ] [`regls`](https://github.com/janstarke/regls)
  - [ ] [`regview`](https://github.com/janstarke/regview)
  - [x] [`ts2date`](https://github.com/dfir-dd/dfir-toolkit/blob/main/doc/ts2date.md)
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

