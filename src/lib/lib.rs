//! # About
//!
//! A configurable and fast Brainfuck interpreter.
//!
//! # Usage
//!
//! For simply running the program:
//!
//! ```
//! # mod io {
//! #   use std::io::{BufReader, BufWriter};
//! #   pub fn stdin() -> BufReader<&'static [u8]> {
//! #       BufReader::new(<&[u8]>::default())
//! #   }
//! #   pub fn stdout() -> BufWriter<Vec<u8>> {
//! #       BufWriter::new(vec![])
//! #   }
//! # }
//! # use brainfuck_rs::{
//! #   engine::{Engine, RuntimeSettings},
//! #   token::Token,
//! #   instruction::Instruction,
//! # };
//! let mut bf = Engine::default();
//! let settings = RuntimeSettings::default();
//!
//! let code = "+>>>>>>>>>>-[,+[-.----------[[-]>]<->]<]";
//!
//! let instructions: Vec<Instruction> = Instruction::parse(Token::tokenize(&code)).unwrap();
//!
//! let mut input = io::stdin();
//! let mut output = io::stdout();
//!
//! # return;
//! bf.run(&instructions, &mut input, &mut output, settings);
//! ```
//!
//! But you can also redirect the input and output to any buffer that implements [`std::io::Read`]
//! and [`std::io::Write`]:
//!
//! ```
//! # use std::io::{BufReader, BufWriter};
//! let mut input = BufReader::new(b"some input".as_slice());
//! let mut output = BufWriter::new(vec![]);
//! ```
#[warn(
    clippy::use_self,
    clippy::unnested_or_patterns,
    clippy::unnecessary_box_returns,
    clippy::uninlined_format_args,
    clippy::unicode_not_nfc,
    clippy::string_to_string,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::str_to_string,
    clippy::missing_errors_doc,
    clippy::map_unwrap_or,
    clippy::manual_let_else,
    clippy::if_then_some_else_none,
    clippy::derive_partial_eq_without_eq,
    clippy::default_trait_access,
    clippy::cloned_instead_of_copied
)]

/// The interpreter that can run Brainfuck programs.
pub mod engine;
/// An AST that is fed to [`Engine`](`crate::engine::Engine`) in order to run Brainfuck programs.
pub mod instruction;
/// Tokens used to generate an AST.
pub mod token;
