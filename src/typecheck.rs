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

    #[error("([ERROR_INCORRECT_NUMBER_OF_ARGUMENTS] incorrect number of arguments (expected {expected:?}, found {actual:?})")]
    IncorrectNumberOfArguments { expected: usize, actual: usize },

    #[error("[ERROR_UNEXPECTED_NUMBER_OF_PARAMETERS_IN_LAMBDA] unexpected number of parameters in lambda (expected {expected:?}, found {actual:?})")]
    UnexpectedNumberOfParametersInLambda { expected: usize, actual: usize },

    #[error("[ERROR_UNDEFINED_VARIABLE] undefined variable `{0}`")]
    UndefinedVariable(String),

    #[error("[ERROR_INCORRECT_ARITY_OF_MAIN] incorrect arity of `main` function")]
    IncorrectArityOfMain,

    #[error("[ERROR_MISSING_MAIN] missing `main` function")]
    MissingMain,

    #[error("[ERROR_MISSING_RECORD_FIELDS] record is missing one or more of the expected fields")]
    MissingRecordFields,

    #[error("[ERROR_UNEXPECTED_RECORD_FIELDS] record has one or more unexpected fields")]
    UnexpectedRecordFields,

    #[error("[ERROR_UNEXPECTED_FIELD_ACCESS] unexpected field access where an expression of a non-record type is expected")]
    UnexpectedFieldAccess,

    #[error("[ERROR_UNEXPECTED_RECORD] unexpected record where an expression of a non-record type is expected")]
    UnexpectedRecord,

    #[error("[ERROR_DUPLICATED_RECORD_FIELD] duplicated record field `{0}`")]
    DuplicatedRecordField(String),

    #[error("[ERROR_NOT_A_RECORD] unexpected expression where a record is expected")]
    NotARecord,

    #[error("[ERROR_UNEXPECTED_TUPLE] unexpected tuple/pair where an expression of a non-tuple type is expected")]
    UnexpectedTuple,

    #[error("[ERROR_UNEXPECTED_TUPLE_LENGTH] unexpected tuple/pair length (expected {0:?}, found {1:?})")]
    UnexpectedTupleLength(usize, usize),

    #[error("[ERROR_TUPLE_INDEX_OUT_OF_BOUNDS] tuple/pair index out of bounds (expected {0:?}, found {1:?})")]
    TupleIndexOutOfBounds(usize, usize),

    #[error("[ERROR_NOT_A_TUPLE] unexpected expression where a tuple/pair is expected")]
    NotATuple,
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

    fn with_params(&self, param_decls: &[ParamDecl]) -> Self {
        let mut new_context = self.clone();
        param_decls
            .iter()
            .for_each(|param_decl| new_context.add(&param_decl.name, param_decl.type_.clone()));
        new_context
    }

    fn add_decl(&mut self, decl: &Decl) {
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
                self.add(&name, fun);
            }
            Decl::DeclTypeAlias { .. } => {
                todo!("Type aliases are not implemented yet")
            }
        }
    }
}

fn typecheck_params(
    params: &Vec<Type>,
    args: &Vec<Expr>,
    context: &Context,
) -> Result<(), TypeError> {
    if params.len() != args.len() {
        return Err(TypeError::IncorrectNumberOfArguments {
            expected: params.len(),
            actual: args.len(),
        });
    }
    params
        .iter()
        .zip(args)
        .try_for_each(|(param, arg)| match_type(param, arg, context))
}

fn check_record_fields(fields: &Vec<RecordFieldType>) -> Result<(), TypeError> {
    fields.iter().try_for_each(|field| {
        if fields.iter().filter(|f| f.label == field.label).count() > 1 {
            return Err(TypeError::DuplicatedRecordField(field.label.clone()));
        }
        Ok(())
    })
}

fn context_with_pattern_bindigs(
    context: &Context,
    pattern_bindigs: &Vec<PatternBinding>,
) -> Result<Context, TypeError> {
    let mut new_context = context.clone();
    for binding in pattern_bindigs {
        match &binding.pattern {
            Pattern::Var(name) => {
                let type_ = infer(&binding.rhs, &context)?;
                new_context.add(name, type_);
            }
            _ => todo!("Oops... This is interesting `Let`"),
        }
    }
    Ok(new_context)
}

fn infer(expr: &Expr, context: &Context) -> Result<Type, TypeError> {
    match expr {
        Expr::ConstTrue => Ok(Type::Bool),
        Expr::ConstFalse => Ok(Type::Bool),
        Expr::ConstUnit => Ok(Type::Unit),
        Expr::ConstInt(_) => Ok(Type::Nat),
        Expr::Sequence(expr, None) => infer(expr, context),
        Expr::Sequence(_, _) => {
            dbg!(expr);
            todo!("Oops... This is interesting `Sequence`")
        }
        Expr::Succ(expr) => {
            match_type(&Type::Nat, expr, context)?;
            Ok(Type::Nat)
        }
        Expr::NatIsZero(expr) => {
            match_type(&Type::Nat, expr, context)?;
            Ok(Type::Bool)
        }
        Expr::Var(name) => context.get(name).cloned(),
        Expr::If(cond, then, else_) => {
            match_type(&Type::Bool, cond, context)?;
            let then = infer(then, context)?;
            match_type(&then, else_, context)?;
            Ok(then)
        }

        Expr::Tuple(exprs) => Ok(Type::Tuple(
            exprs
                .iter()
                .map(|expr| infer(expr, context))
                .collect::<Result<_, _>>()?,
        )),
        Expr::DotTuple(expr, index) => {
            let Type::Tuple(fields) = infer(&expr, context)? else {
                dbg!(expr);
                return Err(TypeError::NotATuple);
            };

            let field = fields.get(index - 1);
            match field {
                Some(field) => Ok(field.clone()),
                None => Err(TypeError::TupleIndexOutOfBounds(fields.len(), *index)),
            }
        }

        Expr::Record(fields) => {
            let fields = fields
                .iter()
                .map(|field| {
                    Ok(RecordFieldType {
                        label: field.name.clone(),
                        type_: infer(&field.expr, context)?,
                    })
                })
                .collect::<Result<_, _>>()?;
            check_record_fields(&fields)?;
            Ok(Type::Record(fields))
        }
        Expr::DotRecord(expr, name) => {
            let Type::Record(fields) = infer(&expr, context)? else {
                dbg!(expr);
                return Err(TypeError::NotARecord);
            };

            let field = fields
                .iter()
                .find(|field| field.label == *name)
                .ok_or(TypeError::UnexpectedFieldAccess)?;
            Ok(field.type_.clone())
        }

        Expr::Let(bindings, expr) => {
            let context = context_with_pattern_bindigs(context, bindings)?;
            infer(expr, &context)
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

            typecheck_params(&params, args, context)?;
            Ok(*return_.clone())
        }
        Expr::NatRec(n, z, s) => {
            let z = infer(z, context)?;
            let fun = Type::Fun(
                vec![Type::Nat],
                Box::new(Type::Fun(vec![z.clone()], Box::new(z.clone()))),
            );
            match_type(&fun, s, context)?;
            match_type(&Type::Nat, n, context)?;
            Ok(z)
        }
        _ => todo!("Oops... This is interesting `expr_type`"),
    }
}

fn match_type(expected: &Type, actual: &Expr, context: &Context) -> Result<(), TypeError> {
    match (actual, expected) {
        (Expr::ConstTrue | Expr::ConstFalse, Type::Bool) => Ok(()),
        (Expr::ConstUnit, Type::Unit) => Ok(()),
        (Expr::ConstInt(_), Type::Nat) => Ok(()),
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
                Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: actual.clone(),
                })
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

        (Expr::Tuple(actual_fields), Type::Tuple(expected_fields)) => {
            if actual_fields.len() != expected_fields.len() {
                return Err(TypeError::UnexpectedTupleLength(
                    expected_fields.len(),
                    actual_fields.len(),
                ));
            }
            actual_fields
                .iter()
                .zip(expected_fields)
                .try_for_each(|(expr, expected)| match_type(expected, expr, context))
        }
        (Expr::Tuple(_), _) => Err(TypeError::UnexpectedTuple),
        (Expr::DotTuple(expr, index), expected) => {
            let Type::Tuple(fields) = infer(&expr, context)? else {
                dbg!(expr);
                return Err(TypeError::NotATuple);
            };

            let field = fields.get(index - 1);
            match field {
                Some(field) if field == expected => Ok(()),
                Some(field) => Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: field.clone(),
                }),
                None => Err(TypeError::TupleIndexOutOfBounds(fields.len(), *index)),
            }
        }

        (Expr::Record(actual_fields), Type::Record(expected_fields)) => {
            if actual_fields.len() != expected_fields.len() {
                return Err(TypeError::UnexpectedRecordFields);
            }
            check_record_fields(expected_fields)?;
            expected_fields.iter().try_for_each(|expected| {
                let actual = actual_fields
                    .iter()
                    .find(|actual| actual.name == expected.label)
                    .ok_or(TypeError::MissingRecordFields)?;
                match_type(&expected.type_, &actual.expr, context)
            })?;

            // todo: надо или нет?
            if expected_fields.is_empty() {
                return Err(TypeError::MissingRecordFields);
            }
            Ok(())
        }
        (Expr::Record(_), _) => Err(TypeError::UnexpectedRecord),
        (Expr::DotRecord(expr, name), expected) => {
            let Type::Record(fields) = infer(&expr, context)? else {
                dbg!(expr);
                return Err(TypeError::NotARecord);
            };

            let field = fields
                .iter()
                .find(|field| field.label == *name)
                .ok_or(TypeError::UnexpectedFieldAccess)?;
            if expected != &field.type_ {
                Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: field.type_.clone(),
                })
            } else {
                Ok(())
            }
        }

        (Expr::Let(bindings, expr), expected) => {
            let context = context_with_pattern_bindigs(context, bindings)?;
            match_type(expected, expr, &context)
        }

        (
            Expr::Abstraction(actual_params, actual_return),
            Type::Fun(expected_params, expected_return),
        ) => {
            let local_context = context.with_params(actual_params);

            if expected_params.len() != actual_params.len() {
                return Err(TypeError::UnexpectedNumberOfParametersInLambda {
                    expected: actual_params.len(),
                    actual: expected_params.len(),
                });
            }

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

            typecheck_params(&params, args, context)?;

            if *return_ != *expected {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: *return_,
                });
            }
            Ok(())
        }
        (Expr::NatRec(n, z, s), expected) => {
            match_type(&Type::Nat, n, context)?;
            let fun = Type::Fun(
                vec![Type::Nat],
                Box::new(Type::Fun(
                    vec![expected.clone()],
                    Box::new(expected.clone()),
                )),
            );
            match_type(expected, z, context)?;
            match_type(&fun, s, context)
        }
        (Expr::Abstraction(_, _), expected) => Err(TypeError::UnexpectedLambda {
            expected: expected.clone(),
        }),
        (expr, _) => {
            let actual = infer(expr, context)?;
            Err(TypeError::UnexpectedTypeForExpression {
                expected: expected.clone(),
                actual,
            })
        }
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
            local_decls,
            ..
        } => {
            let mut context = context.with_params(param_decls);
            for decl in local_decls {
                typecheck_decl(decl, &context)?;
                context.add_decl(decl);
            }
            match_type(expected, return_expr, &context)?;
            Ok(())
        }
        Decl::DeclTypeAlias { name: _, type_: _ } => {
            todo!("Type aliases are not implemented yet")
        }
    }
}

pub fn typecheck_program(program: &Program) -> Result<(), TypeError> {
    let mut global_context = Context::new();
    for decl in &program.decls {
        global_context.add_decl(decl);
    }

    program
        .decls
        .iter()
        .try_for_each(|decl| typecheck_decl(decl, &global_context))?;

    match global_context.get("main") {
        Ok(Type::Fun(params, _)) if params.len() == 1 => Ok(()),
        Ok(_) => Err(TypeError::IncorrectArityOfMain),
        Err(_) => Err(TypeError::MissingMain),
    }
}
