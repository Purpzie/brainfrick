/*!
    An optimizing Brainfuck interpreter.

    - For the full documentation, see [`Brainfuck`].
    - If you don't like swearing, `Brainfuck` is also re-exported as `Brainfrick`.

    # Example
    ```
    use brainfrick::Brainfuck;

    let purpzie_sucks = Brainfuck::execute("
        ++++++++[>++++++++++<-]>.<++[>++++++++++<-]+++[>+++++<-]>+
        +.---.--.++++++++++.<++[>----------<-]>+++.----.<+++++++[>
        ----------<-]>+.<++++++++[>++++++++++<-]>+++.++.<+++[>----
        --<-]>.++++++++.++++++++.<++++++++[>----------<-]>--.
    ")?;

    assert_eq!(purpzie_sucks, "Purpzie sucks!");
    # Ok::<(), brainfrick::Error>(())
    ```
*/

#![allow(clippy::match_bool)]
#![doc(html_root_url = "https://docs.rs/brainfrick/1.1.2")]

mod error;
mod execute;
mod mem;
mod parse;
mod step;

use execute::real_execute;
use parse::real_parse;
use std::sync::Arc;

pub use error::{Error, ErrorKind};

/**
    A struct that parses and runs brainfuck.

    # Example
    ```
    use brainfrick::Brainfuck;

    let purpzie_sucks = Brainfuck::execute("
        ++++++++[>++++++++++<-]>.<++[>++++++++++<-]+++[>+++++<-]>+
        +.---.--.++++++++++.<++[>----------<-]>+++.----.<+++++++[>
        ----------<-]>+.<++++++++[>++++++++++<-]>+++.++.<+++[>----
        --<-]>.++++++++.++++++++.<++++++++[>----------<-]>--.
    ")?;

    assert_eq!(purpzie_sucks, "Purpzie sucks!");
    # Ok::<(), brainfrick::Error>(())
    ```

    ## The debug character
    To aid whoever is crazy enough to write in brainfuck, the question mark `?` will output the
    current cell number and value.

    ```
    # use brainfrick::Brainfuck;
    let where_am_i = Brainfuck::execute(">>+++++?")?;
    assert_eq!(where_am_i, "[2,5]");
    # Ok::<(), brainfrick::Error>(())
    ```

    ## Memory details
    Memory is infinite in both directions. In order to prevent malicious brainfuck from running
    forever, there is a configurable, maximum number of 'steps' you can allow to be executed before
    stopping.

    ```
    # use brainfrick::Brainfuck;
    let mut infinite = Brainfuck::parse("+[>+]")?;
    infinite.max_steps = 100;
    let result = infinite.run();

    assert!(result.is_err());
    # Ok::<(), brainfrick::Error>(())
    ```

    ## Testing multiple inputs
    If you'd like to quickly test the same brainfuck with many different inputs, you can
    parse it beforehand to speed up the process. See [`Brainfuck::input`] for information.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brainfuck {
    steps: Vec<step::Step>,
    indexes: Vec<error::Index>,
    // this is an Arc so that errors don't need to clone it
    code: Arc<String>,
    /// The maximum number of 'steps' that will be run before stopping. Defaults to
    /// [`Brainfuck::MAX_STEPS`].
    pub max_steps: usize,
}

impl Brainfuck {
    /// The default maximum number of 'steps' that will be run before stopping.
    pub const MAX_STEPS: usize = 100_000;

    /**
        Run some brainfuck.

        # Errors
        May return an [`Error`] of kind [`UnmatchedBracket`](ErrorKind::UnmatchedBracket) or
        [`MaxSteps`](ErrorKind::MaxSteps).

        # Example
        ```
        # use brainfrick::Brainfuck;
        let purpzie_sucks = Brainfuck::execute("
            ++++++++[>++++++++++<-]>.<++[>++++++++++<-]+++[>+++++<-]>+
            +.---.--.++++++++++.<++[>----------<-]>+++.----.<+++++++[>
            ----------<-]>+.<++++++++[>++++++++++<-]>+++.++.<+++[>----
            --<-]>.++++++++.++++++++.<++++++++[>----------<-]>--.
        ")?;

        assert_eq!(purpzie_sucks, "Purpzie sucks!");
        # Ok::<(), brainfrick::Error>(())
        ```
    */
    pub fn execute<S: Into<String>>(code: S) -> Result<String, Error> {
        Self::parse(code)?.run()
    }

    /**
        Run some brainfuck with input.

        # Errors
        May return an [`Error`] of kind [`UnmatchedBracket`](ErrorKind::UnmatchedBracket) or
        [`MaxSteps`](ErrorKind::MaxSteps).

        # Example
        ```
        # use brainfrick::Brainfuck;
        let loud = Brainfuck::execute_with_input(
            ",[>++++[<-------->-]<.,]",
            "foobar",
        )?;

        assert_eq!(loud, "FOOBAR");
        # Ok::<(), brainfrick::Error>(())
        ```
    */
    pub fn execute_with_input<O: Into<String>, R: AsRef<str>>(
        code: O,
        input: R,
    ) -> Result<String, Error> {
        Self::parse(code)?.input(input)
    }

    /**
        Parse some brainfuck for later use.

        This essentially just creates a [`Brainfuck`] struct.

        # Errors
        May return an [`UnmatchedBracket`](ErrorKind::UnmatchedBracket) error.

        # Example
        ```
        # use brainfrick::Brainfuck;
        let purpzie = Brainfuck::parse("
            ++++++++[>++++++++++<-]>.<++[>++++++++++<-]+++[>+++++<-]>+
            +.---.--.++++++++++.<++[>----------<-]>+++.----.<+++++++[>
            ----------<-]>+.<++++++++[>++++++++++<-]>+++.++.<+++[>----
            --<-]>.++++++++.++++++++.<++++++++[>----------<-]>--.
        ")?;

        // ...later

        let sucks = purpzie.run()?;

        assert_eq!(sucks, "Purpzie sucks!");
        # Ok::<(), brainfrick::Error>(())
        ```
    */
    pub fn parse<S: Into<String>>(code: S) -> Result<Brainfuck, Error> {
        real_parse(code.into(), Self::MAX_STEPS)
    }

    /**
        Run the parsed brainfuck.

        Note that, for a single [`Brainfuck`], this will always output the same result.

        # Errors
        May return a [`MaxSteps`](ErrorKind::MaxSteps) error.

        # Example
        ```
        # use brainfrick::Brainfuck;
        let purpzie = Brainfuck::parse("
            ++++++++[>++++++++++<-]>.<++[>++++++++++<-]+++[>+++++<-]>+
            +.---.--.++++++++++.<++[>----------<-]>+++.----.<+++++++[>
            ----------<-]>+.<++++++++[>++++++++++<-]>+++.++.<+++[>----
            --<-]>.++++++++.++++++++.<++++++++[>----------<-]>--.
        ")?;

        // ...later

        let sucks = purpzie.run()?;

        assert_eq!(sucks, "Purpzie sucks!");
        # Ok::<(), brainfrick::Error>(())
        ```
    */
    pub fn run(&self) -> Result<String, Error> {
        real_execute(self, None)
    }

    /**
        Run the parsed brainfuck with input.

        # Errors
        May return a [`MaxSteps`](ErrorKind::MaxSteps) error.

        # Example
        ```
        # use brainfrick::Brainfuck;
        let loud = Brainfuck::parse(",[>++++[<-------->-]<.,]")?;

        assert_eq!(loud.input("foobar")?, "FOOBAR");
        assert_eq!(loud.input("heck")?, "HECK");
        assert_eq!(loud.input("aaaaa")?, "AAAAA");
        # Ok::<(), brainfrick::Error>(())
        ```
    */
    pub fn input<S: AsRef<str>>(&self, input: S) -> Result<String, Error> {
        real_execute(self, Some(input.as_ref()))
    }

    /**
        The original brainfuck 'source'.

        # Example
        ```
        # use brainfrick::Brainfuck;
        let code = ",[>++++[<-------->-]<.,]";
        let example = Brainfuck::parse(code)?;
        assert_eq!(code, example.code());
        # Ok::<(), brainfrick::Error>(())
        ```
    */
    pub fn code(&self) -> &str {
        &self.code
    }
}

// for people who don't like swearing
pub use Brainfuck as Brainfrick;
