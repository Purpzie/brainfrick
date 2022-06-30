use std::{
	error::Error,
	fmt::{self, Display},
	io,
};

/// An error that may occur when parsing brainfuck code.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
	/// The bracket at this byte index is missing a matching bracket.
	MissingBracket(usize),

	/// An [`io::Error`] occurred.
	Io(io::Error),
}

impl Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::MissingBracket(n) => write!(
				f,
				"parse error: missing matching bracket for byte index {n}"
			),
			Self::Io(err) => write!(f, "parse error: {err}"),
		}
	}
}

impl Error for ParseError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Io(err) => Some(err),
			_ => None,
		}
	}
}

impl From<io::Error> for ParseError {
	fn from(err: io::Error) -> Self {
		Self::Io(err)
	}
}

/// An error that may occur when executing brainfuck.
#[derive(Debug)]
#[non_exhaustive]
pub enum RunError {
	/// The memory limit defined in [`RunOptions`](crate::RunOptions) was reached.
	MemoryLimit(usize),

	/// The step limit defined in [`RunOptions`](crate::RunOptions) was reached.
	StepLimit(usize),

	/// The brainfuck pointer attempted to become negative.
	NegativePointer,

	/// An [`io::Error`] occurred.
	Io(io::Error),
}

impl Display for RunError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::MemoryLimit(n) => write!(f, "run error: memory limit reached ({n} bytes)"),
			Self::StepLimit(n) => write!(f, "run error: step limit reached ({n})"),
			Self::NegativePointer => write!(f, "run error: negative pointer"),
			Self::Io(err) => write!(f, "run error: {err}"),
		}
	}
}

impl Error for RunError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::Io(err) => Some(err),
			_ => None,
		}
	}
}

impl From<io::Error> for RunError {
	fn from(err: io::Error) -> Self {
		Self::Io(err)
	}
}
