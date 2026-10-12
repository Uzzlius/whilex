use std::collections::HashMap;
use inkwell::attributes::{Attribute, AttributeLoc};
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::{Linkage, Module};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, GlobalValue};
use num_traits::ToPrimitive;
use whilex::grammar::*;

use crate::util::{get_variables, parse_argument_index};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub program: &'a Program<'a>, 

    variables: HashMap<String, GlobalValue<'ctx>>,
    procedures: HashMap<String, FunctionValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn compile_decls(&mut self, declarations: &[Decl<'a>] ) -> Result<(), String> {
        for decl in declarations {
            self.compile_decl(decl)?;
        }
        Ok(())
    }

    fn compile_decl(&mut self, decl: &Decl) -> Result<(), String> {
        match decl {
            Decl::Procedure { name, statement } => {
                let void_type = self.context.void_type();
                let fn_type = void_type.fn_type(&[], false);
                let function = self.module.add_function(*name, fn_type, Some(Linkage::Private));
                self.procedures.insert(String::from(*name), function);
                let previous = self.builder.get_insert_block();
                let entry = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(entry);

                self.compile_stmt(statement)?;
                self.builder.build_return(None).map_err(|e| e.to_string())?;

                if let Some(previous) = previous {
                    self.builder.position_at_end(previous);
                }
                Ok(())
            }, 
            Decl::Statement(statement) => self.compile_stmt(statement)
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    self.compile_stmt(stmt)?;
                }
                Ok(())
            }, 
            Stmt::Procedure { name, line: _ } => {
                let func = self.procedures.get(*name).ok_or("Procedure not found")?;
                self.builder.build_call(*func, &[], "tmp").map_err(|e| e.to_string())?;
                Ok(())
            }, 
            Stmt::VarAssignment { name, expr } => {
                let expression_val = self.compile_expr(expr)?;
                let global = self.variables.get(*name).ok_or("Variable not found")?;
                self.builder.build_store(global.as_pointer_value(), expression_val).map_err(|e| e.to_string())?;
                Ok(())
            },
            Stmt::While { name, statement } => {
                let current_block = self.builder.get_insert_block().ok_or("No insert block")?;
                let function = current_block.get_parent().ok_or("No parent function")?;
                let cond_bb = self.context.append_basic_block(function, "while.cond");
                let body_bb = self.context.append_basic_block(function, "while.body");
                let end_bb  = self.context.append_basic_block(function, "while.end");

                self.builder.build_unconditional_branch(cond_bb).map_err(|e| e.to_string())?;

                self.builder.position_at_end(cond_bb);
                let global = self.variables.get(*name).ok_or("Could not find variable")?;
                let value = self.builder.build_load(
                    self.context.i128_type(), 
                    global.as_pointer_value(), 
                    "value"
                ).map_err(|e| e.to_string())?;
                let should_continue = self.builder.build_int_compare(inkwell::IntPredicate::NE, value.into_int_value(), value.get_type().const_zero().into_int_value(), "should_continue")
                    .map_err(|e| e.to_string())?;
                self.builder.build_conditional_branch(should_continue, body_bb, end_bb)
                    .map_err(|e| e.to_string())?;

                self.builder.position_at_end(body_bb);
                self.compile_stmt(statement)?;
                self.builder.build_unconditional_branch(cond_bb).map_err(|e| e.to_string())?;

                self.builder.position_at_end(end_bb);

                Ok(())
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expr::BinaryOp { left, operator, right } => {
                let left_val = self.compile_literal(left)?.into_int_value();
                let right_val = self.compile_literal(right)?.into_int_value();
                match operator {
                    InfixOperator::Minus => {
                        let can_subtract = self.builder.build_int_compare(
                            inkwell::IntPredicate::UGE, 
                            left_val, 
                            right_val, 
                            "can_subtract"
                        ).map_err(|e| e.to_string())?;
                        
                        let diff = self.builder
                            .build_int_sub(left_val, right_val, "sub")
                            .map_err(|e| e.to_string())?;
                        let zero = left_val.get_type().const_zero();
                        let res = self.builder
                            .build_select(can_subtract, diff, zero, "res")
                            .map_err(|e| e.to_string())?;
                        Ok(res)
                    },
                    InfixOperator::Plus => {
                        Ok(self.builder.build_int_add(left_val, right_val, "res").map_err(|e| e.to_string())?.as_basic_value_enum())
                    }
                }
            },
            Expr::Literal(lit) => self.compile_literal(lit)
        }
    }

    fn compile_literal(&mut self, lit: &Literal) -> Result<BasicValueEnum<'ctx>, String> {
        match lit {
            Literal::Identifier(name) => {
                let global = self.variables.get(*name).ok_or("Variable not found")?;
                let load = self.builder.build_load(self.context.i128_type(), global.as_pointer_value(), "var").map_err(|e| e.to_string())?;
                return Ok(load.into());
            },
            Literal::Number(num) => {
                let num: u128 = match num.to_u128() {
                    Some(v) => v,
                    None => return Err(format!("Literal number {num} too large for u128."))
                };
                let words = [
                    num as u64,
                    (num >> 64) as u64,
                ];

                Ok(self.context.i128_type().const_int_arbitrary_precision(&words).into())
            }
        }
    }
    

    fn compile_program(&mut self) -> Result<FunctionValue<'ctx>, String> {
        let i128_type = self.context.i128_type();
        let i32_type = self.context.i32_type();


        let fn_type = i128_type.fn_type(&[], false);
        let fn_val = self.module.add_function("_main", fn_type, Some(Linkage::External));
        self.fn_value_opt = Some(fn_val);

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            let name = format!("x{i}");
            arg.into_int_value().set_name(&name);
        }

        let entry = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(entry);

        
        let load_arg_fn_type = i128_type.fn_type(&[i32_type.into()], false);
        let load_arg_fn = self.module.add_function("_load_arg", load_arg_fn_type, Some(Linkage::External));
        load_arg_fn.add_attribute(AttributeLoc::Function, self.context.create_enum_attribute(Attribute::get_named_enum_kind_id("memory"), 0x555));
        load_arg_fn.add_attribute(AttributeLoc::Function, self.context.create_enum_attribute(Attribute::get_named_enum_kind_id("nounwind"), 0));
        load_arg_fn.add_attribute(AttributeLoc::Function, self.context.create_enum_attribute(Attribute::get_named_enum_kind_id("willreturn"), 0));


        for name in get_variables(self.program) {
            let global = self.module.add_global(i128_type, None, name);
            global.set_linkage(Linkage::Internal);
            global.set_initializer(&i128_type.const_zero());
            self.variables.insert(String::from(name), global);

            let arg_number = parse_argument_index(name)?;
            let value = match arg_number {
                None => i128_type.const_zero().as_basic_value_enum(), 
                Some(idx) => {
                    if idx.to_u32().is_none() {
                        return Err(format!("The argument index for {name} is too large."))
                    }
                    let index = i32_type.const_int(idx as u64, false);
                    let call = self
                        .builder
                        .build_call(load_arg_fn, &[index.into()], "arg_value")
                        .map_err(|e| e.to_string())?;

                    call.try_as_basic_value().basic().ok_or("Call has no basic value return")?
                }
            };

            self.builder
                .build_store(global.as_pointer_value(), value)
                .map_err(|e| e.to_string())?;
        }

        self.compile_decls(match self.program {
            Program::Declarations(decls) => decls
        })?;

        let return_value = if let Some(global) = self.variables.get("x0") {
            self.builder
                .build_load(i128_type, global.as_pointer_value(), "return_value")
                .map_err(|e| e.to_string())?
                .as_basic_value_enum()
        } else {
            i128_type.const_zero().as_basic_value_enum()
        };
        self.builder.build_return(Some(&return_value)).map_err(|e| e.to_string())?;

        return Ok(fn_val);
    }

    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        program: &Program,
    ) -> Result<FunctionValue<'ctx>, String> {
        let mut compiler = Compiler {
            context,
            builder,
            module,
            program: program,
            fn_value_opt: None,
            variables: HashMap::new(),
            procedures: HashMap::new()
        };

        compiler.compile_program()
    }
}