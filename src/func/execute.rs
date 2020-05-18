use super::{
    error::{Error, ErrorKind},
    mem::Memory,
    step::Step::*,
    Brainfuck,
};

pub(crate) fn execute<I: Iterator<Item = u8>>(
    brainfuck: &Brainfuck,
    mut input: I,
) -> Result<String, Error> {
    let mut output: Vec<u8> = Vec::new();
    let mut mem = Memory::new();
    let mut index: usize = 0;
    let mut step_count: usize = 0;

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
            ));
        }
    }

    Ok(String::from_utf8_lossy(&output).into_owned())
}
