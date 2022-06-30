use crate::{Brainfuck, ParseError, Step};
use std::{collections::BTreeMap, io::Read, num::Wrapping};

impl Brainfuck {
	/// Parse and compile an ASCII brainfuck program.
	///
	/// # Example
	/// ```
	/// # use brainfrick::Brainfuck;
	/// # use std::io;
	/// // prints "hello world"
	/// static CODE: &str = "++++++++[>+++++++++++++>++++<<-]>.---.+++++++..+++.>.<++++++++.--------.+++.------.--------.";
	///
	/// let bf = Brainfuck::parse_ascii(CODE.as_bytes())?;
	/// let mut output = Vec::new();
	/// bf.run(io::empty(), &mut output)?;
	/// let output = String::from_utf8(output)?;
	///
	/// assert_eq!(output, "hello world");
	/// # Ok::<(), Box<dyn std::error::Error>>(())
	/// ```
	pub fn parse_ascii<R: Read>(code: R) -> Result<Brainfuck, ParseError> {
		let mut bf = Brainfuck {
			steps: Vec::new(),
			loop_indexes: BTreeMap::new(),
		};

		struct LoopStartIndex {
			/// index into `bf.steps`
			step_index: usize,

			/// index into `input.bytes()` for error messages
			byte_index: usize,
		}

		let mut stack: Vec<LoopStartIndex> = Vec::new();

		for (byte_index, result) in code.bytes().enumerate() {
			let byte = result?;

			let step = match byte {
				b'+' | b'-' => {
					let amount = Wrapping(if byte == b'+' { 1 } else { -1 });
					if let Some(Step::Add(prev_amount)) = bf.steps.last_mut() {
						*prev_amount += amount;
						continue;
					}
					Step::Add(amount)
				},

				b'>' | b'<' => {
					let amount = if byte == b'>' { 1 } else { -1 };
					if let Some(Step::Move(prev_amount)) = bf.steps.last_mut() {
						if let Some(new_amount) = prev_amount.checked_add(amount) {
							*prev_amount = new_amount;
							continue;
						}
					}
					Step::Move(amount)
				},

				b'[' => {
					stack.push(LoopStartIndex {
						step_index: bf.steps.len(),
						byte_index,
					});
					Step::LoopStart
				},

				b']' => {
					let end_index = bf.steps.len();
					match stack.pop() {
						Some(LoopStartIndex {
							step_index: start_index,
							..
						}) => {
							bf.loop_indexes.insert(start_index, end_index);
							bf.loop_indexes.insert(end_index, start_index);
						},
						None => return Err(ParseError::MissingBracket(byte_index)),
					}
					Step::LoopEnd
				},

				b'.' => Step::Output,
				b',' => Step::Input,

				#[cfg(feature = "debug-char")]
				b'?' => Step::Debug,

				_ => continue,
			};

			bf.steps.push(step);
		}

		if let Some(LoopStartIndex { byte_index, .. }) = stack.pop() {
			return Err(ParseError::MissingBracket(byte_index));
		}

		bf.steps.shrink_to_fit();
		Ok(bf)
	}
}

#[cfg(test)]
mod test {
	use crate::{Brainfuck, MANDELBROT};

	#[test]
	fn matched_brackets() -> anyhow::Result<()> {
		let bf = Brainfuck::parse_ascii(MANDELBROT)?;
		for (k, v) in bf.loop_indexes.iter() {
			assert_eq!(bf.loop_indexes[k], *v);
			assert_eq!(bf.loop_indexes[v], *k);
		}

		Ok(())
	}

	#[test]
	fn bracket_position() -> anyhow::Result<()> {
		let bf = Brainfuck::parse_ascii("[>>>[><+_]][]".as_bytes())?;
		let indexes = bf.loop_indexes;

		assert_eq!(indexes[&0], 6);
		assert!(indexes.get(&1).is_none());
		assert_eq!(indexes[&2], 5);
		assert!(indexes.get(&3).is_none());
		assert!(indexes.get(&4).is_none());
		assert_eq!(indexes[&5], 2);
		assert_eq!(indexes[&6], 0);
		assert_eq!(indexes[&7], 8);
		assert_eq!(indexes[&8], 7);

		Ok(())
	}
}
