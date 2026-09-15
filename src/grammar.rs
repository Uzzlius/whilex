use num_bigint::BigUint;

use crate::lexer::TokenPos;

#[derive(Debug)]
pub enum Program<'a> {
    Declarations(Vec<Decl<'a>>),
}

#[derive(Debug)]
pub enum Decl<'a> {
    Statement(Stmt<'a>),
    Procedure {
        name: &'a str,
        statement: Box<Stmt<'a>>,
    },
}

#[derive(Debug)]
pub enum Stmt<'a> {
    VarAssignment {
        name: &'a str,
        expr: Expr<'a>,
    },
    Block(Vec<Stmt<'a>>),
    While {
        name: &'a str,
        statement: Box<Stmt<'a>>,
    },
    Procedure(&'a str),
}

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    BinaryOp {
        left: Literal<'a>,
        operator: InfixOperator<'a>,
        right: Literal<'a>,
    },
}

#[derive(Debug)]
pub enum Literal<'a> {
    Number { content: BigUint, pos: TokenPos<'a> },
    Identifier { content: &'a str, pos: TokenPos<'a> },
}

#[derive(Debug)]
pub enum InfixOperator<'a> {
    Plus(TokenPos<'a>),
    Minus(TokenPos<'a>),
}
