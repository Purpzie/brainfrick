use brainfrick::{Brainfuck, Error, ErrorKind};
type Result<Ok = (), Err = Error> = std::result::Result<Ok, Err>;

// Big thanks to http://www.hevanet.com/cristofd/brainfuck for many of these tests

#[test]
fn example() -> Result {
    let r = Brainfuck::execute(include_str!("./purpzie_sucks.bf"))?;
    assert_eq!(r, "Purpzie sucks!");
    Ok(())
}

#[test]
fn basic_io() -> Result {
    let r = Brainfuck::parse(">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.")?
        .input("\n")?;

    assert_eq!(r, "LB\nLB\n");
    Ok(())
}

#[test]
fn big_array_right() -> Result {
    // goes to cell 30k
    let mut r = Brainfuck::parse(include_str!("./array_size_right.bf"))?;
    r.max_steps = 14_000_000;
    let r = r.run()?;

    assert_eq!(r, "#\n");
    Ok(())
}

#[test]
fn big_array_left() -> Result {
    // goes to cell -30k
    let mut r = Brainfuck::parse(include_str!("./array_size_left.bf"))?;
    r.max_steps = 14_000_000;
    let r = r.run()?;

    assert_eq!(r, "#\n");
    Ok(())
}

#[test]
fn obscure() -> Result {
    // had to remove the '?' in this example
    let r = Brainfuck::execute(
        "
        []++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]
        \"A*$\";@![#>>+<<]>[>>]<<<<[>++<[-]]>.>.
        ",
    )?;

    assert_eq!(r, "H\n");
    Ok(())
}

#[test]
fn unmatched_left_bracket() -> Result {
    let r = Brainfuck::parse("+++++[>+++++++>++<<-]>.>.[-----");
    match r {
        Ok(..) => panic!("Unmatched left bracket was not caught"),
        Err(e) => match e.kind() {
            ErrorKind::UnmatchedBracket => {
                assert!(e.line() == 0, "Expected line 0, got {}", e.line());
                assert!(e.col() == 25, "Expected col 25, got {}", e.col());
                Ok(())
            }
            kind => panic!("Wrong error kind: {:?}", kind),
        },
    }
}

#[test]
fn unmatched_right_bracket() -> Result {
    let r = Brainfuck::parse("+++++[>+++++++>++<<-]>.>.][");

    match r {
        Ok(..) => panic!("Unmatched right bracket was not caught"),
        Err(e) => match e.kind() {
            ErrorKind::UnmatchedBracket => {
                assert!(e.line() == 0, "Expected line 0, got {}", e.line());
                assert!(e.col() == 25, "Expected col 25, got {}", e.col());
                Ok(())
            }
            kind => panic!("Wrong error kind: {:?}", kind),
        },
    }
}

#[test]
fn max_steps() -> Result {
    match Brainfuck::execute("+[]") {
        Ok(_) => panic!("What?"),
        Err(e) => match e.kind() {
            ErrorKind::MaxSteps => Ok(()),
            kind => panic!("Wrong error kind: {:?}", kind),
        },
    }
}

#[test]
fn deep_nested_brackets() -> Result {
    let r = Brainfuck::parse(include_str!("./rot13.bf"))?.input("~mlk zyx")?;

    assert_eq!(r, "~zyx mlk");
    Ok(())
}

#[test]
fn num_warp() -> Result {
    let r = Brainfuck::parse(include_str!("./numwarp.bf"))?.input("1234")?;

    assert_eq!(r, include_str!("./1234.txt").replace("\r\n", "\n"));
    Ok(())
}
