use std::fmt;

// The implementation of errors is a bit scrappy. It would be cleaner to have
// different errors for different pipeline stages. For example, the lexer could now
// in theory return a `FileNotFound` Error, which is obviously stupid
#[derive(Clone)]
pub enum Error {
    // I/O errors
    FileNotFound(String),
    IncorrectFiletype,
    MissingFilepath,
    ParsingArguments,

    // Lexing errors
    UnknownSymbol(usize),

    // Parsing errors
    ParsingError,
    ExpectedExpression(usize),
    NotAVariable(usize),
    ExpectedToken(String, usize),

    // Runtime error
    MismatchedTypes(usize),
    UndefinedProcedure(usize),
}

// Implementing the display trait for errors, so that they can be ... displayed.
#[rustfmt::skip]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound(s) => write!(f, "Error: Could not find a file at `{}`.", s),
            Self::IncorrectFiletype => write!(f, "Error: The provided file does not end in `.whl`."),
            Self::MissingFilepath => write!(f, "Error: No filepath was provided."),
            Self::ParsingArguments => write!(f, "Error: Could not parse the provided initial variables."),
            Self::UnknownSymbol(line) => write!(f, "Error: Unknown symbol in line {}.", line),
            Self::ExpectedExpression(line) => write!(f, "Error: Expected an expression in line {}.", line),
            Self::MismatchedTypes(line) => write!(f, "Error: Found unexpected type in line {}.", line),
            Self::NotAVariable(line) => write!(f, "Error: Expected a variable in line {}.", line),
            Self::ExpectedToken(token, line) => write!(f, "Error: Expected {} in line {}.", token, line),
            Self::ParsingError => write!(f, "Error: An error occured while parsing."),
            Self::UndefinedProcedure(line) => write!(f, "Error: Referenced Procedure has not been defined in line {}.", line)
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
