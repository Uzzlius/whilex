use crate::environment::Environment;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;

use num_bigint::BigUint;
mod environment;
mod error;
mod grammar;
mod interpreter;
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
                if arg.ends_with(".whl") {
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

    let lexer: Lexer = Lexer::new(&file_contents);
    let environment = Environment::new(config.init);
    let mut interpreter = Interpreter { env: environment };

    let mut parser = Parser::new(lexer);
    let program = parser.program();

    for error in parser.errors {
        eprintln!("{}", error);
    }

    println!("{}", interpreter.program(&program?)?);

    Ok(())
}
