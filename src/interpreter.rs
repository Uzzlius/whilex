use crate::{environment::Environment, error::Error, grammar::*};
use num_bigint::BigUint;

pub struct Interpreter<'a> {
    pub env: Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn program(&mut self, program: &'a Program<'a>) -> Result<BigUint, Error> {
        match program {
            Program::Declarations(declarations) => {
                self.declarations(declarations)?;
            }
        }
        Ok(self.env.retrieve_var("x0").clone())
    }

    fn declarations(&mut self, declarations: &'a [Decl<'a>]) -> Result<(), Error> {
        for declaration in declarations {
            self.declaration(declaration)?;
        }

        Ok(())
    }

    fn declaration(&mut self, declaration: &'a Decl<'a>) -> Result<(), Error> {
        match declaration {
            Decl::Statement(stmt) => return self.statement(stmt),
            Decl::Procedure { name, statement } => {
                self.env.assign_proc(name, statement);
                return Ok(());
            }
        }
    }

    fn statements(&mut self, statements: &[Stmt]) -> Result<(), Error> {
        for statement in statements {
            self.statement(statement)?
        }
        Ok(())
    }

    fn statement(&mut self, statement: &Stmt) -> Result<(), Error> {
        match statement {
            Stmt::VarAssignment { name, expr } => {
                self.env.assign_var(name, self.expression(expr)?);
            }
            Stmt::Block(statements) => {
                self.statements(statements)?;
            }
            Stmt::While { name, statement } => {
                while self.env.retrieve_var(name).clone() != BigUint::ZERO {
                    self.statement(statement)?;
                }
                return Ok(());
            }
            Stmt::Procedure(name) => {
                let stmt = match self.env.retrieve_proc(name) {
                    Some(stmt) => stmt,
                    None => return Err(Error::UndefinedProcedure(0)),
                };
                self.statement(stmt)?;
            }
        }
        return Ok(());
    }

    fn expression(&self, expression: &Expr) -> Result<BigUint, Error> {
        match expression {
            Expr::Literal(literal) => self.literal(literal),
            Expr::BinaryOp {
                left,
                operator,
                right,
            } => self.binary_operation(left, operator, right),
        }
    }

    fn binary_operation(
        &self,
        left: &Literal,
        operator: &InfixOperator,
        right: &Literal,
    ) -> Result<BigUint, Error> {
        let left = self.literal(left)?;
        let right = self.literal(right)?;

        match operator {
            InfixOperator::Plus(_) => Ok(left + right),
            InfixOperator::Minus(_) => {
                if right > left {
                    return Ok(BigUint::ZERO);
                }
                Ok(left - right)
            }
        }
    }

    fn literal(&self, literal: &Literal) -> Result<BigUint, Error> {
        match literal {
            Literal::Number { content, pos } => Ok(content.clone()),
            Literal::Identifier { content, pos } => Ok(self.env.retrieve_var(content).clone()),
        }
    }
}
