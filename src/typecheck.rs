use std::collections::HashMap;
use thiserror::Error;

use crate::ast::*;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("[ERROR_UNEXPECTED_TYPE_FOR_PARAMETER] unexpected type for parameter")]
    UnexpectedTypeForParameter,
    #[error("[ERROR_UNEXPECTED_TYPE_FOR_EXPRESSION] unexpected type for expression (expected {expected:?}, found {actual:?})")]
    UnexpectedTypeForExpression { expected: Type, actual: Type },
    #[error("[ERROR_UNEXPECTED_LAMBDA] unexpected lambda (expected {expected:?})")]
    UnexpectedLambda { expected: Type },
    #[error("[ERROR_NOT_A_FUNCTION] not a function (found {actual:?})")]
    NotAFunction { actual: Type },
    #[error("[ERROR_UNDEFINED_VARIABLE] undefined variable `{0}`")]
    UndefinedVariable(String),
    #[error("[ERROR_MISSING_MAIN] missing `main` function")]
    MissingMain,
}

#[derive(Clone)]
struct Context {
    decls: HashMap<String, Type>,
}

impl Context {
    fn new() -> Self {
        Self {
            decls: HashMap::new(),
        }
    }

    fn add(&mut self, name: &str, type_: Type) {
        self.decls.insert(name.to_owned(), type_);
    }

    fn get(&self, name: &str) -> Result<&Type, TypeError> {
        match self.decls.get(name) {
            Some(type_) => Ok(type_),
            None => Err(TypeError::UndefinedVariable(name.to_owned())),
        }
    }

    fn with_params(&self, param_decls: &Vec<ParamDecl>) -> Self {
        let mut new_context = self.clone();
        param_decls
            .iter()
            .for_each(|param_decl| new_context.add(&param_decl.name, param_decl.type_.clone()));
        new_context
    }
}

fn infer(expr: &Expr, context: &Context) -> Result<Type, TypeError> {
    match expr {
        Expr::ConstTrue => Ok(Type::Bool),
        Expr::ConstFalse => Ok(Type::Bool),
        Expr::ConstInt(0) => Ok(Type::Nat),
        Expr::Sequence(expr, None) => infer(expr, context),
        Expr::Sequence(_, _) => {
            dbg!(expr);
            todo!("Oops... This is interesting `Sequence`")
        }
        Expr::Succ(expr) => {
            let type_ = infer(expr, context)?;
            if type_ != Type::Nat {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: Type::Nat,
                    actual: type_,
                });
            }
            Ok(Type::Nat)
        }
        Expr::NatIsZero(expr) => {
            let type_ = infer(expr, context)?;
            if type_ != Type::Nat {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: Type::Nat,
                    actual: type_,
                });
            }
            Ok(Type::Bool)
        }
        Expr::Var(name) => context.get(name).cloned(),
        Expr::If(cond, then, else_) => {
            let cond = infer(cond, context)?;
            if cond != Type::Bool {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: Type::Bool,
                    actual: cond,
                });
            }
            let then = infer(then, context)?;
            let else_ = infer(else_, context)?;

            match (&then, &else_) {
                (_, _) if then == else_ => Ok(then),
                (Type::Fun(_, _), Type::Fun(_, _)) => Err(TypeError::UnexpectedTypeForExpression {
                    expected: then,
                    actual: else_,
                }),
                (_, Type::Fun(_, _)) => Err(TypeError::UnexpectedLambda { expected: else_ }),
                _ => Err(TypeError::UnexpectedTypeForExpression {
                    expected: then,
                    actual: else_,
                }),
            }
        }
        Expr::Abstraction(params, return_) => {
            let local_context = context.with_params(params);
            let return_ = infer(return_, &local_context)?;
            Ok(Type::Fun(
                params.iter().map(|param| param.type_.clone()).collect(),
                Box::new(return_),
            ))
        }
        Expr::Application(fun, args) => {
            let (params, return_) = match infer(fun, context)? {
                Type::Fun(params, return_) => (params, return_),
                actual => return Err(TypeError::NotAFunction { actual }),
            };

            params
                .iter()
                .zip(args)
                .try_for_each(|(param, arg)| match_type(param, arg, context))?;
            Ok(*return_.clone())
        }
        Expr::NatRec(n, z, s) => {
            let n = infer(n, context)?;
            if n != Type::Nat {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: Type::Nat,
                    actual: n,
                });
            }

            let z = infer(z, context)?;
            let s = infer(s, context)?;

            let expected = Type::Fun(
                vec![Type::Nat],
                Box::new(Type::Fun(vec![z.clone()], Box::new(z.clone()))),
            );

            match (&expected, &s) {
                (_, _) if expected == s => Ok(z),
                (Type::Fun(_, _), Type::Fun(_, _)) => Err(TypeError::UnexpectedTypeForExpression {
                    expected,
                    actual: s,
                }),
                (_, Type::Fun(_, _)) => Err(TypeError::UnexpectedLambda { expected: s }),
                _ => Err(TypeError::UnexpectedTypeForExpression {
                    expected,
                    actual: s,
                }),
            }
        }
        _ => todo!("Oops... This is interesting `expr_type`"),
    }
}

fn match_type(expected: &Type, actual: &Expr, context: &Context) -> Result<(), TypeError> {
    match (actual, expected) {
        (Expr::ConstTrue | Expr::ConstFalse, Type::Bool) => Ok(()),
        (Expr::ConstInt(0), Type::Nat) => Ok(()),
        (Expr::Sequence(actual, None), expected) => match_type(expected, actual, context),
        (Expr::Sequence(_, _), _) => {
            dbg!(actual);
            todo!("Oops... This is interesting `Sequence`")
        }
        (Expr::Succ(expr), Type::Nat) => match_type(&Type::Nat, expr, context),
        (Expr::NatIsZero(expr), Type::Bool) => match_type(&Type::Nat, expr, context),
        (Expr::Var(name), expected) => {
            let actual = context.get(name)?;
            if actual != expected {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: actual.clone(),
                });
            } else {
                Ok(())
            }
        }
        (Expr::If(cond, then, else_), expected) => {
            match_type(&Type::Bool, cond, context)?;
            match_type(expected, then, context)?;
            match_type(expected, else_, context)?;
            Ok(())
        }
        (
            Expr::Abstraction(actual_params, actual_return),
            Type::Fun(expected_params, expected_return),
        ) => {
            let local_context = context.with_params(actual_params);
            expected_params
                .iter()
                .zip(actual_params.iter())
                .try_for_each(|(expected_param, actual_param)| {
                    if *expected_param != actual_param.type_ {
                        return Err(TypeError::UnexpectedTypeForParameter);
                    }
                    Ok(())
                })?;
            match_type(expected_return, actual_return, &local_context)
        }
        (Expr::Application(fun, args), expected) => {
            let (params, return_) = match infer(fun, context)? {
                Type::Fun(params, return_) => (params, return_),
                actual => return Err(TypeError::NotAFunction { actual }),
            };

            params
                .iter()
                .zip(args)
                .try_for_each(|(param, arg)| match_type(param, arg, context))?;

            if *return_ != *expected {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: *return_,
                });
            }
            dbg!("blalba2");
            Ok(())
        }
        (Expr::NatRec(n, z, s), expected) => {
            let fun = Type::Fun(
                vec![Type::Nat],
                Box::new(Type::Fun(
                    vec![expected.clone()],
                    Box::new(expected.clone()),
                )),
            );
            match_type(&fun, s, context)?;
            match_type(&Type::Nat, n, context)?;
            match_type(expected, z, context)
        }
        (Expr::Abstraction(_, _), expected) => Err(TypeError::UnexpectedLambda {
            expected: expected.clone(),
        }),
        _ => Err(TypeError::UnexpectedTypeForExpression {
            expected: expected.clone(),
            actual: infer(actual, context)?,
        }),
    }
}

fn typecheck_decl(decl: &Decl, context: &Context) -> Result<(), TypeError> {
    match decl {
        Decl::DeclFun {
            return_type: None, ..
        } => {
            todo!("Functions returning `Unit` are not implemented yet")
        }
        Decl::DeclFun {
            return_type: Some(expected),
            return_expr,
            param_decls,
            ..
        } => {
            let context = context.with_params(param_decls);
            match_type(expected, return_expr, &context)?;
            Ok(())
        }
        Decl::DeclTypeAlias { name: _, type_: _ } => {
            todo!("Type aliases are not implemented yet")
        }
    }
}

fn create_global_context(program: &Program) -> Context {
    let mut context = Context::new();

    for decl in &program.decls {
        match decl {
            Decl::DeclFun {
                return_type: None, ..
            } => {
                todo!("Functions returning `Unit` are not implemented yet")
            }
            Decl::DeclFun {
                name,
                return_type: Some(type_),
                param_decls,
                ..
            } => {
                let type_ = type_.clone();
                let fun = Type::Fun(
                    param_decls
                        .iter()
                        .map(|param_decl| param_decl.type_.clone())
                        .collect(),
                    Box::new(type_),
                );
                context.add(name, fun);
            }
            Decl::DeclTypeAlias { .. } => {
                todo!("Type aliases are not implemented yet")
            }
        }
    }

    context
}

pub fn typecheck_program(program: &Program) -> Result<(), TypeError> {
    let global_context = create_global_context(program);

    program
        .decls
        .iter()
        .try_for_each(|decl| typecheck_decl(decl, &global_context))?;

    match global_context.get("main") {
        Ok(_) => Ok(()),
        Err(_) => Err(TypeError::MissingMain),
    }
}
