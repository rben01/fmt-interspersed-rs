use std::fmt;

#[cfg(test)]
mod tests;

/// Helper for writing string-separated iterator an existing Writer without intermediate
/// allocations
#[derive(Debug, Clone)]
pub struct FmtInterspersed<I, S, F> {
	iter: I,
	separator: S,
	write_fn: F,
}

impl<I, T, S> FmtInterspersed<I, S, fn(&mut fmt::Formatter, T) -> fmt::Result>
where
	T: fmt::Display,
{
	pub fn new<J>(iter: J, separator: S) -> Self
	where
		J: IntoIterator<IntoIter = I>,
		I: Iterator<Item = T>,
	{
		FmtInterspersed::new_with_fn(iter, |f, x| write!(f, "{}", x), separator)
	}
}

impl<I, S, F> FmtInterspersed<I, S, F> {
	pub fn new_with_fn<T, J>(iter: J, write_fn: F, separator: S) -> Self
	where
		J: IntoIterator<IntoIter = I>,
		I: Iterator<Item = T>,
		F: Fn(&mut fmt::Formatter, T) -> fmt::Result,
	{
		Self {
			iter: iter.into_iter(),
			separator,
			write_fn,
		}
	}
}

impl<I, T, S, F> fmt::Display for FmtInterspersed<I, S, F>
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
		} = self;
		let mut iter = iter.clone();

		if let Some(x) = iter.next() {
			write_fn(f, x)?;
			for x in iter {
				write!(f, "{}", separator)?;
				write_fn(f, x)?;
			}
		}

		Ok(())
	}
}
