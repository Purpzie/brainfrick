use crate::{Brainfuck, RunError, Step};
use std::{
	io::{Read, Write},
	num::Wrapping,
};

/// Options for [`Brainfuck::run_with`].
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct RunOptions {
	/// The maximum allowed size of the memory tape.
	///
	/// Defaults to [`usize::MAX`].
	pub max_mem_bytes: usize,

	/// The maximum number of 'steps' to run.
	///
	/// A 'step' is one loop of the interpreter, which may represent multiple brainfuck instructions.
	/// The exact count may change in future releases as new optimizations are found. The main purpose
	/// of this field is to prevent infinite loops in user-provided brainfuck code.
	///
	/// Defaults to [`usize::MAX`].
	pub max_step_count: usize,
}

impl Default for RunOptions {
	fn default() -> Self {
		Self {
			max_mem_bytes: usize::MAX,
			max_step_count: usize::MAX,
		}
	}
}

impl RunOptions {
	/// Create the default [`RunOptions`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Builder pattern for [`max_mem_bytes`](RunOptions::max_mem_bytes).
	pub fn max_mem_bytes(mut self, max_mem_bytes: usize) -> Self {
		self.max_mem_bytes = max_mem_bytes;
		self
	}

	/// Builder pattern for [`max_step_count`](RunOptions::max_step_count).
	pub fn max_step_count(mut self, max_step_count: usize) -> Self {
		self.max_step_count = max_step_count;
		self
	}
}

impl Brainfuck {
	/// Execute this brainfuck program with the default [`RunOptions`].
	///
	/// See [`run_with`](Brainfuck::run_with) for more information.
	pub fn run<R, W>(&self, input: R, output: W) -> Result<(), RunError>
	where
		R: Read,
		W: Write,
	{
		self.run_with(RunOptions::default(), input, output)
	}

	/// Execute this brainfuck program with custom [`RunOptions`].
	///
	/// Reading and writing are *not* buffered. If you want them to be, wrap your types in
	/// [`BufReader`](std::io::BufReader) and [`BufWriter`](std::io::BufWriter) respectively.
	pub fn run_with<R, W>(
		&self,
		options: RunOptions,
		input: R,
		mut output: W,
	) -> Result<(), RunError>
	where
		R: Read,
		W: Write,
	{
		let mut input = input.bytes();
		let mut step_index: usize = 0;
		let mut step_count: usize = 0;
		let mut pointer: usize = 0;
		let mut tape = vec![Wrapping(0)];

		while let Some(&step) = self.steps.get(step_index) {
			step_count += 1;
			if step_count > options.max_step_count {
				return Err(RunError::StepLimit(options.max_step_count));
			}

			match step {
				Step::Add(amount) => tape[pointer] += Wrapping(amount.0 as u8),

				Step::Move(amount) => {
					let abs = amount.unsigned_abs() as usize;
					if amount > 0 {
						pointer += abs;
						if pointer >= tape.len() {
							if pointer < options.max_mem_bytes {
								tape.resize(pointer + 1, Default::default());
							} else {
								return Err(RunError::MemoryLimit(options.max_mem_bytes));
							}
						}
					} else if let Some(new_pointer) = pointer.checked_sub(abs) {
						pointer = new_pointer;
					} else {
						return Err(RunError::NegativePointer);
					}
				},

				Step::LoopStart | Step::LoopEnd => {
					if (step == Step::LoopStart) == (tape[pointer].0 == 0) {
						step_index = self.loop_indexes[&step_index];
					}
				},

				Step::Output => output.write_all(&[tape[pointer].0])?,

				Step::Input => tape[pointer].0 = input.next().transpose()?.unwrap_or_default(),

				#[cfg(feature = "debug-char")]
				Step::Debug => write!(output, "({pointer}:{cell})", cell = tape[pointer].0)?,
			}

			step_index += 1;
		}

		Ok(())
	}
}
