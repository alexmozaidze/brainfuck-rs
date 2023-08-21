//! An example of how to use `BufReader` and `BufWriter` instead of standard IO.

use std::{
    io::{BufReader, BufWriter},
    str,
};

use brainfuck_rs::{
    engine::{Engine, RuntimeSettings},
    instruction::Instruction,
    token::Token, utils::StripShebang,
};

const ROT13: &str = include_str!("brainfuck-programs/rot13.b");

fn main() {
    let mut bf = Engine::default();
    let settings = RuntimeSettings {
        should_flush: false,
        // NOTE: since rot13.b doesn't terminate on EOF, we should terminate when the input buffer
        // is emptied. This option is *specifically* made for this purpose.
        quit_on_eof: true,
    };

    let instructions = Instruction::parse(Token::tokenize(ROT13.strip_shebang())).unwrap();

    let input_string = b"Hello, World!\n";
    let mut input = BufReader::new(input_string.as_slice());
    let mut output = BufWriter::new(vec![]);

    bf.run(&instructions, &mut input, &mut output, settings);

    let input = str::from_utf8(input_string).unwrap().trim();
    let output = str::from_utf8(output.get_ref()).unwrap().trim();

    println!("Ayo! \"{input}\" ciphered in ROT13 is \"{output}\"");
}
