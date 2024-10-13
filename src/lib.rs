use std::{fmt, marker::PhantomData};

#[cfg(test)]
mod tests;

/// Helper for writing string-separated iterator an existing Writer without intermediate
/// allocations
#[derive(Debug, Clone)]
pub struct FmtSeparated<I, T, S, F> {
	iter: I,
	separator: S,
	write_fn: F,
	phantom: PhantomData<T>,
}

impl<I, T, S> FmtSeparated<I, T, S, fn(&mut fmt::Formatter, T) -> fmt::Result>
where
	T: fmt::Display,
{
	pub fn new<J>(iter: J, separator: S) -> Self
	where
		J: IntoIterator<IntoIter = I>,
		I: Iterator<Item = T>,
	{
		fn write_identity<T>(w: &mut fmt::Formatter, x: T) -> fmt::Result
		where
			T: fmt::Display,
		{
			write!(w, "{x}")
		}

		FmtSeparated::new_with_fn(iter, write_identity, separator)
	}
}

impl<I, T, S, F> FmtSeparated<I, T, S, F> {
	pub fn new_with_fn<J>(iter: J, write_fn: F, separator: S) -> Self
	where
		J: IntoIterator<IntoIter = I>,
		F: Fn(&mut fmt::Formatter, T) -> fmt::Result,
	{
		Self {
			iter: iter.into_iter(),
			separator,
			write_fn,
			phantom: PhantomData,
		}
	}
}

impl<I, T, S, F> fmt::Display for FmtSeparated<I, T, S, F>
where
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
			for x in iter {
				write!(f, "{separator}")?;
				write_fn(f, x)?;
			}
		}

		Ok(())
	}
}
