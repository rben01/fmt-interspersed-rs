# fmt-interspersed

This crate does one thing: given an `IntoIterator` and a separator `sep` to intersperse
it with, write the items interspersed with `sep` to a formatter. This happens without
allocating any intermediate strings.

```rust
use fmt_interspersed::FmtInterspersed;

let s = "abc";
let fmt_sep = FmtInterspersed::new(s.chars(), ", ");
assert_eq!("a, b, c", format!("{fmt_sep}"));
```

In the above, `s.chars()::Item` implements `std::fmt::Display`. But you can specify a
custom function to write the items with, which is useful when the iterator’s items
aren't `Display` or need customization.

```rust
let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
let fmt_sep =
   FmtInterspersed::new_with_fn(&pairs, |f, &(x, y)| write!(f, "(x: {x:?}, y: {y})"), ';');
assert_eq!(
   r#"(x: "a", y: 1);(x: "b", y: 2);(x: "c", y: 3)"#,
   format!("{fmt_sep}")
);
```

This works with all of the `format_args!`-related macros, so you can e.g. write to a
`String` buffer without allocating any intermediate strings:

```rust
// necessary to write to a `&mut String`
use std::fmt::Write;

let v = 1_i32..=5;
let fmt_sep = FmtInterspersed::new_with_fn(v, |f, n| write!(f, "{}", n.pow(2)), 0);

let mut s = String::new();
write!(&mut s, "{fmt_sep}").unwrap();

assert_eq!("10409016025", s);
```

An empty iterator produces no output, and an iterator with one item produces the item
without the separator.

## Pitfalls

1. The `IntoIterator` passed to `FmtInterspersed` must be such that its `IntoIter:
Clone` (where `IntoIter` is the type returned by calling `.into_iter()` on it). If you
   find that a collection’s `IntoIter` is not `Clone`, try passing a borrowed form of
   the collection instead. For instance, `<HashMap as IntoIterator>::IntoIter` is not
   `Clone`, but `<&HashMap as IntoIterator>::IntoIter` is `Clone`. Similarly,
   `hash_map.into_keys()` returns a non-`Clone` iterator, whereas `hash_map.keys()` is
   `Clone`. The iterators produced by `Vec<T>::into_iter`, `<&Vec<T>>::into_iter`,
   `<&[T]>::into_iter`, and `<[T; N]>::into_iter` are all `Clone`, although it's
   probably not a good idea to pass a `Vec<T>`, whose `IntoIter` will clone the
   underlying `Vec<T>` when cloned, if a `&Vec<T>` or `&[T]` will do instead.

1. The type signature of the function passed to `new_with_fn` must _exactly_ match the
   type yielded by the iterator. In the second example above, the signature _must_ be
   `|f, &(x, y)|`, and not, say, `|f, (x, y)|` (which you might normally write in
   non-generic code, expecting the references to magically be moved inside the tuple).
