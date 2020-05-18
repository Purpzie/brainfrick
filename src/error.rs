use std::{fmt, sync::Arc};

/** An error originating from this crate. */
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    index: Index,
    brainfuck: Arc<String>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Index {
    line: usize,
    col: usize,
}

impl Index {
    #[inline]
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

impl Error {
    /// The [`kind`](ErrorKind) of error.
    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The line of brainfuck this error occurred on (starting from 0).
    #[inline]
    pub fn line(&self) -> usize {
        self.index.line
    }

    /// The column of brainfuck this error occurred on (starting from 0).
    #[inline]
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
    #[inline]
    pub fn brainfuck(&self) -> &str {
        &self.brainfuck
    }

    /// An alias for [`Error::brainfuck`].
    #[inline(always)]
    pub fn brainfrick(&self) -> &str {
        self.brainfuck()
    }

    #[inline]
    pub(crate) fn new(kind: ErrorKind, index: Index, brainfuck: Arc<String>) -> Self {
        Self {
            kind,
            index,
            brainfuck,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        f.write_str(match self {
            UnmatchedBracket => "Unmatched bracket",
            MaxSteps => "Max steps reached",
        })
    }
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // description and location
        let Index { line, col } = self.index;
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
        write!(f, "\n{0:>1$}", "^", offset)
    }
}
