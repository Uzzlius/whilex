use std::iter::Peekable;

use crate::Error;
use crate::Lexer;
use crate::grammar::*;
use crate::lexer::TokenType;

pub struct Parser<'a> {
    tokens: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { tokens: lexer }
    }

    pub fn expression(&mut self) -> Result<Expr<'a>, Error> {
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
        let token = self.tokens.peek().ok_or(Error::ExpectedExpression(0))?;

        match token.typ {
            TokenType::IDENTIFIER(n) => {
                self.tokens.next();
                Ok(Literal::Identifier(n))
            }
            TokenType::NUMBER(n) => {
                self.tokens.next();
                Ok(Literal::Number(n))
            }
            _ => Err(Error::ExpectedExpression(token.line)),
        }
    }

    fn infix_operator(&mut self) -> Result<InfixOperator, Error> {
        let token = self.tokens.peek().ok_or(Error::ExpectedExpression(0))?;

        match token.typ {
            TokenType::PLUS => {
                self.tokens.next();
                Ok(InfixOperator::Plus)
            }
            TokenType::MINUS => {
                self.tokens.next();
                Ok(InfixOperator::Plus)
            }
            _ => Err(Error::ExpectedExpression(token.line)),
        }
    }
}
