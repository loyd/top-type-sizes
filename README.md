# top-type-sizes

Nightly rustc provides the `print-type-sizes` option for printing sizes of all types. It's especially useful for analyzing why futures are soo big, that can hurt performance a lot if such futures are moved.

However, `print-type-sizes` produces unordered and cluttered output. This crate parses that output, refine it and show top types by size in compact form.

Features:
* Sorts types by size.
* Deduplicates same types.
* Merges variants with similar layout.
* Shows layouts in compact form.
* Sorts fields by size (`-s`).
* Hides small types and fields (`-h`).
* Hides wrappers like `MaybeUninit` and custom ones (`-w`).
* Filters by type names (`-f` and `-e`).
* Limits output (`-l`).
* Expands specific types with children (`-p`).

## Usage
Firstly, install by using `cargo install top-type-sizes` or clone the repository and run `cargo build --release`.

Secondly, compile your project:
```sh
$ RUSTFLAGS=-Zprint-type-sizes cargo +nightly build -j1 > type-sizes.txt
```
* It should be a fresh build without cache. Otherwise, part of info will be lost.
* `-Zprint-type-sizes` requires the nightly compiler.
* `-j1` is required to avoid incorrect shuffled output.

Finally, use this crate:
```sh
$ top-type-sizes < type-sizes.txt | less
```

### Help
```sh
$ top-type-sizes --help
```

```text
top-type-sizes 0.2.1

USAGE:
    top-type-sizes [FLAGS] [OPTIONS]

FLAGS:
        --expand-by-size
            Modify the -p/--expand option to expand also by field's size

        --help
            Prints help information

    -w, --remove-wrappers
            Hides wrappers like `MaybeUninit` and `ManuallyDrop`.

            This option removes types having the same layout as an inner type.

    -r, --reverse
            Prints types in descending order.

            This option is applied after the -l/--limit option.

    -s, --sort-fields
            Sorts fields by size and hides paddings.

            Note: enum variants are sorted and merged anyway.

    -V, --version
            Prints version information


OPTIONS:
    -e, --exclude <exclude>...
            Excludes types that match these patterns.

            Patterns are regex (in the regex crate's syntax). Can be provided multiple times.

    -p, --expand <expand>...
            Shows only types that match these patterns and their children.

            It uses two mechanisms to expand types:
             - by field's type name (requires at least nightly 24-03-22)
             - by field's size if the `--expand_by_size` option is enabled

            Note: currently field's type names are provided only for `await`.

            Patterns are regex (in the regex crate's syntax). Can be provided multiple times.

    -f, --filter <filter>...
            Shows only types that match these patterns.

            Patterns are regex (in the regex crate's syntax). Can be provided multiple times.

    -h, --hide-less <hide-less>
            Hides types and fields with size less than this value

    -l, --limit <limit>
            Shows only this number of top types.

            This limit is applied after all other filters.
```

## Examples
For instance, let's analyze the [`tokio/chat`](https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs) example:
```sh
RUSTFLAGS=-Zprint-type-sizes cargo +nightly build --example chat -j 1 > chat.txt
```

Once the compiler's output is collected, we can perform multiple queries until results become representative.

Initially, show all types sorted by size and find interesting ones:
```sh
top-type-sizes < chat.txt | less
```

For instance, if we want to expand `async fn process()` function:
```text
...
1024 {async fn body of process()} align=8
...
```

We can use the `-p`/`--expand` option to show only this function and its children types:
```sh
top-type-sizes -ws -h33 -p 'process\(\)' < chat.txt | less
```
* `-w` hides wrappers, e.g.
    ```text
    1032 std::mem::MaybeUninit<[async fn body@examples/chat.rs:174:33: 243:2]> align=8
       1032 variant MaybeUninit
           1032 value
    ```
* `-s` sorts fields by size and hides paddings.
* `-h <size>` hides all types and fields with size less than the provided size.
* `-p <pattern>` hides all types that aren't contained in `<patten>` types. Note that the compiler doesn't provide types of fields for all types, only for awaitees. It's possible to use `--expand-by-size` to expand also by field's size, but it can show also irrelevant types.

Output:
```text
1024 {async fn body of process()} align=8
   1023 variant Suspend2
        472 __awaitee align=8 type={async fn body of Peer::new()}
        144 lines
         40 stream (upvar) align=8 offset=0
    663 variant Suspend3, Suspend7, Suspend9
        152 peer
        144 lines
        112 __awaitee align=8 type={async fn body of tokio::sync::Mutex<Shared>::lock()}
         40 stream (upvar) align=8 offset=0
    639 variant Suspend4, Suspend8, Suspend10
        152 peer
        144 lines
         64 __awaitee type={async fn body of Shared::broadcast()}
         40 stream (upvar) align=8 offset=0
    615 variant Suspend5
        152 peer
        144 lines
         40 stream (upvar) align=8 offset=0
         40 futures align=8
    591 variant Suspend6
        152 peer
        144 lines
         40 stream (upvar) align=8 offset=0
    575 variant Suspend0
        144 lines
         40 stream (upvar) align=8 offset=0
    559 variant Suspend1
        144 lines
         40 stream (upvar) align=8 offset=0
     80 variant Unresumed, Returned, Panicked
         40 stream (upvar) align=8 offset=0

472 {async fn body of Peer::new()} align=8
    465 variant Suspend0
        144 lines (upvar) align=8 offset=0
        144 lines
        112 __awaitee type={async fn body of tokio::sync::Mutex<Shared>::lock()}
    152 variant Unresumed, Returned, Panicked
        144 lines (upvar) align=8 offset=0

112 {async fn body of tokio::sync::Mutex<Shared>::lock()} align=8
    104 variant Suspend0
         88 __awaitee type={async block@tokio::sync::Mutex<Shared>::lock::{closure#0}::{closure#0}}
      8 variant Unresumed, Returned, Panicked

88 {async block@tokio::sync::Mutex<Shared>::lock::{closure#0}::{closure#0}} align=8
     80 variant Suspend0
         72 __awaitee type={async fn body of tokio::sync::Mutex<Shared>::acquire()}
      8 variant Unresumed, Returned, Panicked

72 {async fn body of tokio::sync::Mutex<Shared>::acquire()} align=8
     64 variant Suspend0
         56 __awaitee type=tokio::sync::batch_semaphore::Acquire<'_>
      8 variant Unresumed, Returned, Panicked

64 {async fn body of Shared::broadcast()} align=8
     56 variant Unresumed, Returned, Panicked

56 tokio::sync::batch_semaphore::Acquire<'_> align=8
     40 node
```

Note: `__awaitee` means awaiting on an inner future.

Then, we can use `-f` and `-e` to refine output even more.
