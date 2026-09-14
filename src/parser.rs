use crate::Error;
use crate::Lexer;
use crate::grammar::Literal::Identifier;
use crate::grammar::*;
use crate::lexer::TokenType;

pub struct Parser<'a> {
    tokens: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { tokens: lexer }
    }

    pub fn program(&mut self) -> Result<Program<'a>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self._check_next(|n| matches!(n, TokenType::EOF)) {
            statements.push(self.statement()?);
        }

        Ok(Program::Statements(statements))
    }

    fn statement(&mut self) -> Result<Stmt<'a>, Error> {
        let line;

        let left = self.literal()?;
        let name = match left {
            Literal::Number { content, pos } => {
                return Err(Error::InvalidAssignmentTarget(pos.line));
            }
            Literal::Identifier { content, pos } => {
                line = pos.line;
                content
            }
        };

        if !self.match_next(|n| matches!(n, TokenType::LARROW)) {
            return Err(Error::ExpectedToken("<".to_string(), line));
        }

        let right = self.expression()?;

        if !self.match_next(|n| matches!(n, TokenType::SEMICOLON)) {
            return Err(Error::ExpectedToken(";".to_string(), line));
        }

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
        let token = self.tokens.peek().ok_or(Error::ExpectedExpression(0))?;

        match token.typ {
            TokenType::IDENTIFIER(n) => Ok(Literal::Identifier {
                content: n,
                pos: self.tokens.next().unwrap().pos,
            }),
            TokenType::NUMBER(n) => Ok(Literal::Number {
                content: n,
                pos: self.tokens.next().unwrap().pos,
            }),
            _ => Err(Error::ExpectedExpression(token.pos.line)),
        }
    }

    fn infix_operator(&mut self) -> Result<InfixOperator<'a>, Error> {
        let token = self.tokens.peek().ok_or(Error::ExpectedExpression(0))?;

        match token.typ {
            TokenType::PLUS => Ok(InfixOperator::Plus(self.tokens.next().unwrap().pos)),
            TokenType::MINUS => Ok(InfixOperator::Minus(self.tokens.next().unwrap().pos)),
            _ => Err(Error::ExpectedExpression(token.pos.line)),
        }
    }

    fn match_next<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(TokenType) -> bool,
    {
        let peek = match self.tokens.peek() {
            Some(n) => n.typ,
            None => return false,
        };
        if f(peek) {
            self.tokens.next();
            return true;
        } else {
            return false;
        }
    }

    fn _check_next<F>(&self, f: F) -> bool
    where
        F: FnOnce(TokenType) -> bool,
    {
        let peek = match self.tokens.peek() {
            Some(n) => n.typ,
            None => return false,
        };
        f(peek)
    }
}
