//! A simple usage example.

use std::io;

use brainfuck_rs::{
	engine::{Engine, RuntimeSettings},
	instruction::Instruction,
	token::Token,
	utils::StripShebang,
};

const HELLO_WORLD: &str = include_str!("brainfuck-programs/hello-world.b");

fn main() {
	let mut bf = Engine::default();
	let settings = RuntimeSettings::default();

	let instructions = Instruction::parse(Token::tokenize(HELLO_WORLD.strip_shebang())).unwrap();

	let mut input = io::stdin();
	let mut output = io::stdout();

	bf.run(&instructions, &mut input, &mut output, settings)
		.unwrap();
}
