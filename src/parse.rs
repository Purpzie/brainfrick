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

pub(crate) fn parse(code: String, max_steps: usize) -> Result<Brainfuck, Error> {
    let code = Arc::new(code);

    let indexed_steps: Vec<IndexedStep> = code
        .lines()
        .enumerate()
        .flat_map(|(line, content)| {
            content.bytes().enumerate().filter_map(move |(col, byte)| {
                // now we have both the line and col
                let index = Index::new(line, col);
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
                .map(|step| IndexedStep { step, index })
            })
        })
        // optimization by combining repeated steps
        .coalesce(|left, right| {
            match (left.step, right.step) {
                (Step::Add(a), Step::Add(b)) => Some(Step::Add(a.wrapping_add(b))),
                (Step::Move(a), Step::Move(b)) => {
                    Some(Step::Move(a.checked_add(b).expect("Pointer overflow")))
                }
                _ => None,
            }
            .map_or(Err((left, right)), |combined| {
                Ok(IndexedStep {
                    step: combined,
                    index: left.index,
                })
            })
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
