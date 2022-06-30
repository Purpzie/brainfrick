// many tests are from http://brainfuck.org/tests.b by Daniel B Cristofani

use brainfrick::{Brainfuck, RunOptions};
use std::{io, str};

#[test]
fn io() -> anyhow::Result<()> {
	let code: &[u8] = b">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<.";
	let bf = Brainfuck::parse_ascii(code)?;
	let mut output = Vec::new();
	bf.run("\n".as_bytes(), &mut output)?;
	assert_eq!(str::from_utf8(&output)?, "LB\nLB\n");
	Ok(())
}

#[test]
fn mem_size() -> anyhow::Result<()> {
	// goes to the 30,000th cell exactly
	let code: &[u8] =
		b"++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]+++++[>+++++++<<++>-]>.<<.";
	let bf = Brainfuck::parse_ascii(code)?;
	let mut output = Vec::new();
	let mut options = RunOptions::new().max_mem_bytes(30_000);
	bf.run_with(options.clone(), io::empty(), &mut output)?;
	assert_eq!(str::from_utf8(&output)?, "#\n");
	drop(output);
	options.max_mem_bytes = 29_999;
	let result = bf.run_with(options, io::empty(), io::sink());
	assert!(result.is_err());
	Ok(())
}

#[test]
fn obscure_problems() -> anyhow::Result<()> {
	let code: &[u8] =
		br#"[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]"A*$";@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."#;
	let bf = Brainfuck::parse_ascii(code)?;
	let mut output = Vec::new();
	bf.run(io::empty(), &mut output)?;
	assert_eq!(str::from_utf8(&output)?, "H\n");
	Ok(())
}

#[test]
#[should_panic(expected = "MissingBracket(25)")]
fn missing_left_bracket() {
	Brainfuck::parse_ascii(b"+++++[>+++++++>++<<-]>.>.[".as_slice()).unwrap();
}

#[test]
#[should_panic(expected = "MissingBracket(25)")]
fn missing_right_bracket() {
	Brainfuck::parse_ascii(b"+++++[>+++++++>++<<-]>.>.][".as_slice()).unwrap();
}

#[test]
fn rot13() -> anyhow::Result<()> {
	let code: &[u8] = include_bytes!("./rot13.bf");
	let bf = Brainfuck::parse_ascii(code)?;
	let mut output = Vec::new();
	let input = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()";
	let expected_output = "nopqrstuvwxyzabcdefghijklmNOPQRSTUVWXYZABCDEFGHIJKLM!@#$%^&*()";
	bf.run(input.as_bytes(), &mut output)?;
	assert_eq!(str::from_utf8(&output)?, expected_output);
	Ok(())
}

#[test]
fn purpzie_sucks() -> anyhow::Result<()> {
	let code: &[u8] = include_bytes!("./purpzie_sucks.bf");
	let bf = Brainfuck::parse_ascii(code)?;
	let mut output = Vec::new();
	bf.run(io::empty(), &mut output)?;
	assert_eq!(str::from_utf8(&output)?, "Purpzie sucks!");
	Ok(())
}

#[test]
fn debug_char() -> anyhow::Result<()> {
	let code: &[u8] = b"?+?>?+?";
	let bf = Brainfuck::parse_ascii(code)?;
	let mut output = Vec::new();
	bf.run(io::empty(), &mut output)?;
	if cfg!(feature = "debug-char") {
		assert_eq!(str::from_utf8(&output)?, "(0:0)(0:1)(1:0)(1:1)");
	} else {
		assert!(output.is_empty());
	}
	Ok(())
}
