use num_bigint::BigUint;

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    BinaryOp {
        left: Literal<'a>,
        operator: InfixOperator,
        right: Literal<'a>,
    },
}

#[derive(Debug)]
pub enum Literal<'a> {
    Number(BigUint),
    Identifier(&'a str),
}

#[derive(Debug)]
pub enum InfixOperator {
    Plus,
    Minus,
}
