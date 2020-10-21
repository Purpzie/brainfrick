use super::{
    error::{Error, ErrorKind},
    mem::Memory,
    step::Step,
    Brainfuck,
};
use std::sync::Arc;

pub(crate) fn real_execute(brainfuck: &Brainfuck, input: Option<&str>) -> Result<String, Error> {
    let mut input = input.unwrap_or("").bytes();
    let mut output: Vec<u8> = Vec::new();
    let mut mem = Memory::new();
    let mut index: usize = 0;
    let mut step_count: usize = 0;

    while index < brainfuck.steps.len() {
        match brainfuck.steps[index] {
            Step::Add(n) => mem.add(n),
            Step::Move(n) => mem.move_pointer(n),
            Step::Output => output.push(mem.get_cell()),
            Step::Input => mem.set_cell(input.next().unwrap_or(0)),
            Step::Loop(kind, jump) if kind.should_jump(&mem) => index = jump,
            Step::Debug => mem.append_debug(&mut output),
            _ => (), // skipped loops
        }

        index += 1;
        step_count += 1;
        if step_count >= brainfuck.max_steps {
            return Err(Error {
                kind: ErrorKind::MaxSteps,
                index: brainfuck.indexes[index],
                brainfuck: Arc::clone(&brainfuck.code),
                output: parse_bytes(output),
            });
        }
    }

    Ok(parse_bytes(output).unwrap_or_default())
}

// this func exists because Cow clones when using into_owned() & we want to avoid that when possible
fn parse_bytes(output: Vec<u8>) -> Option<String> {
    use std::borrow::Cow;
    if output.is_empty() {
        None
    } else {
        Some(match String::from_utf8_lossy(&output) {
            // safety: if the result is borrowed then it is always valid utf-8
            Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(output) },
            Cow::Owned(s) => s,
        })
    }
}
