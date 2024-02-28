use std::collections::HashMap;
use thiserror::Error;

use crate::ast::*;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("[ERROR_UNEXPECTED_TYPE_FOR_PARAMETER] unexpected type for parameter")]
    UnexpectedTypeForParameter,

    #[error("[ERROR_UNEXPECTED_TYPE_FOR_EXPRESSION] unexpected type for expression (expected {expected:?}, found {actual:?})")]
    UnexpectedTypeForExpression {
        expected: Type,
        actual: Option<Type>,
    },

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

    #[error("[ERROR_UNEXPECTED_PATTERN_FOR_TYPE] unexpected pattern for type (expected {0:?}, found {1:?})")]
    UnexpectedPatternForType(Type, Pattern),

    #[error("[ERROR_AMBIGUOUS_SUM_TYPE] type inference of sum type failed (use type ascriptions)")]
    AmbiguousSumType,

    #[error("[ERROR_AMBIGUOUS_LIST_TYPE]")]
    AmbiguousListType,
    #[error("[ERROR_ILLEGAL_EMPTY_MATCHING]")]
    IllegalEmptyMatching,
    #[error("[ERROR_NONEXHAUSTIVE_MATCH_PATTERNS]")]
    NonexhaustiveMatchPatterns,
    #[error("[ERROR_NOT_A_LIST]")]
    NotAList,
    #[error("[ERROR_UNEXPECTED_LIST]")]
    UnexpectedList,
    #[error("[ERROR_UNEXPECTED_INJECTION]")]
    UnexpectedInjection,
    #[error("[ERROR_AMBIGUOUS_VARIANT_TYPE]")]
    AmbiguousVariantType,
    #[error("[ERROR_UNEXPECTED_VARIANT]")]
    UnexpectedVariant,
    #[error("[ERROR_UNEXPECTED_VARIANT_LABEL]")]
    UnexpectedVariantLabel,
    #[error("[ERROR_UNEXPECTED_DATA_FOR_NULLARY_LABEL]")]
    UnexpectedDataForNullaryLabel,
    #[error("[ERROR_MISSING_DATA_FOR_LABEL]")]
    MissingDataForLabel,
    #[error("[ERROR_UNEXPECTED_NULLARY_VARIANT_PATTERN]")]
    UnexpectedNullaryVariantPattern,
    #[error("[ERROR_UNEXPECTED_NON_NULLARY_VARIANT_PATTERN]")]
    UnexpectedNonNullaryVariantPattern,

    #[error("[ERROR_EXCEPTION_TYPE_NOT_DECLARED]")]
    ExceptionTypeNotDeclared,
    #[error("[ERROR_AMBIGUOUS_THROW_TYPE]")]
    AmbiguousThrowType,
    #[error("[ERROR_AMBIGUOUS_REFERENCE_TYPE]")]
    AmbiguousReferenceType,
    #[error("[ERROR_AMBIGUOUS_PANIC_TYPE]")]
    AmbiguousPanicType,
    #[error("[ERROR_NOT_A_REFERENCE]")]
    NotAReference,
    #[error("[ERROR_UNEXPECTED_MEMORY_ADDRESS]")]
    UnexpectedMemoryAddress,
}

#[derive(Clone)]
struct Context(HashMap<String, Type>);

impl Context {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add(&mut self, name: &str, type_: Type) {
        self.0.insert(name.to_owned(), type_);
    }

    fn get(&self, name: &str) -> Result<&Type, TypeError> {
        match self.0.get(name) {
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

    fn add_pattern(&mut self, pattern: &Pattern, type_: &Type) -> Result<(), TypeError> {
        match (type_, pattern) {
            (Type::Nat, Pattern::Int(_)) => Ok(()),
            (Type::Nat, Pattern::Succ(nat)) => self.add_pattern(nat, &Type::Nat),
            (Type::Bool, Pattern::False | Pattern::True) => Ok(()),
            (Type::Unit, Pattern::Unit) => Ok(()),
            (type_, Pattern::Var(name)) => {
                self.add(name, type_.clone());
                Ok(())
            }
            (Type::Tuple(actual_fields), Pattern::Tuple(pattern_fields)) => {
                if actual_fields.len() != pattern_fields.len() {
                    return Err(TypeError::UnexpectedPatternForType(
                        type_.clone(),
                        pattern.clone(),
                    ));
                }
                actual_fields
                    .iter()
                    .zip(pattern_fields)
                    .try_for_each(|(type_, pattern)| self.add_pattern(pattern, type_))
            }
            (Type::Record(actual_fields), Pattern::Record(pattern_fields)) => {
                actual_fields.iter().try_for_each(|field| {
                    let pattern = pattern_fields
                        .iter()
                        .find(|pattern| pattern.label == field.label)
                        .ok_or(TypeError::UnexpectedPatternForType(
                            type_.clone(),
                            pattern.clone(),
                        ))?;
                    self.add_pattern(&pattern.pattern, &field.type_)
                })
            }
            (Type::Sum(left, _), Pattern::Inl(pattern)) => self.add_pattern(pattern, left),
            (Type::Sum(_, right), Pattern::Inr(pattern)) => self.add_pattern(pattern, right),
            (Type::List(inner), Pattern::Cons(head, tail)) => {
                self.add_pattern(head, inner)?;
                self.add_pattern(tail, &type_)
            }
            (Type::List(inner), Pattern::List(patterns)) => patterns
                .iter()
                .try_for_each(|pattern| self.add_pattern(pattern, inner)),
            (Type::Variant(fields), Pattern::Variant(label, inner)) => {
                let field = fields.iter().find(|field| field.label == *label).ok_or(
                    TypeError::UnexpectedPatternForType(type_.clone(), pattern.clone()),
                )?;
                match (inner, field.type_.as_ref()) {
                    (Some(pattern), Some(type_)) => self.add_pattern(pattern, type_),
                    (None, None) => Ok(()),
                    (_, None) => Err(TypeError::UnexpectedNonNullaryVariantPattern),
                    (None, _) => Err(TypeError::UnexpectedNullaryVariantPattern),
                }
            }
            _ => {
                dbg!(
                    "Oops... This is interesting `Pattern` {} {}",
                    pattern,
                    type_
                );
                Err(TypeError::UnexpectedPatternForType(
                    type_.clone(),
                    pattern.clone(),
                ))
            }
        }
    }

    fn with_pattern(&self, matched: &Type, pattern: &Pattern) -> Result<Context, TypeError> {
        let mut new_context = self.clone();
        new_context.add_pattern(pattern, matched)?;
        Ok(new_context)
    }

    fn with_let_bindigs(&self, let_bindings: &Vec<PatternBinding>) -> Result<Context, TypeError> {
        let mut new_context = self.clone();
        for binding in let_bindings {
            let type_ = infer(&binding.rhs, &new_context)?;
            new_context.add_pattern(&binding.pattern, &type_)?;
        }
        Ok(new_context)
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
            Decl::DeclGenericFun { .. } => {
                todo!("Generic functions are not implemented yet")
            }
            Decl::DeclExceptionType(_) => todo!(),
            Decl::DeclExceptionVariant { .. } => todo!(),
        }
    }
}

fn check_params(params: &Vec<Type>, args: &Vec<Expr>, context: &Context) -> Result<(), TypeError> {
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

/// Check if the match is exhaustive for the given type
///
/// todo: it is actually not a correct match at all!!!!
fn check_exhaustiveness(matched: &Type, cases: &Vec<MatchCase>) -> Result<(), TypeError> {
    fn unit_or_error(cond: bool) -> Result<(), TypeError> {
        if cond {
            Ok(())
        } else {
            Err(TypeError::NonexhaustiveMatchPatterns)
        }
    }
    match matched {
        Type::Unit => {
            for case in cases {
                match case.pattern {
                    Pattern::Unit => return Ok(()),
                    Pattern::Var(_) => return Ok(()),
                    _ => {}
                }
            }
            Err(TypeError::NonexhaustiveMatchPatterns)
        }
        Type::Bool => {
            let mut false_ = false;
            let mut true_ = false;
            for case in cases {
                match case.pattern {
                    Pattern::False => false_ = true,
                    Pattern::True => true_ = true,
                    Pattern::Var(_) => return Ok(()),
                    _ => {}
                }
            }
            unit_or_error(false_ && true_)
        }
        Type::Nat => {
            for case in cases {
                match case.pattern {
                    Pattern::Var(_) => return Ok(()),
                    Pattern::Succ(_) => return Ok(()),
                    _ => {}
                }
            }
            Err(TypeError::NonexhaustiveMatchPatterns)
        }
        Type::Sum(_, _) => {
            let mut left = false;
            let mut right = false;
            for case in cases {
                match case.pattern {
                    Pattern::Inl(_) => left = true,
                    Pattern::Inr(_) => right = true,
                    Pattern::Var(_) => return Ok(()),
                    _ => {}
                }
            }
            unit_or_error(left && right)
        }
        Type::List(_) => {
            let mut empty = false;
            let mut head = false;
            for case in cases {
                match &case.pattern {
                    Pattern::Cons(_, _) => head = true,
                    Pattern::List(list) => {
                        if list.is_empty() {
                            empty = true;
                        }
                    }
                    Pattern::Var(_) => return Ok(()),
                    _ => {}
                }
            }
            unit_or_error(empty && head)
        }
        Type::Variant(fields) => {
            let mut variants = fields
                .iter()
                .map(|field| field.label.clone())
                .collect::<Vec<_>>();
            for case in cases {
                match &case.pattern {
                    Pattern::Variant(label, _) => {
                        if let Some(index) = variants.iter().position(|v| v == label) {
                            variants.remove(index);
                        }
                    }
                    Pattern::Var(_) => return Ok(()),
                    _ => {}
                }
            }
            unit_or_error(variants.is_empty())
        }
        Type::Tuple(_) => {
            for case in cases {
                match case.pattern {
                    Pattern::Tuple(_) => return Ok(()),
                    Pattern::Var(_) => return Ok(()),
                    _ => {}
                }
            }
            Err(TypeError::NonexhaustiveMatchPatterns)
        }
        _ => Ok(()),
    }
}

fn infer(expr: &Expr, context: &Context) -> Result<Type, TypeError> {
    match expr {
        Expr::ConstTrue => Ok(Type::Bool),
        Expr::ConstFalse => Ok(Type::Bool),
        Expr::ConstUnit => Ok(Type::Unit),
        Expr::ConstInt(_) => Ok(Type::Nat),
        Expr::Sequence(preceding, following) => {
            match_type(&Type::Unit, preceding, context)?;
            infer(following, context)
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
                return Err(TypeError::NotARecord);
            };

            let field = fields
                .iter()
                .find(|field| field.label == *name)
                .ok_or(TypeError::UnexpectedFieldAccess)?;
            Ok(field.type_.clone())
        }

        Expr::Let(bindings, expr) => {
            let context = context.with_let_bindigs(bindings)?;
            infer(expr, &context)
        }
        Expr::LetRec(bindings, expr) => {
            let context = context.with_let_bindigs(bindings)?;
            infer(expr, &context)
        }

        Expr::Match(expr, cases) => {
            let matched = infer(expr, context)?;

            let first = {
                let case = cases.first().ok_or(TypeError::IllegalEmptyMatching)?;
                let local = context.with_pattern(&matched, &case.pattern)?;
                infer(&case.expr, &local)
            }?;

            cases[1..].iter().try_for_each(|case| {
                let local = context.with_pattern(&matched, &case.pattern)?;
                match_type(&first, &case.expr, &local)
            })?;

            check_exhaustiveness(&matched, cases)?;
            Ok(first)
        }
        Expr::TypeAscription(expr, asc) => {
            match_type(asc, expr, context)?;
            Ok(asc.clone())
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
            check_params(&params, args, context)?;
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
        Expr::Fix(expr) => {
            let Type::Fun(params, _) = infer(expr, context)? else {
                return Err(TypeError::NotAFunction {
                    actual: Type::Fun(vec![], Box::new(Type::Unit)),
                });
            };

            let param = params
                .first()
                .ok_or_else(|| TypeError::NotAFunction { actual: Type::Bool })?;

            if !params[1..].is_empty() {
                return Err(TypeError::NotAFunction { actual: Type::Bool });
            }

            let fun = Type::Fun(vec![param.clone()], Box::new(param.clone()));
            match_type(&fun, expr, context)?;
            Ok(param.clone())
        }
        Expr::Inl(_) | Expr::Inr(_) => Err(TypeError::AmbiguousSumType),
        Expr::Variant(..) => Err(TypeError::AmbiguousVariantType),
        Expr::List(exprs) => {
            let first = exprs.first().ok_or(TypeError::AmbiguousListType)?;
            let first = infer(first, context)?;
            exprs[1..]
                .iter()
                .try_for_each(|expr| match_type(&first, expr, context))?;
            Ok(Type::List(Box::new(first)))
        }
        Expr::Cons(head, tail) => {
            let head = infer(head, context)?;
            let list = Type::List(Box::new(head));
            match_type(&list, tail, context)?;
            Ok(list)
        }
        Expr::ListHead(expr) | Expr::ListTail(expr) => {
            let Type::List(type_) = infer(expr, context)? else {
                return Err(TypeError::NotAList);
            };
            Ok(*type_.clone())
        }
        Expr::ListIsEmpty(expr) => {
            let Type::List(_) = infer(expr, context)? else {
                return Err(TypeError::NotAList);
            };
            Ok(Type::Bool)
        }
        Expr::ConstMemory(_) => Err(TypeError::AmbiguousReferenceType),
        Expr::Reference(expr) => {
            let type_ = infer(expr, context)?;
            Ok(Type::Ref(Box::new(type_)))
        }
        Expr::Dereference(expr) => {
            let Type::Ref(type_) = infer(expr, context)? else {
                return Err(TypeError::NotAReference);
            };
            Ok(*type_.clone())
        }
        Expr::Assignment(reference, value) => {
            let Type::Ref(type_) = infer(reference, context)? else {
                return Err(TypeError::NotAReference);
            };
            match_type(&type_, value, context)?;
            Ok(Type::Unit)
        }

        _ => unreachable!(),
    }
}

fn match_type(expected: &Type, actual: &Expr, context: &Context) -> Result<(), TypeError> {
    match (actual, expected) {
        (Expr::ConstTrue | Expr::ConstFalse, Type::Bool) => Ok(()),
        (Expr::ConstUnit, Type::Unit) => Ok(()),
        (Expr::ConstInt(_), Type::Nat) => Ok(()),
        (Expr::Sequence(preceding, following), _expected) => {
            match_type(&Type::Unit, preceding, context)?;
            match_type(expected, following, context)
        }
        (Expr::Succ(expr), Type::Nat) => match_type(&Type::Nat, expr, context),
        (Expr::NatIsZero(expr), Type::Bool) => match_type(&Type::Nat, expr, context),
        (Expr::Var(name), expected) => {
            let actual = context.get(name)?;
            if actual != expected {
                Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: Some(actual.clone()),
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
                return Err(TypeError::NotATuple);
            };

            let field = fields.get(index - 1);
            match field {
                Some(field) if field == expected => Ok(()),
                Some(field) => Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: Some(field.clone()),
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
                return Err(TypeError::NotARecord);
            };

            let field = fields
                .iter()
                .find(|field| field.label == *name)
                .ok_or(TypeError::UnexpectedFieldAccess)?;
            if expected != &field.type_ {
                Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: Some(field.type_.clone()),
                })
            } else {
                Ok(())
            }
        }

        (Expr::Let(bindings, expr), expected) => {
            let context = context.with_let_bindigs(bindings)?;
            match_type(expected, expr, &context)
        }
        (Expr::LetRec(bindings, expr), expected) => {
            let context = context.with_let_bindigs(bindings)?;
            match_type(expected, expr, &context)
        }

        (Expr::Match(_, cases), _) if cases.is_empty() => Err(TypeError::IllegalEmptyMatching),
        (Expr::Match(matched, cases), expected) => {
            let matched = infer(matched, context)?;
            cases.iter().try_for_each(|case| {
                let local = context.with_pattern(&matched, &case.pattern)?;
                match_type(expected, &case.expr, &local)
            })?;
            check_exhaustiveness(&matched, cases)
        }

        (Expr::Inl(expr), Type::Sum(left, _)) => match_type(left, expr, context),
        (Expr::Inr(expr), Type::Sum(_, right)) => match_type(right, expr, context),
        (Expr::Inl(_), _) | (Expr::Inr(_), _) => Err(TypeError::UnexpectedInjection),
        (Expr::TypeAscription(expr, asc), expected) if asc == expected => {
            match_type(asc, expr, context)
        }

        (Expr::List(exprs), Type::List(expected)) => exprs
            .iter()
            .try_for_each(|expr| match_type(expected, expr, context)),
        (Expr::Cons(head, tail), Type::List(inner)) => {
            match_type(inner, head, context)?;
            match_type(expected, tail, context)
        }
        (Expr::List(_), _) => Err(TypeError::UnexpectedList),
        (Expr::ListHead(expr), expected) => {
            let expected = Type::List(Box::new(expected.clone()));
            match_type(&expected, expr, context)
        }
        (Expr::ListTail(list), expected) => {
            // todo: if they change again to NOT_A_LIST
            // let Type::List(_) = infer(list, context)? else {
            //     dbg!(list);
            //     return Err(TypeError::NotAList);
            // };
            match_type(expected, list, context)
        }
        (Expr::ListIsEmpty(expr), Type::Bool) => {
            let Type::List(_) = infer(expr, context)? else {
                return Err(TypeError::NotAList);
            };
            Ok(())
        }
        (
            Expr::Abstraction(actual_params, actual_return),
            Type::Fun(expected_params, expected_return),
        ) => {
            let local = context.with_params(actual_params);

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
            match_type(expected_return, actual_return, &local)
        }
        (Expr::Abstraction(_, _), expected) => Err(TypeError::UnexpectedLambda {
            expected: expected.clone(),
        }),

        (Expr::Variant(label, expr), Type::Variant(fields)) => {
            let field = fields
                .iter()
                .find(|field| field.label == *label)
                .ok_or(TypeError::UnexpectedVariantLabel)?;
            match (expr, field.type_.as_ref()) {
                (Some(actual), Some(expected)) => match_type(expected, actual, context),
                (None, None) => Ok(()),
                (_, None) => Err(TypeError::UnexpectedDataForNullaryLabel),
                (None, _) => Err(TypeError::MissingDataForLabel),
            }
        }
        (Expr::Variant(_, _), _) => Err(TypeError::UnexpectedVariant),

        (Expr::ConstMemory(_), Type::Ref(_)) => Ok(()),
        (Expr::ConstMemory(_), _) => Err(TypeError::UnexpectedMemoryAddress),
        (Expr::Reference(actual), Type::Ref(expected)) => match_type(expected, actual, context),
        (Expr::Dereference(actual), expected) => {
            let expected = Type::Ref(Box::new(expected.clone()));
            match_type(&expected, actual, context)
        }

        (Expr::Assignment(reference, value), Type::Unit) => {
            let Type::Ref(value_type) = infer(reference, context)? else {
                dbg!(reference);
                return Err(TypeError::NotAReference);
            };
            match_type(&value_type, value, context)
        }

        (Expr::Application(fun, args), expected) => {
            let (params, return_) = match infer(fun, context)? {
                Type::Fun(params, return_) => (params, return_),
                actual => return Err(TypeError::NotAFunction { actual }),
            };

            check_params(&params, args, context)?;

            if *return_ != *expected {
                return Err(TypeError::UnexpectedTypeForExpression {
                    expected: expected.clone(),
                    actual: Some(*return_),
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
        (Expr::Fix(expr), expected) => {
            let Type::Fun(..) = infer(expr, context)? else {
                return Err(TypeError::NotAFunction {
                    actual: Type::Fun(vec![], Box::new(Type::Unit)),
                });
            };
            let fun = Type::Fun(vec![expected.clone()], Box::new(expected.clone()));
            match_type(&fun, expr, context)
        }
        _ => Err(TypeError::UnexpectedTypeForExpression {
            expected: expected.clone(),
            actual: None,
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
        Decl::DeclGenericFun { .. } => {
            todo!("Generic functions are not implemented yet")
        }
        Decl::DeclExceptionType(_) => todo!(),
        Decl::DeclExceptionVariant { .. } => todo!(),
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
