use crate::{environment::Environment, error::Error, grammar::*};
use num_bigint::BigUint;

pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn program(&mut self, program: Program) -> Result<BigUint, Error> {
        match program {
            Program::Statements(statements) => {
                for statement in statements {
                    self.statement(statement)?;
                }
            }
        }
        Ok(self.env.retrieve("x0").clone())
    }

    fn statement(&mut self, stmt: Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::VarAssignment { name, expr } => {
                self.env.assign(name, self.expression(expr)?);
                return Ok(());
            }
        }
    }

    fn expression(&self, expression: Expr) -> Result<BigUint, Error> {
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
        left: Literal,
        operator: InfixOperator,
        right: Literal,
    ) -> Result<BigUint, Error> {
        let left = self.literal(left)?;
        let right = self.literal(right)?;

        match operator {
            InfixOperator::Plus(_) => Ok(left + right),
            InfixOperator::Minus(_) => {
                if (right > left) {
                    return Ok(BigUint::ZERO);
                }
                Ok(left - right)
            }
        }
    }

    fn literal(&self, literal: Literal) -> Result<BigUint, Error> {
        match literal {
            Literal::Number { content, pos } => Ok(content),
            Literal::Identifier { content, pos } => Ok(self.env.retrieve(content).clone()),
        }
    }
}
