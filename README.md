# top-type-sizes

Nightly rustc provides the `print-type-sizes` option for printing sizes of all types. It's this is especially useful for analyzing why futures are soo big, that can hurt performance a lot if such futures are moved.

However, `print-type-sizes` produces unordered and cluttered output. This crate parses that output, refine it and show top types by a size in compact form.

Features:
* Sorts layouts by a type size.
* Deduplicates same types.
* Merges variants with similar layout.
* Shows layouts in compact form.
* Limits output.

## Usage
Firstly, install by using `cargo install top-type-sizes` or clone the repository and run `cargo build --release`.

Secondly, compile your project:
```sh
$ RUSTFLAGS=-Zprint-type-sizes cargo +nightly build -j 1 > type-sizes.txt
```
* `-Zprint-type-sizes` requires the nightly compiler.
* `-j 1` is required to avoid incorrect shuffled output.

Finally, use this crate:
```sh
$ top-type-sizes < type-sizes.txt | less
```

### Help

```sh
$ top-type-sizes --help
```

```
top-type-sizes 0.1.0

USAGE:
    top-type-sizes [FLAGS] [OPTIONS]

FLAGS:
        --help       Prints help information
    -r, --reverse
    -V, --version    Prints version information

OPTIONS:
    -h, --hide-less <hide-less>     [default: 0]
    -l, --limit <limit>             [default: 100]
```
