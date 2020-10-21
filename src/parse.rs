use itertools::Itertools;
use std::sync::Arc;

use crate::{
    error::{Error, ErrorKind, Index},
    step::{LoopKind, Step},
    Brainfuck,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IndexedStep {
    step: Step,
    index: Index,
}

pub(crate) fn real_parse(code: String, max_steps: usize) -> Result<Brainfuck, Error> {
    let code = Arc::new(code);

    #[rustfmt::skip]
    let indexed_steps: Vec<IndexedStep> =
        code.lines().enumerate().flat_map(|(line_number, line_content)| {
            line_content.bytes().enumerate().filter_map(move |(col_number, byte)| {
                // all of that was just to get this index
                let index = Index::new(line_number, col_number);
                // now we interpret the byte...
                match byte {
                    b'+' => Some(Step::Add(1)),
                    b'-' => Some(Step::Add(-1)),
                    b'>' => Some(Step::Move(1)),
                    b'<' => Some(Step::Move(-1)),
                    b'.' => Some(Step::Output),
                    b',' => Some(Step::Input),
                    b'?' => Some(Step::Debug),
                    b'[' => Some(Step::Loop(LoopKind::Begin, 0)),
                    b']' => Some(Step::Loop(LoopKind::End, 0)),
                    _ => None,
                }
                // ...and store the index with it
                .map(|step| IndexedStep { step, index })
            })
        })
        // optimize by combining repeated steps
        // TODO: remove itertools dependency and do this yourself
        .coalesce(|left, right| {
            use Step::{Add, Move};
            match (left.step, right.step) {
                (Add(a), Add(b)) => Some(Add(a.wrapping_add(b))),
                (Move(a), Move(b)) => Some(
                    // TODO: document this or make it an error
                    // needs a gigantic string the size of half of the possible address space to
                    // panic, which is insane and not even possible on most systems
                    Move(a.checked_add(b).expect("Pointer overflow"))
                ),
                _ => None,
            }
            .map_or(
                Err((left, right)),
                |combined_step| Ok(IndexedStep { step: combined_step, index: left.index }),
            )
        })
        .collect();

    // split indexes from steps (makes everything easier)
    let mut steps = Vec::with_capacity(indexed_steps.len());
    let mut indexes = Vec::with_capacity(indexed_steps.len());
    for IndexedStep { step, index } in indexed_steps {
        steps.push(step);
        indexes.push(index);
    }

    // match loops together
    let mut open_brackets = Vec::with_capacity(steps.len());
    for (i, step) in steps.iter_mut().enumerate() {
        if let Step::Loop(kind, jump) = step {
            match kind {
                LoopKind::Begin => open_brackets.push((i, jump)),
                LoopKind::End => match open_brackets.pop() {
                    // make them refer to each other
                    Some((open_i, open_jump)) => {
                        *open_jump = i;
                        *jump = open_i;
                    }
                    None => {
                        return Err(Error {
                            kind: ErrorKind::UnmatchedBracket,
                            index: indexes[i],
                            brainfuck: code,
                            output: None,
                        })
                    }
                },
            }
        }
    }

    // make sure they all got matched
    match open_brackets.pop() {
        None => Ok(Brainfuck {
            steps,
            indexes,
            code,
            max_steps,
        }),
        Some((i, _)) => Err(Error {
            kind: ErrorKind::UnmatchedBracket,
            index: indexes[i],
            brainfuck: code,
            output: None,
        }),
    }
}
