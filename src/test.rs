
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use crate::prelude::*;
use core::fmt;

#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

#[test]
fn test_simple() {
	use core::fmt::Write;

	fn test_case<T, S>(expected: &str, v: Vec<T>, separator: S)
	where
		T: fmt::Display,
		S: fmt::Display,
	{
		assert_eq!(expected, format_interspersed!(&v, &separator));
		assert_eq!(expected, format_interspersed!(&v, &separator, x => "{}", x));
	}

	test_case("", Vec::<i32>::new(), "");
	test_case("", Vec::<i32>::new(), ", ");

	test_case("", vec![""], "");
	test_case("a", vec!["a"], "");
	test_case("a", vec!["a"], ", ");
	test_case("123", vec![123], "");
	test_case("123", vec![123], ", ");

	test_case(", ", vec!["", ""], ", ");
	test_case(", a", vec!["", "a"], ", ");
	test_case("a, ", vec!["a", ""], ", ");
	test_case("a, b", vec!["a", "b"], ", ");

	test_case(", , ", vec!["", "", ""], ", ");
	test_case("a, b, c", vec!["a", "b", "c"], ", ");

	test_case("99", vec!["", "", ""], 9);
	test_case("a9b9c", vec!["a", "b", "c"], 9);
}

#[test]
fn test_format_strings() {
	use core::fmt::Write;

	assert_eq!(
		"",
		format_interspersed!(Vec::<i32>::new(), "", _ => "nonempty")
	);
	assert_eq!(
		"",
		format_interspersed!(Vec::<i32>::new(), "nonempty", _ => "nonempty")
	);

	assert_eq!(
		"149",
		format_interspersed!(vec![1_i32, 2, 3], "", i => "{}", i.pow(2))
	);
	assert_eq!(
		"1, 4, 9",
		format_interspersed!(vec![1_i32, 2, 3], ", ", i => "{}", i.pow(2))
	);
	assert_eq!(
		"10409",
		format_interspersed!(vec![1_i32, 2, 3], 0, i => "{}", i.pow(2))
	);

	assert_eq!(
		r#"(x: "a", y: 1); (x: "b", y: 2); (x: "c", y: 3)"#,
		format_interspersed!(
			vec![("a", 1), ("b", 2), ("c", 3)],
			"; ",
			(x, y) => "(x: {:?}, y: {})",
			x,
			y
		)
	);
}

// TODO: test `write` without relying on allocation
#[cfg(feature = "alloc")]
#[test]
fn test_write() {
	use core::fmt::Write;

	let mut buf = std::string::String::new();
	write_interspersed!(&mut buf, 1..=5, 0).unwrap();
	assert_eq!(r#"102030405"#, buf);

	let mut buf = std::string::String::new();
	writeln_interspersed!(
		&mut buf,
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	)
	.unwrap();
	assert_eq!(
		"(x: \"a\", y: 1); (x: \"b\", y: 2); (x: \"c\", y: 3)\n",
		buf
	);
}

// just testing that these compile, are resolved successfully
#[cfg(feature = "std")]
#[test]
fn test_print_compiles() {
	print_interspersed!(1..=3, "; ");
	println_interspersed!(1..=3, "; ");
	eprint_interspersed!(1..=3, "; ");
	eprintln_interspersed!(1..=3, "; ");

	// trailing commas
	print_interspersed!(1..=3, "; ",);
	println_interspersed!(1..=3, "; ",);
	eprint_interspersed!(1..=3, "; ",);
	eprintln_interspersed!(1..=3, "; ",);

	print_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);
	println_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);
	eprint_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);
	eprintln_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);

	// trailing commas
	print_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
	println_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
	eprint_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
	eprintln_interspersed!(
		vec![("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
}
