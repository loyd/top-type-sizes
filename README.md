# top-type-sizes

Nightly rustc provides the `print-type-sizes` option for printing sizes of all types. It's especially useful for analyzing why futures are soo big, that can hurt performance a lot if such futures are moved.

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

## Example
```
2032 [async fn body@/home/.cargo/registry/src/github.com-1ecc6299db9ec823/trust-dns-resolver-0.22.0/src/name_server/name_ser
ver_pool.rs:310:1: 374:2] align=8
    1 <discriminant>
 2031 variant Suspend0
    128 opts align=8 offset=0
    128 opts
     32 err
     16 backoff
     24 conns
    216 request
    504 busy
    504 par_conns
     24 conns
    216 request
      1 generator_field11
      1 generator_field12
      1 generator_field13
      5 <padding>
    216 request_cont align=8
     16 __awaitee
 1831 variant Suspend1
    128 opts align=8 offset=0
    128 opts
     32 err
     16 backoff
     24 conns
    216 request
    504 busy
    504 par_conns
     24 conns
    216 request
      1 generator_field11
      1 generator_field12
      1 generator_field13
      5 <padding>
     24 requests align=8
      8 __awaitee
 1792 variant Unresumed, Returned, Panicked
    127 <padding>
    128 opts align=8
   1296 <padding>
     24 conns align=8
    216 request
```
