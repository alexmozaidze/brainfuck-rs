use std::io;
use std::{
    io::{Read, Write},
    num::Wrapping,
};

use thiserror::Error;

use crate::{brainfuck::Brainfuck, token::Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Inc,
    Dec,
    Next,
    Prev,
    Print,
    Read,
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
            _ => panic!("attempt to convert Token into Instruction"),
        }
    }
}

#[allow(dead_code)]
impl Instruction {
    #[allow(dead_code)]
    pub fn get_loop(&self) -> Option<&Vec<Self>> {
        match self {
            Instruction::Loop(x) => Some(x),
            _ => None,
        }
    }

    pub fn get_mut_loop(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Instruction::Loop(x) => Some(x),
            _ => None,
        }
    }

    pub fn parse(tokens: &[Token]) -> Result<Vec<Instruction>, ParseError> {
        let mut instructions: Vec<Instruction> = vec![];

        let mut nesting = 0;

        macro_rules! get_last_deepest {
            () => {{
                let mut instruction_ref: &mut Instruction = instructions.last_mut().unwrap();

                for _ in 1..nesting {
                    instruction_ref = instruction_ref.get_mut_loop().unwrap().last_mut().unwrap();
                }

                instruction_ref
            }};
        }

        for token in tokens.iter().copied() {
            match token {
                Token::LoopStart => {
                    if nesting > 0 {
                        get_last_deepest!()
                            .get_mut_loop()
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
                        get_last_deepest!()
                            .get_mut_loop()
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

    pub fn run(bf: &mut Brainfuck, instruction: &Instruction) {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        let mut stack: Vec<&Instruction> = vec![instruction];

        while let Some(current_instruction) = stack.pop() {
            match current_instruction {
                Instruction::Loop(inner) => {
                    if bf.tape[bf.pointer].0 != 0 {
                        // NOTE: since we're executing in reverse order, we must push in reverse too
                        stack.push(current_instruction);

                        for inner_instruction in inner.iter().rev() {
                            stack.push(inner_instruction);
                        }
                    }
                }
                Instruction::Inc => bf.tape[bf.pointer] += Wrapping(1),
                Instruction::Dec => bf.tape[bf.pointer] -= Wrapping(1),
                Instruction::Next => bf.next(),
                Instruction::Prev => bf.prev(),
                Instruction::Print => {
                    let output = bf.tape[bf.pointer].0;

                    stdout.write_all(&[output]).unwrap();

                    // TODO: make flush configurable at runtime
                    stdout.flush().unwrap();
                }
                Instruction::Read => {
                    let mut input_char: [u8; 1] = [0];

                    let _ = stdin.read_exact(&mut input_char);

                    bf.tape[bf.pointer] = Wrapping(input_char[0]);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("could not find match for `[`")]
    UnmatchedLoopStart,
    #[error("could not find match for `]`")]
    UnmatchedLoopEnd,
}
