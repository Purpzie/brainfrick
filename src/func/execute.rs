use super::{
    error::{Error, ErrorKind},
    mem::Memory,
    step::Step::*,
    Brainfuck,
};

pub(crate) fn execute(brainfuck: &Brainfuck, input: Option<&str>) -> Result<String, Error> {
    let mut output: Vec<u8> = Vec::new();
    let mut mem = Memory::new();
    let mut index: usize = 0;
    let mut step_count: usize = 0;

    let mut input = match input {
        Some(s) => s,
        None => "",
    }
    .bytes();

    while index < brainfuck.steps.len() {
        match brainfuck.steps[index] {
            Add(n) => mem.add(n),
            Move(n) => mem.move_pointer(n),
            Output => output.push(mem.cell()),
            Input => mem.set_cell(input.next().unwrap_or(0)),
            Loop(kind, jump) if kind.should_jump(&mem) => index = jump,
            Debug => {
                let (pointer, positive) = mem.pointer();
                let s = format!(
                    "[{}{},{}]",
                    if positive { "" } else { "-" },
                    pointer,
                    mem.cell()
                );
                output.append(&mut s.into_bytes());
            }
            _ => (), // skipped loops
        }

        index += 1;
        step_count += 1;
        if step_count == brainfuck.max_steps {
            return Err(Error::new(
                ErrorKind::MaxSteps,
                brainfuck.indexes[index],
                brainfuck.code.clone(), // Arc
                make_output(output),
            ));
        }
    }

    Ok(make_output(output).unwrap_or_default())
}

// this function exists because Cow clones when using into_owned(), and we want to avoid that when possible
#[inline]
fn make_output(output: Vec<u8>) -> Option<String> {
    use std::borrow::Cow;
    if output.is_empty() {
        None
    } else {
        Some(match String::from_utf8_lossy(&output) {
            // safety: if the result is borrowed it is always valid utf-8
            Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(output) },
            Cow::Owned(s) => s,
        })
    }
}
