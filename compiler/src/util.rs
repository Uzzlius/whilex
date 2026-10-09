use std::collections::HashSet;
use whilex::grammar::*;

pub fn get_args(program: &Program<'_>) -> Result<Vec<usize>, &'static str> {
    get_variables(program)
        .into_iter()
        .filter_map(|name| name.strip_prefix('x'))
        .filter(|suffix| !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()))
        .map(|suffix| suffix.parse::<usize>().map_err(|_| "Too many arguments"))
        .collect()
}

pub fn get_variables<'a>(
    program: &'a Program<'a>,
) -> HashSet<&'a str> {
    let mut variables = HashSet::new();

    match program {
        Program::Declarations(declarations) => {
            for decl in declarations {
                match decl {
                    Decl::Statement(stmt) => {
                        collect_stmt_variables(stmt, &mut variables);
                    }
                    Decl::Procedure { statement, .. } => {
                        collect_stmt_variables(statement, &mut variables);
                    }
                }
            }
        }
    }

    variables
}

fn collect_stmt_variables<'a>(
    stmt: &'a Stmt<'a>,
    variables: &mut HashSet<&'a str>,
) {
    match stmt {
        Stmt::VarAssignment { name, expr } => {
            variables.insert(name);
            collect_expr_variables(expr, variables);
        }
        Stmt::Block(statements) => {
            for stmt in statements {
                collect_stmt_variables(stmt, variables);
            }
        }
        Stmt::While { statement, .. } => {
            collect_stmt_variables(statement, variables);
        }
        Stmt::Procedure { .. } => {}
    }
}

fn collect_expr_variables<'a>(
    expr: &'a Expr<'a>,
    variables: &mut HashSet<&'a str>,
) {
    match expr {
        Expr::Literal(literal) => {
            collect_literal_variable(literal, variables);
        }
        Expr::BinaryOp { left, right, .. } => {
            collect_literal_variable(left, variables);
            collect_literal_variable(right, variables);
        }
    }
}

fn collect_literal_variable<'a>(
    literal: &'a Literal<'a>,
    variables: &mut HashSet<&'a str>,
) {
    if let Literal::Identifier(name) = literal {
        variables.insert(name);
    }
}
