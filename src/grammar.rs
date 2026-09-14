use num_bigint::BigUint;

use crate::lexer::TokenPos;

#[derive(Debug)]
pub enum Program<'a> {
    Statements(Vec<Stmt<'a>>),
}

#[derive(Debug)]
pub enum Stmt<'a> {
    VarAssignment { name: &'a str, expr: Expr<'a> },
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
