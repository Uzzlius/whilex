use crate::error::Error;
use std::str::CharIndices;

// The theoretical while-language allows for infinitely large integers, which is
// why we use the external crate num_bigint that manages these unbounded numbers
use num_bigint::BigUint;

/// Enumerates all possible Tokens in the language. There are only so many in a simple
/// language like this. Only IDENTIFIER and NUMBER hold values.
#[derive(Debug)]
enum TokenType<'a> {
    // Single-character tokens
    PLUS,
    MINUS,
    LARROW,
    SEMICOLON,
    LBRACKET,
    RBRACKET,

    // Two-character tokens
    NEQUAL,

    // Literals
    IDENTIFIER(&'a str),
    NUMBER(BigUint),

    // Keywords
    PROCEDURE,
    WHILE,
}

/// The datatype of a single token, that specifies its type, lexeme,
/// and the line number the token occurred on.
#[derive(Debug)]
pub struct Token<'a> {
    typ: TokenType<'a>,
    lexeme: &'a str,
    line: usize,
}

/// The Lexer-struct holds an iterator over the characters in the source file and
/// the line number that is currently being evaluated, as well as a vector containing
/// all the errors that were found while lexing.
pub struct Lexer<'a> {
    source: &'a str,
    chars: CharIndices<'a>,
    line: usize,
    pub errors: Vec<Error>,
}

// Implementations on the Lexer struct
impl<'a> Lexer<'a> {
    /// Create a new lexer from a String reference or string slice.
    /// # Example
    /// let lexer: Lexer = Lexer::new("x0 < 12;");
    pub fn new(s: &'a str) -> Self {
        Self {
            source: s,
            chars: s.char_indices(),
            line: 1,
            errors: Vec::new(),
        }
    }

    // A helper function that allows for easy token creation. It takes in the reference to the source code,
    // as well as a start index and how many bytes have been read and returns an Option<Token> out of it
    fn make_token(&self, typ: TokenType<'a>, start: usize, diff: usize) -> Option<Token<'a>> {
        Some(Token {
            typ: typ,
            lexeme: &self.source[start..start + diff],
            line: self.line,
        })
    }

    // A helper function that allows non-consuming lookahead. This is needed for seeing whether the
    // next character belongs to the current token or not
    fn peek(&self) -> Option<char> {
        // Uses clone but is still pretty cheap as iterators are pretty small
        self.chars.clone().next().map(|(_, c)| c)
    }

    // A helper function that checks whether the next character matches some given expected character.
    // If so, it consumes the character and returns the characters size. If not, it returns None.
    fn match_next(&mut self, expect: char) -> Option<usize> {
        let peek = self.peek();
        let c = match peek {
            Some(c) => c,
            None => return None,
        };
        match c == expect {
            true => {
                self.chars.next();
                Some(c.len_utf8())
            }
            false => None,
        }
    }

    // A helper function that consumes characters as long as they fulfill a given criteria.
    // The function takes a closure that evaluates to a bool. If the closure evaluates to true,
    // the character is consumed. If it evaluates to false, the function returns.
    // The sum of consumed bytes is counted up and returned.
    fn consume_while<F>(&mut self, f: F) -> usize
    where
        F: Fn(char) -> bool,
    {
        let mut byte_count: usize = 0;
        loop {
            let next = match self.peek() {
                Some(next) => next,
                None => return byte_count,
            };

            if f(next) {
                byte_count += self
                    .chars
                    .next()
                    .expect("should not panic, as next() did not return `None` on the clone either")
                    .1
                    .len_utf8();
            } else {
                return byte_count;
            }
        }
    }
}

// Implementing the Iterator trait on the lexer.
impl<'a> Iterator for Lexer<'a> {
    // The iterator is supposed to return a new Token
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Starting with a new Token, we get the next character and its byte index.
            // If there is no new character, we return None
            let (start, c) = self.chars.next()?;

            // Matching the new character
            match c {
                // Single-character tokens
                '\n' => self.line += 1,
                c if c.is_whitespace() => (),
                '+' => return self.make_token(TokenType::PLUS, start, c.len_utf8()),
                '-' => return self.make_token(TokenType::MINUS, start, c.len_utf8()),
                '<' => return self.make_token(TokenType::LARROW, start, c.len_utf8()),
                ';' => return self.make_token(TokenType::SEMICOLON, start, c.len_utf8()),
                '{' => return self.make_token(TokenType::LBRACKET, start, c.len_utf8()),
                '}' => return self.make_token(TokenType::RBRACKET, start, c.len_utf8()),

                // Two-character token !=
                '!' => {
                    let mut byte_len = c.len_utf8();
                    byte_len += match self.match_next('=') {
                        Some(byte_len) => byte_len,
                        None => {
                            self.errors.push(Error::UnknownSymbol(self.line));
                            continue;
                        }
                    };
                    return self.make_token(TokenType::NEQUAL, start, byte_len);
                }

                // Multi-character tokens
                //
                // Comments. When / is detected, we check if it is followed by another.
                '/' => {
                    match self.match_next('/') {
                        Some(_) => (),

                        // If not, we report an error as `/` is not a valid character
                        None => {
                            self.errors.push(Error::UnknownSymbol(self.line));
                            continue;
                        }
                    }
                    // If there is another /, we consume the entire line without adding a token
                    self.consume_while(|c| c != '\n');
                }

                // Numbers. Once an ascii digit (0, ..., 9) is detected, we will consume all following digits
                // by using the consume_while() method.
                c if c.is_ascii_digit() => {
                    // counts the consumed bytes
                    let mut byte_len: usize = self.consume_while(|c| c.is_ascii_digit());
                    byte_len += c.len_utf8();

                    // Converting the lexeme to a BigUint
                    let number: BigUint = self.source[start..start + byte_len]
                        .parse::<BigUint>()
                        .expect(
                            "Conversion should be valid, as ascii digit only has been enshured",
                        );

                    return self.make_token(TokenType::NUMBER(number), start, byte_len);
                }

                // Identifiers. Once an ascii letter (a, ..., z, A, ..., Z) is detected, we will consume all
                // following letters and digits, again by using consume_while()
                c if c.is_ascii_alphabetic() => {
                    // counting consumed bytes
                    let mut byte_len: usize =
                        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
                    byte_len += c.len_utf8();

                    let identifier: &str = &self.source[start..start + byte_len];

                    // An identifier could be a keyword, which is why we check for the only two keywords
                    // the language allows, being `procedure` and `while`. If the lexeme does not match
                    // these keywords, it is an identifier.
                    let token = match identifier {
                        "procedure" => TokenType::PROCEDURE,
                        "while" => TokenType::WHILE,
                        _ => TokenType::IDENTIFIER(identifier),
                    };

                    return self.make_token(token, start, byte_len);
                }

                // All other tokens are invalid and will be reported as an error
                _ => {
                    self.errors.push(Error::UnknownSymbol(self.line));
                }
            }
        }
    }
}
