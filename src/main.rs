#![warn(
    clippy::if_not_else
)]
#![deny(
    clippy::as_conversions,
    clippy::default_trait_access,
    clippy::implicit_clone,
    clippy::inefficient_to_string,
    clippy::string_add_assign,
    clippy::semicolon_if_nothing_returned,
    clippy::chars_last_cmp,
    clippy::chars_next_cmp,
    clippy::comparison_chain,
    clippy::comparison_to_empty,
    clippy::len_zero,
    clippy::implicit_return
)]

use std::error::Error;
use std::time::Instant;
use structopt::StructOpt;

mod definition;
mod io;
mod optimizer;
mod parser;
mod grammar;
mod token;
mod token_grammar;
mod token_stream;
mod tokenizer;
mod transpiler;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    // parse cli options
    let opt = cli::Opt::from_args();

    // read file
    let file_text = io::read_file(opt.file())?;

    // tokenize
    let tokens = tokenizer::tokenize(&file_text)?;

    // parse
    let ast = parser::parse_syntax(&tokens)?;

    // write file
    // io::write_file("out/sample.wasm")?;

    // println!("{}", std::mem::size_of::<std::ops::Range<usize>>());

    println!("Process time: {}ms", now.elapsed().as_millis());

    return Ok(());
}