use crate::error::Error;
use crate::lexer::{Lexer, Token};
use std::fs;

use num_bigint::BigUint;
mod error;
mod grammar;
mod lexer;
mod parser;

// The configuration struct represents everything the user entered, being
// the path to a `.while` file and an arbitrary amount of initial values.
#[derive(Debug)]
pub struct Config {
    file_path: String,
    init: Vec<BigUint>,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, Error> {
        args.next();
        let file_path = match args.next() {
            Some(arg) => {
                if arg.ends_with(".while") {
                    arg
                } else {
                    return Err(Error::IncorrectFiletype);
                }
            }
            None => return Err(Error::MissingFilepath),
        };

        let init: Result<Vec<BigUint>, _> = args.map(|s| s.parse::<BigUint>()).collect();
        let init = match init {
            Ok(v) => v,
            Err(_) => return Err(Error::ParsingArguments),
        };

        Ok(Self { file_path, init })
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    let file_contents = match fs::read_to_string(&config.file_path) {
        Ok(file_contents) => file_contents,
        Err(_) => return Err(Error::FileNotFound(config.file_path)),
    };

    let mut lexer: Lexer = Lexer::new(&file_contents);

    for token in &mut lexer {
        println!("{:?}", token);
    }

    for error in &mut lexer.errors {
        eprintln!("{}", error);
    }

    let mut parser = parser::Parser::new(Lexer::new(&file_contents));

    let ast = parser.expression();

    match ast {
        Ok(o) => println!("{:?}", o),
        Err(err) => eprintln!("{}", err),
    }

    Ok(())
}
