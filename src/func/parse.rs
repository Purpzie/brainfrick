use super::{
    error::{Error, ErrorKind::UnmatchedBracket, Index},
    step::{
        Loop::*,
        Step::{self, *},
    },
    Brainfuck,
};

use itertools::Itertools;
use std::sync::Arc;

pub(crate) fn parse(code: String, max_steps: usize) -> Result<Brainfuck, Error> {
    let code = Arc::new(code);
    let mut indexes = Vec::with_capacity(code.len());

    // in order for helpful errors to point out where they came from, every step must be stored with its corresponding index
    #[rustfmt::skip]
    let mut steps: Vec<Step> = code.lines().enumerate()
        .flat_map(|(line, content)| content.bytes().enumerate()
        .filter_map(move |(col, byte)|
            match byte {
                b'+' => Some(Add(1)),
                b'-' => Some(Add(-1)),
                b'>' => Some(Move(1)),
                b'<' => Some(Move(-1)),
                b'.' => Some(Output),
                b',' => Some(Input),
                b'?' => Some(Debug),
                b'[' => Some(Loop(Begin, 0)),
                b']' => Some(Loop(End, 0)),
                _ => None,
            }
            .map(|step| (step, Index::new(line, col)))
        ))
        // optimization by combining repeated steps
        .coalesce(|left, right|
            match (left.0, right.0) {
                (Add(a), Add(b)) => Some(Add(a.wrapping_add(b))),
                (Move(a), Move(b)) => Some(Move(a.checked_add(b).unwrap())),
                _ => None,
            }
            .map_or(
                Err((left, right)),
                |step| Ok((step, left.1)),
            )
        )
        // separate indexes from steps
        .map(|(step, index)| {
            indexes.push(index);
            step
        })
        .collect();

    indexes.shrink_to_fit();

    // match loops together
    let mut brackets: Vec<(usize, &mut usize)> = Vec::with_capacity(steps.len());
    for (i, bracket) in steps.iter_mut().enumerate() {
        match bracket {
            Loop(Begin, jump) => brackets.push((i, jump)),
            Loop(End, jump) => match brackets.pop() {
                Some((matching_i, matching_jump)) => {
                    *jump = matching_i;
                    *matching_jump = i;
                }
                None => {
                    return Err(Error {
                        kind: UnmatchedBracket,
                        index: indexes[i],
                        brainfuck: code,
                        output: None,
                    })
                }
            },
            _ => (),
        }
    }

    // make sure they all got matched
    match brackets.pop() {
        None => Ok(Brainfuck {
            steps,
            indexes,
            code,
            max_steps,
        }),
        Some((i, _)) => Err(Error {
            kind: UnmatchedBracket,
            index: indexes[i],
            brainfuck: code,
            output: None,
        }),
    }
}

#[cfg(test)]
#[test]
fn test() {
    let p = parse("+[+++++>>>++<<-]".to_string(), 0).unwrap();
    assert_eq!(
        p.steps,
        vec![
            Add(1),
            Loop(Begin, 7),
            Add(5),
            Move(3),
            Add(2),
            Move(-2),
            Add(-1),
            Loop(End, 1),
        ]
    );
}
