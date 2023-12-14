use std::{
	io::{self, ErrorKind, Read, Write},
	num::Wrapping,
};

use crate::instruction::Instruction;

/// Contains the state of the program.
pub struct Engine {
	/// Current cursor/pointer index.
	pub pointer: usize,
	/// The tape that contains all the cells.
	pub tape: Vec<Wrapping<u8>>,
}

impl Engine {
	/// Shift pointer to the next cell or wraps around.
	pub fn next(&mut self) {
		if self.pointer == self.tape.len() - 1 {
			self.pointer = 0;
		} else {
			self.pointer += 1;
		}
	}

	/// Shift pointer to the previous cell or wraps around.
	pub fn prev(&mut self) {
		if self.pointer == 0 {
			self.pointer = self.tape.len() - 1;
		} else {
			self.pointer -= 1;
		}
	}

	/// Run Brainfuck instructions.
	///
	/// # Usage
	///
	/// ```
	/// # // HACK: trying to get stdio in test environment hangs tests
	/// # mod io {
	/// #   use std::io::{BufReader, BufWriter};
	/// #   pub fn stdin() -> BufReader<&'static [u8]> {
	/// #       BufReader::new(<&[u8]>::default())
	/// #   }
	/// #   pub fn stdout() -> BufWriter<Vec<u8>> {
	/// #       BufWriter::new(vec![])
	/// #   }
	/// # }
	/// # use brainfuck_rs::{
	/// #   instruction::Instruction,
	/// #   engine::{Engine, RuntimeSettings},
	/// #   token::Token,
	/// # };
	/// let mut bf = Engine::default();
	/// let settings = RuntimeSettings::default();
	///
	/// let code = "+>>>>>>>>>>-[,+[-.----------[[-]>]<->]<]";
	/// let tokens = Token::tokenize(&code);
	/// let instructions: Vec<Instruction> = Instruction::parse(tokens).unwrap();
	///
	/// let mut input = io::stdin();
	/// let mut output = io::stdout();
	///
	/// # return;
	/// bf.run(&instructions, &mut input, &mut output, settings);
	/// ```
	///
	/// You can also capture program's output and specify its input into a separate buffer by
	/// replacing [`io::stdin()`](`std::io::stdin`) and [`io::stdout()`](`std::io::stdout`) with [`BufReader`](`std::io::BufReader`) and [`BufWriter`](`std::io::BufWriter`)
	/// respectively.
	///
	/// ```
	/// # use std::io::{BufReader, BufWriter};
	/// let mut input = BufReader::new(b"Hello, World!".as_slice());
	/// let mut output = BufWriter::new(vec![]);
	/// ```
	///
	/// You can use any buffer, as long as it implements [`std::io::Write`] and [`std::io::Read`].
	///
	/// # Errors
	///
	/// In case of an IO error, it returns [`io::Error`] without continuing function execution.
	#[deny(clippy::unwrap_in_result, clippy::panic_in_result_fn)]
	pub fn run<'a, I>(
		&mut self,
		instructions: I,
		stdin: &mut impl Read,
		stdout: &mut impl Write,
		settings: RuntimeSettings,
	) -> Result<(), io::Error>
	where
		I: IntoIterator<Item = &'a Instruction>,
		I::IntoIter: DoubleEndedIterator,
	{
		let mut stack: Vec<&Instruction> = instructions.into_iter().rev().collect();

		while let Some(current_instruction) = stack.pop() {
			match current_instruction {
				Instruction::Loop(inner) => {
					if self.tape[self.pointer].0 != 0 {
						// NOTE: since we're executing in reverse order, we must push in reverse too
						stack.push(current_instruction);

						for inner_instruction in inner.iter().rev() {
							stack.push(inner_instruction);
						}
					}
				}
				Instruction::Inc => self.tape[self.pointer] += 1,
				Instruction::Dec => self.tape[self.pointer] -= 1,
				Instruction::Next => self.next(),
				Instruction::Prev => self.prev(),
				Instruction::Print => {
					let output = self.tape[self.pointer].0;

					stdout.write_all(&[output])?;

					if settings.should_flush {
						stdout.flush()?;
					}
				}
				Instruction::Read => {
					if !settings.should_flush {
						stdout.flush()?;
					}

					let mut input_char: [u8; 1] = [0];

					match stdin.read_exact(&mut input_char) {
						Ok(_) => {}
						Err(e) if settings.quit_on_eof && e.kind() == ErrorKind::UnexpectedEof => {
							return Ok(());
						}
						Err(e) if !settings.quit_on_eof && e.kind() == ErrorKind::UnexpectedEof => {
						}
						other_error => return other_error,
					}

					self.tape[self.pointer] = Wrapping(input_char[0]);
				}
			}
		}

		Ok(())
	}
}

impl Default for Engine {
	/// Creates a new `Engine` with default values:
	///
	/// ```
	/// # use std::num::Wrapping;
	/// # use brainfuck_rs::engine::Engine;
	/// Engine {
	///     pointer: 0,
	///     tape: vec![Wrapping(0); 30_000],
	/// }
	/// # ;
	/// ```
	fn default() -> Self {
		Self {
			pointer: 0,
			tape: vec![Wrapping(0); 30_000],
		}
	}
}

/// Settings that determine how interpreter should behave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSettings {
	/// If `true`, the output is flushed on every print instruction, otherwise the output is buffered.
	pub should_flush: bool,
	/// Stop execution on EOF.
	///
	/// Particularly usefull for environments with less control, like piped data and input buffers.
	pub quit_on_eof: bool,
}

impl Default for RuntimeSettings {
	/// Creates a new `RuntimeSettings` with default values:
	///
	/// ```
	/// # use brainfuck_rs::engine::RuntimeSettings;
	/// RuntimeSettings {
	///     should_flush: true,
	///     quit_on_eof: false,
	/// }
	/// # ;
	/// ```
	fn default() -> Self {
		Self {
			should_flush: true,
			quit_on_eof: false,
		}
	}
}

#[cfg(test)]
mod tests {
	use std::io::{BufReader, BufWriter};
	use std::str;

	use lazy_static::lazy_static;

	use crate::token::Token;
	use crate::utils::StripShebang;

	use super::*;

	lazy_static! {
		static ref HELLO_WORLD: &'static str =
			include_str!("../../examples/brainfuck-programs/hello-world.b").strip_shebang();
		static ref ROT13: &'static str =
			include_str!("../../examples/brainfuck-programs/rot13.b").strip_shebang();
	}

	#[test]
	fn non_stdout_buffer() {
		let mut bf = Engine::default();
		let settings = RuntimeSettings::default();

		let mut input = BufReader::new(<&[u8]>::default());
		let mut output = BufWriter::new(vec![]);

		let tokens = Token::tokenize(&HELLO_WORLD);
		let instructions = Instruction::parse(tokens).unwrap();

		bf.run(&instructions, &mut input, &mut output, settings)
			.unwrap();

		assert_eq!(
			"Hello World!\n",
			str::from_utf8(output.into_inner().unwrap().as_slice()).unwrap()
		);
	}

	#[test]
	fn non_stdin_buffer() {
		let mut bf = Engine::default();
		let settings = RuntimeSettings {
			quit_on_eof: true,
			..Default::default()
		};

		let mut input = BufReader::new(b"Hello, World!".as_slice());
		let mut output = BufWriter::new(vec![]);

		let tokens = Token::tokenize(&ROT13);
		let instructions = Instruction::parse(tokens).unwrap();

		bf.run(&instructions, &mut input, &mut output, settings)
			.unwrap();

		assert_eq!(
			"Uryyb, Jbeyq!",
			str::from_utf8(output.into_inner().unwrap().as_slice()).unwrap()
		);
	}
}
