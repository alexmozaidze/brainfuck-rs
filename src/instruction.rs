use thiserror::Error;

use crate::token::Token;

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
            loop_token => panic!("attempt to convert {:?} into Instruction", loop_token),
        }
    }
}

impl Instruction {
    pub fn get_inner_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Instruction::Loop(x) => Some(x),
            _ => None,
        }
    }
    pub fn get_last_deepest_mut(&mut self, nesting: usize) -> &mut Self {
        let mut instruction_ref: &mut Instruction = self;

        for _ in 1..nesting {
            instruction_ref = instruction_ref
                .get_inner_mut()
                .unwrap()
                .last_mut()
                .unwrap();
        }

        instruction_ref
    }

    pub fn parse(tokens: &[Token]) -> Result<Vec<Instruction>, ParseError> {
        let mut instructions: Vec<Instruction> = vec![];

        let mut nesting = 0;
        for token in tokens.iter().copied() {
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

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("could not find match for `[`")]
    UnmatchedLoopStart,
    #[error("could not find match for `]`")]
    UnmatchedLoopEnd,
}
