# top-type-sizes

Nightly rustc provides the `print-type-sizes` option for printing sizes of all types. It's especially useful for analyzing why futures are soo big, that can hurt performance a lot if such futures are moved.

However, `print-type-sizes` produces unordered and cluttered output. This crate parses that output, refine it and show top types by size in compact form.

Features:
* Sorts types by size.
* Deduplicates same types.
* Merges variants with similar layout.
* Shows layouts in compact form.
* Sorts fields by size (`-s`).
* Hides small fields (`-h`).
* Hides wrappers like `MaybeUninit` and custom ones (`-w`).
* Filters by type names (`-f` and `-e`).
* Limits output (`-l`).
* Expands specific types with children, heuristically (`-p`).

## Usage
Firstly, install by using `cargo install top-type-sizes` or clone the repository and run `cargo build --release`.

Secondly, compile your project:
```sh
$ RUSTFLAGS=-Zprint-type-sizes cargo +nightly build -j 1 > type-sizes.txt
```
* It should be a fresh build without cache. Otherwise, part of info will be lost.
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
top-type-sizes 0.1.4

USAGE:
    top-type-sizes [FLAGS] [OPTIONS]

FLAGS:
        --help               Prints help information
    -w, --remove-wrappers    Removes wrappers like `MaybeUninit`
    -r, --reverse            Prints top `limit` types in ascending order
    -s, --sort-fields        Sorts fields by size and removes paddings
    -V, --version            Prints version information

OPTIONS:
    -e, --exclude <exclude>...     Excludes types that match these patterns
    -p, --expand <expand>...       Shows only types that match these patterns and their children, heuristically
    -f, --filter <filter>...       Shows only types that match these patterns
    -h, --hide-less <hide-less>    Hides fields with size less than this value [default: 0]
    -l, --limit <limit>            Shows only this number of top types [default: 100]
```

## Example
```
3456 [async block@/home/.cargo/registry/src/github.com-1ecc6299db9ec823/trust-dns-resolver-0.22.0/src/name_server/name_server_pool.rs:256:23: 296:10] align=8
      1 <discriminant>
   3455 variant Suspend1
        128 opts align=8 offset=0
         16 datagram_conns
         16 stream_conns
        216 request
        216 tcp_message
          1 generator_field3
          1 generator_field4
          1 generator_field5
          1 generator_field6
          1 generator_field7
          1 generator_field8
          1 generator_field9
          1 <padding>
        200 udp_res align=8
       2656 __awaitee
   3255 variant Suspend0
        128 opts align=8 offset=0
         16 datagram_conns
         16 stream_conns
        216 request
        216 tcp_message
          3 <padding>
          1 generator_field6 align=1
          1 generator_field7
          1 generator_field8
          1 generator_field9
          1 <padding>
       2656 __awaitee align=8
    592 variant Unresumed, Returned, Panicked
        128 opts align=8 offset=0
         16 datagram_conns
         16 stream_conns
        216 request
        216 tcp_message
```
