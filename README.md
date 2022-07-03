# bj

A CLI for converting BCS-encoded messages into JSON.

## Installing

Run:

```bash
# If on Diem
cargo install bj

# If on Sui
cargo install bj --features address20

# If on Aptos
cargo install bj --features address32
```

## Usage

```
bj 0.1.0
Ian Macalinao <ian@moving.wtf>
A CLI for converting BCS-encoded messages into JSON.

USAGE:
    bj <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

FORMATS:
    errmap    A [move_core_types::errmap::ErrorMapping]. (see
                  <https://docs.rs/mv-core-types/latest/move_core_types/errmap/struct.ErrorMapping.html>)
```

## License

`bj` is licensed under the Apache License, Version 2.0.
