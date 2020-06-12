use std::{fmt, sync::Arc};

/** An error originating from this crate. */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) index: Index,
    pub(crate) brainfuck: Arc<String>,
    pub(crate) output: Option<String>,
}

impl Error {
    /// The [`kind`](ErrorKind) of error.
    #[inline(always)]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The line of brainfuck this error occurred on (starting from 0).
    #[inline(always)]
    pub fn line(&self) -> usize {
        self.index.line
    }

    /// The column of brainfuck this error occurred on (starting from 0).
    #[inline(always)]
    pub fn col(&self) -> usize {
        self.index.col
    }

    /**
        The original, brainfuck 'source' this error came from.

        If you don't like swearing, you may use [`Error::brainfrick`].

        # Example
        ```
        # use brainfrick::Brainfuck;
        // this will run infinitely and therefore error
        let code = "+[++++]";
        let result = Brainfuck::execute(code).unwrap_err();
        assert_eq!(code, result.brainfuck());
        ```
    */
    #[inline(always)]
    pub fn brainfuck(&self) -> &str {
        &self.brainfuck
    }

    /// An alias for [`Error::brainfuck`].
    #[inline(always)]
    pub fn brainfrick(&self) -> &str {
        self.brainfuck()
    }

    /// The output produced before the error occurred, if applicable.
    #[inline(always)]
    pub fn output(&self) -> Option<&str> {
        self.output.as_deref()
    }
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Index { line, col } = self.index;

        // description and location
        writeln!(f, "{} @ line {} col {}", self.kind, line + 1, col + 1)?;

        // get snippet range (try to get 10 chars on either side)
        let line = self.brainfuck.lines().nth(line).expect("Invalid line");
        let (start, dots_begin, mut offset) = match col.checked_sub(10) {
            Some(num) => (num, num != 0, 11),
            None => (0, false, col + 1),
        };
        let mut end = col + 10;
        let dots_end = end < line.len();
        if !dots_end {
            end = line.len();
        }

        // write snippet
        if dots_begin {
            f.write_str("...")?;
            offset += 3;
        }
        f.write_str(&line[start..end])?;
        if dots_end {
            f.write_str("...")?;
        }

        // add ^ indicator
        write!(f, "\n{0:>1$}", "^", offset)
    }
}

/** The types of [`Errors`](Error) that can be encountered. */
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// A bracket `[]` was found without a matching bracket.
    UnmatchedBracket,
    /// The [maximum number of steps](crate::Brainfuck::max_steps) was reached.
    MaxSteps,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::UnmatchedBracket => "Unmatched bracket",
            Self::MaxSteps => "Max steps reached",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Index {
    pub line: usize,
    pub col: usize,
}

// alternate syntax to create
impl Index {
    #[inline(always)]
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
