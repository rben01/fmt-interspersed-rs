use crate::StringIntersperser;
use std::fmt;

#[test]
fn test_identity() {
	fn test_case<T, S>(expected: &str, v: Vec<T>, separator: S)
	where
		T: fmt::Display,
		S: fmt::Display,
	{
		assert_eq!(
			format!("{}", StringIntersperser::new(&v, &separator)),
			expected
		);
		assert_eq!(
			format!(
				"{}",
				StringIntersperser::new_with_fn(&v, |f, x| write!(f, "{x}"), &separator)
			),
			expected
		);
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
fn test_functions() {
	fn test_case<T, S>(
		expected: &str,
		v: Vec<T>,
		f: impl Fn(&mut fmt::Formatter, &T) -> fmt::Result,
		separator: S,
	) where
		S: fmt::Display,
	{
		assert_eq!(
			format!("{}", StringIntersperser::new_with_fn(&v, f, separator)),
			expected
		);
	}

	test_case("", Vec::<i32>::new(), |f, _| write!(f, "nonempty"), "");
	test_case(
		"",
		Vec::<i32>::new(),
		|f, _| write!(f, "nonempty"),
		"nonempty",
	);

	test_case(
		"149",
		vec![1_i32, 2, 3],
		|f, i| write!(f, "{}", i.pow(2)),
		"",
	);
	test_case(
		"1, 4, 9",
		vec![1_i32, 2, 3],
		|f, i| write!(f, "{}", i.pow(2)),
		", ",
	);
	test_case(
		"10409",
		vec![1_i32, 2, 3],
		|f, i| write!(f, "{}", i.pow(2)),
		0,
	);

	test_case(
		r#"(x: "a", y: 1); (x: "b", y: 2); (x: "c", y: 3)"#,
		vec![("a", 1), ("b", 2), ("c", 3)],
		|f, (x, y)| write!(f, "(x: {x:?}, y: {y})"),
		"; ",
	);
}
