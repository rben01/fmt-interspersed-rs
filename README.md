<!-- begin -->
# fmt-interspersed

[![build](https://img.shields.io/github/actions/workflow/status/rben01/fmt-interspersed-rs/rust.yml?branch=main)](https://github.com/rben01/fmt-interspersed-rs/actions?query=branch%3Amain)
[![docs.rs](https://img.shields.io/docsrs/fmt-interspersed)](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/)
[![crates.io](https://img.shields.io/crates/v/fmt-interspersed.svg)](https://crates.io/crates/fmt-interspersed)
[![msrv](https://img.shields.io/crates/msrv/fmt-interspersed.svg)](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/rben01/fmt-interspersed-rs/blob/main/LICENSE)



This crate provides analogs of the
[`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html) macros such as
[`format!`](https://doc.rust-lang.org/std/macro.format.html) and
[`write!`](https://doc.rust-lang.org/std/macro.write.html) to make it easier to
“stringify” the contents of an iterator interspersed with a separator without any
intermediate allocations. The items yielded by the iterator do not need to be the same
type as the separator.

<!-- end -->

```rust
use fmt_interspersed::prelude::*;

let s = "abc";
assert_eq!("a0b0c", format_interspersed!(s.chars(), 0));
```

<!-- begin -->

In the above, `s.chars()::Item` implements
[`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html). But you can
specify a custom format to use to display the items, which is useful when the iterator’s
items aren't `Display` or need customization. This takes the form of `pattern =>
fmt_args...` as the final argument. (The separator is always stringified using its
`Display` implementation and must implement `Display`.)

<!-- end -->

```rust
let pairs = vec![("a", 1), ("b", 2)];
assert_eq!(
    r#"(x: "a", y: 1); (x: "b", y: 2)"#,
    format_interspersed!(pairs, "; ", (x, y) => "(x: {x:?}, y: {y})")
);
```

<!-- begin -->

There are equivalents of all of the `format_args!`-related macros (except for
`format_args!` itself), so you can, for example, write to a string, file, or buffer without
allocating any intermediate strings:

<!-- end -->

```rust
// as with `write!`, the necessary trait for writing, either `fmt::Write`
// (for strings) or `io::Write` (for files or other byte sinks), must be in scope
use std::fmt::Write;

let mut buf = String::new();
write_interspersed!(&mut buf, 1_i32..=5, '-', n => "{:02}", n.pow(2))?;
assert_eq!("01-04-09-16-25", buf);
```

<!-- begin -->
<!-- end -->

```rust
use std::io::{Cursor, Write};

let mut buf = Cursor::new(Vec::<u8>::new());
writeln_interspersed!(&mut buf, "abc".bytes(), ',', b => "{}", b - b'a')?;
write_interspersed!(&mut buf, "abc".bytes(), ',', b => "{}", (b - b'a' + b'A') as char)?;
assert_eq!("0,1,2\nA,B,C", String::from_utf8(buf.into_inner()).unwrap());
```

<!-- begin -->

## Macros, features, and `no_std`

This crate has two features: `alloc` and `std`. `std` is enabled by default and implies
`alloc`. With `alloc` and `std` disabled, this crate is `#![no_std]`-compatible.

Below is the list of this crate’s macros and the features that enable them:

- Require no features
  - [`write_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.write_interspersed.html)
  - [`writeln_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.writeln_interspersed.html)

- Requires `alloc`
  - [`format_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.format_interspersed.html)

- Require `std`
  - [`print_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.print_interspersed.html)
  - [`println_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.println_interspersed.html)
  - [`eprint_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.eprint_interspersed.html)
  - [`eprintln_interspersed!`](https://docs.rs/fmt-interspersed/latest/fmt_interspersed/macro.eprintln_interspersed.html)

<!-- end -->
