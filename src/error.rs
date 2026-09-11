use std::fmt;

// The implementation of errors is a bit scrappy. It would be cleaner to have
// different errors for different pipeline stages. For example, the lexer could now
// in theory return a `FileNotFound` Error, which is obviously stupid
pub enum Error {
    // I/O errors
    FileNotFound(String),
    IncorrectFiletype,
    MissingFilepath,
    ParsingArguments,

    // Lexing errors
    UnknownSymbol(usize),
}

// Implementing the display trait for errors, so that they can be ... displayed.
#[rustfmt::skip]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound(s) => write!(f, "Error: Could not find a file at `{}`.", s),
            Self::IncorrectFiletype => write!(f, "Error: The provided file does not end in `.while`."),
            Self::MissingFilepath => write!(f, "Error: No filepath was provided."),
            Self::ParsingArguments => write!(f, "Error: Could not parse the provided initial variables."),
            Self::UnknownSymbol(line) => write!(f, "Error: Unknown symbol in line {}.", line),
        }
    }
}
