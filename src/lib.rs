#![doc = include_str!("../README.md")]
#![cfg_attr(docs_rs, feature(doc_auto_cfg))]
#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

mod error;
mod parse;
mod run;
pub use crate::{error::*, run::RunOptions};

use std::{collections::BTreeMap, num::Wrapping};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
	Add(Wrapping<i8>),
	Move(i8),
	LoopStart,
	LoopEnd,
	Output,
	Input,

	#[cfg(feature = "debug-char")]
	Debug,
}

/// A precompiled brainfuck program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brainfuck {
	steps: Vec<Step>,
	loop_indexes: BTreeMap<usize, usize>,
}

// this is huge, so only include it once here for all tests
#[cfg(test)]
static MANDELBROT: &[u8] = include_bytes!("../tests/mandelbrot.bf");
