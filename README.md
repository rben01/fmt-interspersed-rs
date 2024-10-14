# fmt-interspersed

This crate provides analogs of the [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html) macros such as
[`format!`](https://doc.rust-lang.org/std/macro.format.html) and
[`write!`](https://doc.rust-lang.org/std/macro.write.html) to make it easier to
“stringify” the contents of an iterator interspersed with a separator, without
intermediate allocations. The items yielded by the iterator do not need to be the same
type as the separator.


```rust
use fmt_interspersed::prelude::*;

let s = "abc";
assert_eq!("a0b0c", format_interspersed!(s.chars(), 0));
```

In the above, `s.chars()::Item` implements
[`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html). But you can
specify a custom format to use to display the items, which is useful when the iterator’s
items aren't `Display` or need customization. (The separator is always stringified using
its `Display` implementation, and must implement `Display`.)

```rust
let pairs = vec![("a", 1), ("b", 2)];
assert_eq!(
    r#"(x: "a", y: 1); (x: "b", y: 2)"#,
    format_interspersed!(pairs, "; ", (x, y) => "(x: {x:?}, y: {y})")
);
```

There are equivalents of all of the `format_args!`-related macros (except for
`format_args!` itself), so you can, for example, write to a file without allocating any
intermediate strings:

```rust
// as with `write!`, the necessary trait for writing, either 
// io::Write or fmt::Write, must be in scope
use std::{fs, io::Write};

let mut f = fs::File::create("file.txt")?;
write_interspersed!(f, 1_i32..=5, '-', n => "{:02}", n.pow(2));
let s = fs::read_to_string("file.txt")?;
assert_eq!("01-04-09-16-25", s);
```

## Features and `no_std`

This crate has two features: `alloc` and `std`. `std` is enabled by default and implies
`alloc`. With `alloc` and `std` disabled, this crate is `#![no_std]`-compatible.

The macros enabled with the various features are as follows:

- No features
  - [`write_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.write_interspersed.html)
  - [`writeln_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.writeln_interspersed.html)

- `alloc`
  - [`format_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.format_interspersed.html)

- `std`
  - [`eprint_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.eprint_interspersed.html)
  - [`eprintln_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.eprintln_interspersed.html)
  - [`print_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.print_interspersed.html)
  - [`println_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.println_interspersed.html)

## Pitfalls

1. Unlike `write!`, you cannot `.unwrap()` the result of `write_interspersed!`. This is
   because `write_interspersed!` expands to multiple calls to `write!`, so there is no
   single expression to unwrap. If you want to unwrap the result of
   `write_interspersed!`, you can wrap it in a closure as follows:

   ```rust
   // result type here must suit the destination;
   // io::Result for a file, fmt::Result for a string
   (|| -> std::io::Result {
      write_interspersed!(args...);
      Ok(())
   }).unwrap()
   ```

   When [`try_blocks`](https://github.com/rust-lang/rust/issues/31436) are stabilized,
   that will become the preferred syntax for this.
