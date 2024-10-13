use std::{fmt, marker::PhantomData};

/// Helper for writing string-separated iterator an existing Writer without intermediate
/// allocations
pub struct StringIntersperser<I, T, S, F> {
	iter: I,
	separator: S,
	write_fn: F,
	phantom: PhantomData<T>,
}

impl<I, T, S> StringIntersperser<I, T, S, fn(&mut fmt::Formatter, T) -> fmt::Result>
where
	T: fmt::Display,
{
	pub fn new<J>(iter: J, separator: S) -> Self
	where
		J: IntoIterator<IntoIter = I>,
	{
		fn write_identity<T>(w: &mut fmt::Formatter, x: T) -> fmt::Result
		where
			T: fmt::Display,
		{
			write!(w, "{x}")
		}

		StringIntersperser::new_with_fn(iter, write_identity, separator)
	}
}

impl<I, T, S, F> StringIntersperser<I, T, S, F> {
	pub fn new_with_fn<J>(iter: J, write_fn: F, separator: S) -> Self
	where
		J: IntoIterator<IntoIter = I>,
	{
		Self {
			iter: iter.into_iter(),
			separator,
			write_fn,
			phantom: PhantomData,
		}
	}
}

impl<I, T, S, F> fmt::Display for StringIntersperser<I, T, S, F>
where
	T: fmt::Display,
	S: fmt::Display,
	I: Iterator<Item = T> + Clone,
	F: Fn(&mut fmt::Formatter, T) -> fmt::Result,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let Self {
			iter,
			separator,
			write_fn,
			phantom: _,
		} = self;
		let mut iter = iter.clone();

		if let Some(x) = iter.next() {
			write_fn(f, x)?;
		}
		for x in iter {
			write!(f, "{separator}")?;
			write_fn(f, x)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test() {
		fn test_case<T>(expected: &str, v: Vec<T>, separator: &str)
		where
			T: fmt::Display + Clone,
		{
			assert_eq!(
				format!("{}", StringIntersperser::new(v, separator)),
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
	}
}
