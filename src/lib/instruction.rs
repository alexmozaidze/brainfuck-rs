use thiserror::Error;

use crate::token::Token;

/// Instructions that are executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
	/// `+`
	Inc,
	/// `-`
	Dec,
	/// `>`
	Next,
	/// `<`
	Prev,
	/// `.`
	Print,
	/// `.`
	Read,
	/// `[` and `]`
	Loop(Vec<Instruction>),
}

impl From<Token> for Instruction {
	fn from(token: Token) -> Self {
		match token {
			Token::Inc => Instruction::Inc,
			Token::Dec => Instruction::Dec,
			Token::Next => Instruction::Next,
			Token::Prev => Instruction::Prev,
			Token::Print => Instruction::Print,
			Token::Read => Instruction::Read,
			loop_token => panic!("attempt to convert {:?} into Instruction", loop_token),
		}
	}
}

impl Instruction {
	/// Get the inside of [`Instruction::Loop`]
	pub fn get_inner_mut(&mut self) -> Option<&mut Vec<Self>> {
		match self {
			Instruction::Loop(x) => Some(x),
			_ => None,
		}
	}

	/// Get the deepest [`Instruction::Loop`] inside a nested [`Instruction::Loop`].
	pub fn get_last_deepest_mut(&mut self, nesting: usize) -> &mut Self {
		let mut instruction_ref: &mut Instruction = self;

		for _ in 1..nesting {
			instruction_ref = instruction_ref.get_inner_mut().unwrap().last_mut().unwrap();
		}

		instruction_ref
	}

	/// Parse a sequence of [`Token`]s into a [`Vec`] of [`Instruction`]s.
	///
	/// # Usage
	///
	/// ```
	/// # use brainfuck_rs::{
	/// #   token::Token,
	/// #   instruction::Instruction,
	/// # };
	/// # let code = "+>>>>>>>>>>-[,+[-.----------[[-]>]<->]<]";
	/// let instructions: Vec<Instruction> = Instruction::parse(Token::tokenize(&code)).unwrap();
	/// ```
	///
	/// # Errors
	///
	/// It may error if there is unmatched loop start or loop end.
	pub fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Vec<Instruction>, ParseError> {
		let mut instructions: Vec<Instruction> = vec![];

		let mut nesting = 0;
		for token in tokens.into_iter() {
			match token {
				Token::LoopStart => {
					if nesting > 0 {
						instructions
							.last_mut()
							.unwrap()
							.get_last_deepest_mut(nesting)
							.get_inner_mut()
							.unwrap()
							.push(Instruction::Loop(vec![]));
					} else {
						instructions.push(Instruction::Loop(vec![]));
					}

					nesting += 1;
				}
				Token::LoopEnd => {
					if nesting == 0 {
						return Err(ParseError::UnmatchedLoopEnd);
					}

					nesting -= 1;
				}
				other => {
					if nesting > 0 {
						instructions
							.last_mut()
							.unwrap()
							.get_last_deepest_mut(nesting)
							.get_inner_mut()
							.unwrap()
							.push(other.into());
					} else {
						instructions.push(other.into());
					}
				}
			}
		}

		if nesting > 0 {
			return Err(ParseError::UnmatchedLoopStart);
		}

		Ok(instructions)
	}
}

/// An error that could be created if there is something wrong at the parsing stage.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
	/// Could not find match for `[`
	#[error("could not find match for `[`")]
	UnmatchedLoopStart,
	/// Could not find match for `]`
	#[error("could not find match for `]`")]
	UnmatchedLoopEnd,
}

#[cfg(test)]
mod tests {
	use super::*;

	mod deepest_loop_tests {
		use super::*;
		use lazy_static::lazy_static;

		lazy_static! {
			static ref INPUT_LOOP: Instruction = Instruction::Loop(vec![ // nesting = 1
				Instruction::Inc,
				Instruction::Loop(vec![/* should never reach here */]),
				Instruction::Dec,
				Instruction::Loop(vec![ // nesting = 2
					Instruction::Next,
					Instruction::Loop(vec![ // nesting = 3
						Instruction::Dec,
						Instruction::Dec,
					]),
				]),
			]);

			static ref EXPECTED_LOOP: Instruction = Instruction::Loop(vec![ // nesting = 1
				Instruction::Inc,
				Instruction::Loop(vec![/* should never reach here */]),
				Instruction::Dec,
				Instruction::Loop(vec![ // nesting = 2
					Instruction::Next,
					Instruction::Loop(vec![/* now this is empty */]),
				]),
			]);
		}

		#[test]
		fn correct_input() {
			let nesting = 3;
			let mut instruction = INPUT_LOOP.clone();

			instruction
				.get_last_deepest_mut(nesting)
				.get_inner_mut()
				.expect("could not get inner loop's contents")
				.clear();

			let expected = EXPECTED_LOOP.clone();

			assert_eq!(instruction, expected);
		}

		#[test]
		#[should_panic]
		fn excessive_nesting() {
			let nesting = 5;
			let mut instruction = INPUT_LOOP.clone();

			instruction
				.get_last_deepest_mut(nesting)
				.get_inner_mut()
				.expect("could not get inner loop's contents")
				.clear();

			let expected = EXPECTED_LOOP.clone();

			assert_eq!(instruction, expected);
		}

		#[test]
		#[should_panic]
		fn zero_nesting() {
			let nesting = 0;
			let mut instruction = INPUT_LOOP.clone();

			instruction
				.get_last_deepest_mut(nesting)
				.get_inner_mut()
				.expect("could not get inner loop's contents")
				.clear();

			let expected = EXPECTED_LOOP.clone();

			assert_eq!(instruction, expected);
		}
	}

	mod parser_tests {
		use super::*;

		#[test]
		fn unmatched_loop_end() {
			let program = r#"
                ++
                [
                    --><
                    oops! there was supposed to be an opening bracket here!
                        ++++.
                    ]
                ]
            "#;

			assert_eq!(
				ParseError::UnmatchedLoopEnd,
				Instruction::parse(Token::tokenize(&program)).unwrap_err()
			);
		}

		#[test]
		fn unmatched_loop_start() {
			let program = r#"
                ++
                [
                    --><
                    [
                        ++++.
                    oops! there was supposed to be a closing bracket here!
                ]
            "#;

			assert_eq!(
				ParseError::UnmatchedLoopStart,
				Instruction::parse(Token::tokenize(&program)).unwrap_err()
			);
		}

		#[test]
		fn nesting_handling() {
			let program = r#"
                ++
                [
                    --><
                    [
                        ++++.
                    ]
                ]
            "#;

			let expected = vec![
				Instruction::Inc,
				Instruction::Inc,
				Instruction::Loop(vec![
					Instruction::Dec,
					Instruction::Dec,
					Instruction::Next,
					Instruction::Prev,
					Instruction::Loop(vec![
						Instruction::Inc,
						Instruction::Inc,
						Instruction::Inc,
						Instruction::Inc,
						Instruction::Print,
					]),
				]),
			];

			let instructions: Vec<Instruction> =
				Instruction::parse(Token::tokenize(&program)).expect("parsing failed");

			assert_eq!(expected, instructions);
		}
	}
}
