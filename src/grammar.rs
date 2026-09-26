use num_bigint::BigUint;

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
    Procedure {
        name: &'a str,
        line: usize,
    },
}

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
