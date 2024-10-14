#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_export]
macro_rules! write_interspersed {
	($writer:expr, $iter:expr, $separator:expr, $arg:pat_param => $($fmt:tt)*) => {{
		let mut iter = $iter.into_iter();
		if let ::core::option::Option::Some($arg) = iter.next() {
			write!($writer, $($fmt)*)?;
			for $arg in iter {
				write!($writer, "{}", $separator)?;
				write!($writer, $($fmt)*)?;
			}
		}
	}};
	($writer:expr, $iter:expr, $separator:expr $(,)?) => {
		$crate::write_interspersed!($writer, $iter, $separator, x => "{}", x)
	};
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! format_interspersed {
	($($args:tt)*) => {{
		let mut __buf = ::alloc::string::String::new();
		(|| -> ::core::fmt::Result {
			$crate::write_interspersed!(&mut __buf, $($args)*);
			::core::result::Result::Ok(())
		})().unwrap();

		__buf
	}};
}

#[macro_export]
macro_rules! writeln_interspersed {
	($writer:expr, $($args:tt)*) => {{
		$crate::write_interspersed!($writer, $($args)*);
		writeln!($writer)?;
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! print_interspersed {
	(print = $print:path; $iter:expr, $separator:expr, $arg:pat_param => $($fmt:tt)*) => {
		let mut iter = $iter.into_iter();
		if let ::core::option::Option::Some($arg) = iter.next() {
			$print!($($fmt)*);
			for $arg in iter {
				$print!("{}", $separator);
				$print!($($fmt)*);
			}
		}
	};
	(print = $print:path; $iter:expr, $separator:expr $(,)?) => {
		$crate::print_interspersed!(print = $print; $iter, $separator, x => "{}", x)
	};
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::print_interspersed!(print = ::std::print; $iter, $separator $(, $($args)*)?);
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! println_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::print_interspersed!(print = ::std::print; $iter, $separator $(, $($args)*)?);
		::std::println!();
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprint_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::print_interspersed!(print = ::std::eprint; $iter, $separator $(, $($args)*)?);
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprintln_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::print_interspersed!(print = ::std::eprint; $iter, $separator $(, $($args)*)?);
		::std::eprintln!();
	}};
}

pub mod prelude {
	pub use crate::{write_interspersed, writeln_interspersed};

	#[cfg(feature = "alloc")]
	pub use crate::format_interspersed;

	#[cfg(feature = "std")]
	pub use crate::{
		eprint_interspersed, eprintln_interspersed, print_interspersed, println_interspersed,
	};
}

#[cfg(test)]
mod test;
