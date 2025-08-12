mod lexer;
mod lexing_rules;
mod token;

use std::fs::File;
use std::io::{BufReader, Error, prelude::*};

use crate::lexer::lex;

const INPUT_FILE: &str = "input.txt";

fn main() -> Result<(), Error> {
	println!("Hello, world!");

	let input_file = File::open(INPUT_FILE).expect("input file");
	let mut buf_reader = BufReader::new(input_file);
	let mut text = String::new();
	buf_reader.read_to_string(&mut text)?;
	let tokens = lex(&mut text);
	println!("{:?}", tokens);
	Ok(())
}
