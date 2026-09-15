use crate::Error;
use crate::Lexer;
use crate::grammar::*;
use crate::lexer::{Token, TokenType};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,
    pub errors: Vec<Error>,
    last_line: usize,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            errors: Vec::new(),
            tokens: lexer.peekable(),
            last_line: 1,
        }
    }

    pub fn program(&mut self) -> Result<Program<'a>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();

        loop {
            match self.peek() {
                Ok(t) => match t.typ {
                    TokenType::EOF => break,
                    _ => (),
                },
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                    continue;
                }
            }

            match self.statement() {
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }
        }

        if self.errors.len() != 0 {
            return Err(Error::ParsingError);
        }

        Ok(Program::Statements(statements))
    }

    fn statement(&mut self) -> Result<Stmt<'a>, Error> {
        let name = self.var()?;
        self.consume(TokenType::LARROW, "<")?;
        let right = self.expression()?;
        self.consume(TokenType::SEMICOLON, ";")?;

        Ok(Stmt::VarAssignment {
            name: name,
            expr: right,
        })
    }

    fn expression(&mut self) -> Result<Expr<'a>, Error> {
        let left = self.literal()?;

        if let Ok(operator) = self.infix_operator() {
            let right = self.literal()?;

            return Ok(Expr::BinaryOp {
                left: left,
                operator: operator,
                right: right,
            });
        }

        Ok(Expr::Literal(left))
    }

    fn literal(&mut self) -> Result<Literal<'a>, Error> {
        let token = self.peek()?.clone();

        match token.typ {
            TokenType::IDENTIFIER(n) => Ok(Literal::Identifier {
                content: n,
                pos: self.next()?.pos,
            }),
            TokenType::NUMBER(n) => Ok(Literal::Number {
                content: n,
                pos: self.next()?.pos,
            }),
            _ => Err(Error::ExpectedExpression(self.last_line)),
        }
    }

    fn infix_operator(&mut self) -> Result<InfixOperator<'a>, Error> {
        let token = self.peek()?;

        match token.typ {
            TokenType::PLUS => Ok(InfixOperator::Plus(self.next()?.pos)),
            TokenType::MINUS => Ok(InfixOperator::Minus(self.next()?.pos)),
            _ => Err(Error::ExpectedExpression(self.last_line)),
        }
    }

    fn var(&mut self) -> Result<&'a str, Error> {
        let token = self.peek()?;

        match token.typ {
            TokenType::IDENTIFIER(n) => {
                self.next()?;
                Ok(n)
            }
            _ => Err(Error::InvalidAssignmentTarget(self.last_line)),
        }
    }

    fn consume(&mut self, expected: TokenType, expect_msg: &str) -> Result<(), Error> {
        let next = self.peek()?;
        if expected.is_same_kind(&next.typ) {
            self.next()?;
            return Ok(());
        }
        Err(Error::ExpectedToken(expect_msg.to_string(), self.last_line))
    }

    fn synchronize(&mut self) {
        loop {
            let next = match self.peek() {
                Ok(next) => next,
                Err(_) => {
                    let _ = self.next();
                    continue;
                }
            };
            if matches!(
                next.typ,
                TokenType::EOF | TokenType::WHILE | TokenType::PROCEDURE
            ) {
                return;
            }
            let next = self.next().expect("Error handled on peek previously");
            if matches!(next.typ, TokenType::SEMICOLON) {
                return;
            }
        }
    }

    // Returns a reference to the next token without consuming it. Only if the next token call
    // returns an Error (lexing error), that error will be consumed and returned.
    fn peek(&mut self) -> Result<&Token<'a>, Error> {
        match self.tokens.peek() {
            Some(Ok(t)) => Ok(t),
            Some(Err(e)) => Err(e.clone()),
            None => Err(Error::ParsingError),
        }
    }

    // Unwraps the next token from the Option<> and returns a Result<Token, Error>
    fn next(&mut self) -> Result<Token<'a>, Error> {
        match self.tokens.next() {
            Some(Ok(t)) => {
                self.last_line = t.pos.line;
                Ok(t)
            }
            None => Err(Error::ParsingError),
            Some(Err(error)) => Err(error),
        }
    }
}
