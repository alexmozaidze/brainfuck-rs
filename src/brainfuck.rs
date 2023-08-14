use std::{num::Wrapping, io::{self, Write, Read}};

use crate::instruction::Instruction;

pub struct Brainfuck {
    pub pointer: usize,
    pub tape: Vec<Wrapping<u8>>,
}

impl Brainfuck {
    pub fn next(&mut self) {
        if self.pointer == self.tape.len() - 1 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.tape.len() - 1;
        } else {
            self.pointer -= 1;
        }
    }

    pub fn run(&mut self, instruction: &Instruction, should_flush: bool) {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        let mut stack: Vec<&Instruction> = vec![instruction];

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

                    stdout.write_all(&[output]).unwrap();

                    if should_flush {
                        stdout.flush().unwrap();
                    }
                }
                Instruction::Read => {
                    if !should_flush {
                        stdout.flush().unwrap();
                    }

                    let mut input_char: [u8; 1] = [0];

                    let _ = stdin.read_exact(&mut input_char);

                    self.tape[self.pointer] = Wrapping(input_char[0]);
                }
            }
        }
    }
}
