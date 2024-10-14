#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
#[test]
fn test_simple() {
	use core::fmt::{self, Write};

	fn test_case<T, S>(expected: &str, v: &[T], separator: S)
	where
		T: fmt::Display,
		S: fmt::Display,
	{
		assert_eq!(expected, format_interspersed!(&v, &separator));
		assert_eq!(expected, format_interspersed!(&v, &separator, x => "{}", x));
	}

	test_case("", &[0_i32; 0], "");
	test_case("", &[0_i32; 0], ", ");

	test_case("", &[""], "");
	test_case("a", &["a"], "");
	test_case("a", &["a"], ", ");
	test_case("123", &[123], "");
	test_case("123", &[123], ", ");

	test_case(", ", &["", ""], ", ");
	test_case(", a", &["", "a"], ", ");
	test_case("a, ", &["a", ""], ", ");
	test_case("a, b", &["a", "b"], ", ");

	test_case(", , ", &["", "", ""], ", ");
	test_case("a, b, c", &["a", "b", "c"], ", ");

	test_case("99", &["", "", ""], 9);
	test_case("a9b9c", &["a", "b", "c"], 9);
}

#[cfg(feature = "alloc")]
#[test]
fn test_format_strings() {
	use core::fmt::Write;

	assert_eq!("", format_interspersed!(&[0_i32; 0], "", _ => "nonempty"));
	assert_eq!(
		"",
		format_interspersed!(&[0_i32; 0], "nonempty", _ => "nonempty")
	);

	assert_eq!(
		"149",
		format_interspersed!([1_i32, 2, 3], "", i => "{}", i.pow(2))
	);
	assert_eq!(
		"1, 4, 9",
		format_interspersed!([1_i32, 2, 3], ", ", i => "{}", i.pow(2))
	);
	assert_eq!(
		"10409",
		format_interspersed!([1_i32, 2, 3], 0, i => "{}", i.pow(2))
	);

	assert_eq!(
		r#"(x: "a", y: 1); (x: "b", y: 2); (x: "c", y: 3)"#,
		format_interspersed!(
			[("a", 1), ("b", 2), ("c", 3)],
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
fn test_write() -> core::fmt::Result {
	use core::fmt::Write;

	let mut buf = std::string::String::new();
	write_interspersed!(&mut buf, 1..=5, 0);
	assert_eq!(r#"102030405"#, buf);

	let mut buf = std::string::String::new();
	writeln_interspersed!(
		&mut buf,
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
	assert_eq!(
		"(x: \"a\", y: 1); (x: \"b\", y: 2); (x: \"c\", y: 3)\n",
		buf
	);

	Ok(())
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
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);
	println_interspersed!(
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);
	eprint_interspersed!(
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);
	eprintln_interspersed!(
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y
	);

	// trailing commas
	print_interspersed!(
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
	println_interspersed!(
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
	eprint_interspersed!(
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
	eprintln_interspersed!(
		[("a", 1), ("b", 2), ("c", 3)],
		"; ",
		(x, y) => "(x: {:?}, y: {})",
		x,
		y,
	);
}
