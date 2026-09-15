use num_bigint::BigUint;

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
        let mut declarations: Vec<Decl> = Vec::new();

        while match self.peek() {
            Ok(token) => match token.typ {
                TokenType::EOF => false,
                _ => true,
            },
            _ => true,
        } {
            match self.declaration() {
                Ok(decl) => declarations.push(decl),
                Err(_) => continue,
            }
        }
        if !self.errors.is_empty() {
            return Err(Error::ParsingError);
        }

        return Ok(Program::Declarations(declarations));
    }

    fn declaration(&mut self) -> Result<Decl<'a>, Error> {
        //Procedure declaration
        if let Ok(()) = self.consume(TokenType::PROCEDURE, "procedure") {
            match self.procedure() {
                Ok(proc) => return Ok(proc),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                    return Err(Error::ParsingError);
                }
            }
        }

        // Statement
        match self.statement() {
            Ok(stmt) => return Ok(Decl::Statement(stmt)),
            Err(err) => {
                self.errors.push(err);
                self.synchronize();
                return Err(Error::ParsingError);
            }
        }
    }

    fn procedure(&mut self) -> Result<Decl<'a>, Error> {
        let name = self.var()?;
        let stmt = self.statement()?;

        return Ok(Decl::Procedure {
            name: name,
            statement: Box::new(stmt),
        });
    }

    fn statement(&mut self) -> Result<Stmt<'a>, Error> {
        // while statement
        if let Ok(()) = self.consume(TokenType::WHILE, "while") {
            let name = self.var()?;
            self.consume(TokenType::NEQUAL, "!=")?;
            match self.literal()? {
                Literal::Number { content, pos } if content == BigUint::ZERO => (),
                _ => return Err(Error::ExpectedToken("0".to_string(), self.last_line)),
            }
            let stmt = self.statement()?;

            return Ok(Stmt::While {
                name: name,
                statement: Box::new(stmt),
            });
        }

        // {} block
        if let Ok(()) = self.consume(TokenType::LBRACKET, "{") {
            let mut statements = Vec::new();

            while match self.peek() {
                Ok(t) => !matches!(t.typ, TokenType::RBRACKET | TokenType::EOF),
                Err(_) => true,
            } {
                statements.push(self.statement()?);
            }

            self.consume(TokenType::RBRACKET, "}")?;
            return Ok(Stmt::Block(statements));
        }

        // var declaration or procedure calling
        self.var_dec_or_proc_call()
    }

    fn var_dec_or_proc_call(&mut self) -> Result<Stmt<'a>, Error> {
        let name = self.var()?;
        if let Ok(()) = self.consume(TokenType::LARROW, "<") {
            let right = self.expression()?;
            self.consume(TokenType::SEMICOLON, ";")?;

            return Ok(Stmt::VarAssignment {
                name: name,
                expr: right,
            });
        }
        self.consume(TokenType::SEMICOLON, ";")?;

        return Ok(Stmt::Procedure(name));
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
            _ => Err(Error::NotAVariable(self.last_line)),
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
                TokenType::EOF | TokenType::WHILE | TokenType::PROCEDURE | TokenType::LBRACKET
            ) {
                return;
            }
            let next = self.next().expect("Error handled on peek previously");
            if matches!(next.typ, TokenType::SEMICOLON | TokenType::RBRACKET) {
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
