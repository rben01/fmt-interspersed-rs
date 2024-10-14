#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_export]
macro_rules! macro_interspersed {
	(dst = $dst:expr; $iter:expr, $separator:expr, $arg:pat_param => $($fmt:tt)*) => {
		(|| -> core::fmt::Result {
			let mut iter = $iter.into_iter();
			if let ::core::option::Option::Some($arg) = iter.next() {
				write!($dst, $($fmt)*)?;
				for $arg in iter {
					write!($dst, "{}", $separator)?;
					write!($dst, $($fmt)*)?;
				}
			};
			::core::fmt::Result::Ok(())
		})()
	};
	(dst = $dst:expr; $iter:expr, $separator:expr $(,)?) => {
		$crate::macro_interspersed!(dst = $dst; $iter, $separator, x => "{}", x)
	};
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
		$crate::macro_interspersed!(print = $print; $iter, $separator, x => "{}", x)
	};
}

#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! format_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		let mut buf = ::alloc::string::String::new();
		$crate::macro_interspersed!(dst = &mut buf; $iter, $separator $(, $($args)*)?).unwrap();
		buf
	}};
}

#[macro_export]
macro_rules! write_interspersed {
	($writer:expr, $iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::macro_interspersed!(dst = $writer; $iter, $separator $(, $($args)*)?)
	}};
}

#[macro_export]
macro_rules! writeln_interspersed {
	($writer:expr, $iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::write_interspersed!($writer, $iter, $separator $(, $($args)*)?);
		writeln!($writer)?;
		::core::fmt::Result::Ok(())
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! print_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::macro_interspersed!(print = ::std::print; $iter, $separator $(, $($args)*)?);
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! println_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::macro_interspersed!(print = ::std::print; $iter, $separator $(, $($args)*)?);
		::std::println!();
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprint_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::macro_interspersed!(print = ::std::eprint; $iter, $separator $(, $($args)*)?);
	}};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprintln_interspersed {
	($iter:expr, $separator:expr $(, $($args:tt)*)?) => {{
		$crate::macro_interspersed!(print = ::std::eprint; $iter, $separator $(, $($args)*)?);
		::std::eprintln!();
	}};
}

#[cfg(test)]
mod tests;
