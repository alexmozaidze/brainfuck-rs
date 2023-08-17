use std::{
    io::{Read, Write},
    num::Wrapping,
};

use crate::instruction::Instruction;

/// Contains the state of the program.
pub struct Engine {
    /// Current cursor/pointer index.
    pub pointer: usize,
    /// The tape that contains all the cells.
    pub tape: Vec<Wrapping<u8>>,
    /// If `true`, the output is flushed on every print instruction, otherwise the output is buffered.
    pub should_flush: bool,
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

    /// Runs a Brainfuck instruction.
    ///
    /// # Usage
    ///
    /// Chances are, you most likely want to run an instruction *sequence*, in that case, you simply
    /// iterate over instructions and run them like so:
    ///
    /// ```
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
    /// #   engine::Engine,
    /// #   token::Token,
    /// # };
    /// let mut bf = Engine::default();
    ///
    /// let code = "+>>>>>>>>>>-[,+[-.----------[[-]>]<->]<]";
    /// let tokens = Token::tokenize(&code);
    /// let instructions: Vec<Instruction> = Instruction::parse(tokens).unwrap();
    ///
    /// let mut input = io::stdin();
    /// let mut output = io::stdout();
    ///
    /// # return;
    /// for instruction in instructions {
    ///     bf.run(&instruction, &mut input, &mut output);
    /// }
    /// ```
    ///
    /// You can also capture program's output and specify its input into a separate buffer by
    /// replacing [`io::stdin()`](`std::io::stdin`) and [`io::stdout()`](`std::io::stdout`) with [`BufReader`](`std::io::BufReader`) and [`BufWriter`](`std::io::BufWriter`)
    /// respectively.
    ///
    /// ```
    /// # use std::io::{BufReader, BufWriter};
    /// # use brainfuck_rs::engine::Engine;
    /// # let bf = Engine::default();
    /// let mut input = BufReader::new(b"Hello, World!".as_slice());
    /// let mut output = BufWriter::new(vec![]);
    /// ```
    ///
    /// You can use any buffer, as long as it implements [`std::io::Write`] and [`std::io::Read`].
    pub fn run(
        &mut self,
        instruction: &Instruction,
        stdin: &mut impl Read,
        stdout: &mut impl Write,
    ) {
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

                    if self.should_flush {
                        stdout.flush().unwrap();
                    }
                }
                Instruction::Read => {
                    if !self.should_flush {
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

impl Default for Engine {
    /// Creates a new `Engine` with default values:
    ///
    /// ```
    /// # use std::num::Wrapping;
    /// # use brainfuck_rs::engine::Engine;
    /// Engine {
    ///     pointer: 0,
    ///     tape: vec![Wrapping(0); 30_000],
    ///     should_flush: true,
    /// }
    /// # ;
    /// ```
    fn default() -> Self {
        Self {
            pointer: 0,
            tape: vec![Wrapping(0); 30_000],
            should_flush: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter};
    use std::str;

    use crate::token::Token;

    use super::*;

    const HELLO_WORLD: &str = include_str!("../../brainfuck-programs/hello-world.b");

    #[test]
    fn non_stdout_buffer() {
        let mut bf = Engine::default();

        let mut input = BufReader::new(<&[u8]>::default());
        let mut output = BufWriter::new(vec![]);

        let tokens = Token::tokenize(HELLO_WORLD);
        let instructions = Instruction::parse(tokens).unwrap();

        for instruction in &instructions {
            bf.run(instruction, &mut input, &mut output);
        }

        assert_eq!(
            "Hello World!\n",
            str::from_utf8(output.into_inner().unwrap().as_slice()).unwrap()
        );
    }

    #[test]
    fn non_stdin_buffer() {
        let mut bf = Engine::default();

        let mut input = BufReader::new(b"Hello, World!".as_slice());
        let mut output = BufWriter::new(vec![]);

        let tokens = Token::tokenize(&",.,.,.,.,.,.,.,.,.,.,.,.,.");
        let instructions = Instruction::parse(tokens).unwrap();

        for instruction in &instructions {
            bf.run(instruction, &mut input, &mut output);
        }

        assert_eq!(
            "Hello, World!",
            str::from_utf8(output.into_inner().unwrap().as_slice()).unwrap()
        );
    }
}
