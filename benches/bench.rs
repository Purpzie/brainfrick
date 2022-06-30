use brainfrick::Brainfuck;
use std::{io, time::Instant};

static CODE: &str = include_str!("../tests/mandelbrot.bf");

fn main() -> anyhow::Result<()> {
	let bf = Brainfuck::parse_ascii(CODE.as_bytes())?;

	let empty = io::empty();
	let stdout = io::stdout();

	let start = Instant::now();
	let result = bf.run(empty, stdout);
	let time = Instant::now() - start;

	println!("done in {time:?}");
	result?;
	Ok(())
}
