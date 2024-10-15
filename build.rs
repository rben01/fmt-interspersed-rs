use regex::Regex;
use std::{
	env, fmt, fs,
	io::{BufRead, BufReader, Write},
	path::PathBuf,
};

#[derive(Default)]
enum WriteKind {
	#[default]
	Fmt,
	Io,
}

impl fmt::Display for WriteKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Fmt => "fmt",
				Self::Io => "io",
			}
		)
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let in_file = BufReader::new(fs::File::open("README.md")?);

	let mut i = 0;

	let mut is_in_doctest = false;
	let mut current_doctest = String::new();

	let mut write_kind = WriteKind::default();

	let md_link_re =
		Regex::new(r"\[([^\]]*?)\]\((https://docs.rs/fmt-interspersed[^)]*?)\)").unwrap();

	let mut out_file =
		fs::File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("docs.md"))?;

	for line in in_file.lines() {
		let line = line?;
		if line == "<!-- begin -->" {
			if i >= 1 {
				writeln!(
					&mut out_file,
					r##"```rust
# extern crate alloc;
# extern crate std;
# use fmt_interspersed::prelude::*;
{}
# Ok::<_, std::{write_kind}::Error>(())
```"##,
					current_doctest.trim_matches('\n'),
				)?;

				current_doctest.clear();
			}
			i += 1;
			is_in_doctest = false;
		} else if line == "<!-- end -->" {
			is_in_doctest = true;
		} else if is_in_doctest {
			if !line.trim().starts_with("```") {
				current_doctest.push_str(&line);
				current_doctest.push('\n');

				if line.contains("::fmt::") {
					write_kind = WriteKind::Fmt;
				} else if line.contains("::io::") {
					write_kind = WriteKind::Io;
				}
			}
		} else {
			let line = md_link_re.replace_all(&line, "[$1]");
			writeln!(&mut out_file, "{line}")?;
		}
	}
	Ok(())
}
