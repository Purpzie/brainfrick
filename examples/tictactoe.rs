use brainfrick::Brainfuck;
use std::io;

static CODE: &str = include_str!("../tests/tictactoe.bf");

fn main() -> anyhow::Result<()> {
	let bf = Brainfuck::parse_ascii(CODE.as_bytes())?;
	bf.run(io::stdin(), io::stdout())?;
	Ok(())
}
