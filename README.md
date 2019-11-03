Memmon
======

[![Actions Status](https://github.com/itkovian/sarchive/workflows/memmon%20tests/badge.svg)](https://github.com/itkovian/memmon/actions)
[![Coverage Status](https://coveralls.io/repos/github/itkovian/memmon/badge.svg)](https://coveralls.io/github/itkovian/memmon)
[![License](https://img.shields.io/github/license/itkovian/memmon)](https://opensource.org/licenses/MIT)

This crate aims to provide a monitoring service for memory usage of applications running on Linux through the proc fs.

# Done

- Show stats at end of process when `exit` is called.

# TODO

- Intercept signals to show data upon termination as well
- Ship information at regular intervals to a backend of choice
    - file
    - elasticsearch
    - tcp

# Usage

- cargo build --release
- export LD_PRELOAD=$PWD/target/release/libmemmon.so
