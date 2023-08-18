use brainfuck_rs::{
    engine::{Engine, RuntimeSettings},
    instruction::Instruction,
    token::Token,
};
use clap::{command, value_parser, Arg};
use color_eyre::eyre::Result;
use fs_err as fs;
use std::{
    io::{self, IsTerminal},
    num::Wrapping,
    path::PathBuf,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = command!()
        .arg(
            Arg::new("input")
                .required(true)
                .value_name("FILE")
                .help("Brainfuck program to run")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("tape-length")
                .short('t')
                .long("tape-length")
                .value_name("BYTES")
                .help("Tape length")
                .value_parser(value_parser!(usize))
                .default_value("30000"),
        )
        .arg(
            Arg::new("should-flush")
                .short('f')
                .long("flush")
                .value_name("BOOL")
                .help("Flush the buffer on every print")
                .value_parser(value_parser!(bool))
                .default_value("true"),
        )
        .get_matches();

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut bf = Engine {
        pointer: 0,
        tape: vec![Wrapping(0); *matches.get_one::<usize>("tape-length").unwrap()],
    };
    let settings = RuntimeSettings {
        should_flush: *matches.get_one::<bool>("should-flush").unwrap(),
        quit_on_eof: !stdin.is_terminal(),
    };

    let code = fs::read_to_string(
        matches
            .get_one::<PathBuf>("input")
            .map(PathBuf::as_path)
            .unwrap(),
    )?;

    let instructions = Instruction::parse(Token::tokenize(&code))?;

    bf.run(&instructions, &mut stdin, &mut stdout, settings);

    println!();

    Ok(())
}
