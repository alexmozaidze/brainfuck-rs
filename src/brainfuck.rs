use std::num::Wrapping;

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
}
