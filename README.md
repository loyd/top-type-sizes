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

```text
top-type-sizes 0.1.5

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

## Examples
For instance, let's analyze the [`tokio/chat`](https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs) example:
```sh
RUSTFLAGS=-Zprint-type-sizes cargo +nightly build --example chat -j 1 > chat.txt
```

Once the compiler's output is collected, you can perform multiple queries until results become representative.

Initially, find interesting entry functions:
```sh
top-type-sizes -f chat.rs < chat.txt | less
```

* `-f <pattern>` hides all types that doesn't match the provided pattern. Note, that `async fn` has a path in a type name.

```text
...
1032 [async fn body@examples/chat.rs:174:33: 243:2] align=8
...
```

Ok, it's the [`process`](https://github.com/tokio-rs/tokio/blob/4ea632005d689f850e87a116b9e535a0015a7a0f/examples/chat.rs#L170) function, let's check it and children types.

```sh
top-type-sizes -w -s -h 33 -p body@examples/chat.rs:174:33 < chat.txt | less
```

* `-w` hides wrappers, e.g.
    ```text
    1032 std::mem::MaybeUninit<[async fn body@examples/chat.rs:174:33: 243:2]> align=8
       1032 variant MaybeUninit
           1032 value
    ```
* `-s` sorts fields by size and hides paddings.
* `-h <size>` hides all fields with size less than the provided size.
* `-p <pattern>` hides all types that aren't contained in `<patten>` types. Note that the compiler doesn't provide types of fields, so this parameter filters types recursively by field sizes and can leave a lot of irrelevant types for small sizes (because they are more frequent). But it's very useful anyway.

Output:
```text
1032 [async fn body@examples/chat.rs:174:33: 243:2] align=8
   1031 variant Suspend2
        472 __awaitee align=8
        144 lines
         40 stream
    671 variant Suspend3, Suspend7, Suspend9
        152 peer
        144 lines
        112 __awaitee align=8
         40 stream
    647 variant Suspend4, Suspend8, Suspend10
        152 peer
        144 lines
         64 __awaitee
         40 stream
    623 variant Suspend5
        152 peer
        144 lines
         40 stream
         40 futures
    599 variant Suspend6
        152 peer
        144 lines
         40 stream
    583 variant Suspend0
        144 lines
         40 stream
    567 variant Suspend1
        144 lines
         40 stream
    552 variant Unresumed, Returned, Panicked
         40 stream

472 [async fn body@examples/chat.rs:155:27: 166:6] align=8
    465 variant Suspend0
        144 lines
        144 lines
        112 __awaitee
    464 variant Unresumed, Returned, Panicked
        144 lines align=8
...
```

Note: `__awaitee` means awaiting on an inner future.

Then, you can use `-f` and `-e` to refine output even more.
