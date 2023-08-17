use brainfuck_rs::{engine::Engine, instruction::Instruction, token::Token};
use clap::{command, value_parser, Arg, ArgMatches};
use color_eyre::eyre::{eyre, Context, Result};
use std::{
    fs,
    io::{self, Stdin},
    num::Wrapping,
    path::PathBuf,
};

fn read_until_eof(handle: &Stdin, buffer: &mut String) -> Result<()> {
    loop {
        let input_length = handle
            .read_line(&mut *buffer)
            .wrap_err("unable to read from stdin")?;

        if input_length == 0 {
            break;
        }
    }

    Ok(())
}

fn get_program(matches: &ArgMatches) -> Result<String> {
    match matches.get_one::<PathBuf>("input") {
        Some(file_path) => {
            let code = match fs::read_to_string(file_path) {
                Ok(code) => code,
                Err(error) => {
                    return Err(
                        eyre!("unable to open file \"{}\"", file_path.display()).wrap_err(error)
                    )
                }
            };

            Ok(code)
        }
        None => {
            let mut input = "".to_owned();
            read_until_eof(&io::stdin(), &mut input).unwrap();

            Ok(input)
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = command!()
        .arg(
            Arg::new("input")
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

    let mut bf = Engine {
        pointer: 0,
        tape: vec![Wrapping(0); *matches.get_one::<usize>("tape-length").unwrap()],
        should_flush: *matches.get_one::<bool>("should-flush").unwrap(),
    };

    let code: String = get_program(&matches)?;
    if code.trim().is_empty() {
        return Ok(());
    }

    let instructions = match Instruction::parse(Token::tokenize(&code)) {
        Ok(ok) => ok,
        Err(error) => return Err(eyre!("error while parsing").wrap_err(error)),
    };

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    for instruction in instructions {
        bf.run(&instruction, &mut stdin, &mut stdout);
    }

    println!();

    Ok(())
}
