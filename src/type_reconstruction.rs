use crate::ast::*;
use crate::typecheck::{get_exception_type, Context, TypeError};
use std::collections::HashMap;
use std::vec;

#[derive(Debug)]
pub struct Constraints {
    map: HashMap<TypeVarID, Type>,
    next_id: TypeVarID,
}

impl Constraints {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    fn new_var(&mut self) -> TypeVarID {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn substitute(&self, type_: &Type) -> Type {
        self.map.iter().fold(type_.clone(), |type_, (k, v)| {
            substitute_type_var(&type_, k, v)
        })
    }

    fn update(&mut self, id: TypeVarID, new: &Type) {
        dbg!(id, new);
        dbg!(&self.map);
        self.map.insert(id, new.clone());
        self.map = self
            .map
            .clone()
            .into_iter()
            .map(|(k, v)| (k, substitute_type_var(&v, &id, &new)))
            .collect();
        dbg!(&self.map);
    }
}

fn substitute_type_var(type_: &Type, id: &TypeVarID, value: &Type) -> Type {
    match type_ {
        Type::Bool | Type::Nat | Type::Unit | Type::Var(_) => type_.clone(),
        Type::Fun(args, return_) => {
            let args = args
                .into_iter()
                .map(|arg| substitute_type_var(arg, id, value))
                .collect();
            let return_ = substitute_type_var(return_, id, value);
            Type::Fun(args, Box::new(return_))
        }
        Type::Tuple(fields) => {
            let fields = fields
                .into_iter()
                .map(|field| substitute_type_var(field, id, value))
                .collect();
            Type::Tuple(fields)
        }
        Type::Record(fields) => {
            let fields = fields
                .into_iter()
                .map(|field| RecordFieldType {
                    label: field.label.clone(),
                    type_: substitute_type_var(&field.type_, id, value),
                })
                .collect();
            Type::Record(fields)
        }
        Type::List(inner) => {
            let inner = Box::new(substitute_type_var(inner, id, value));
            Type::List(inner)
        }
        Type::Sum(left, right) => {
            let left = Box::new(substitute_type_var(left, id, value));
            let right = Box::new(substitute_type_var(right, id, value));
            Type::Sum(left, right)
        }
        Type::Variant(fields) => {
            let fields = fields
                .into_iter()
                .map(|field| VariantFieldType {
                    label: field.label.clone(),
                    type_: field
                        .type_
                        .as_ref()
                        .map(|type_| substitute_type_var(type_, id, value)),
                })
                .collect();
            Type::Variant(fields)
        }
        Type::Ref(inner) => {
            let inner = Box::new(substitute_type_var(inner, id, value));
            Type::Ref(inner)
        }
        Type::TypeVar(id_) => {
            if id == id_ {
                value.clone()
            } else {
                type_.clone()
            }
        }
        Type::ForAll(_, _) | Type::Rec(_, _) | Type::Top | Type::Bottom | Type::Auto => {
            unimplemented!("{:?}", type_)
        }
    }
}

pub fn decl_with_type_variables(decl: &Decl, context: &Context) -> Decl {
    match decl {
        Decl::DeclFun {
            annotations,
            name,
            param_decls,
            return_type,
            throws_types,
            local_decls,
            return_expr,
        } => Decl::DeclFun {
            annotations: annotations.clone(),
            name: name.clone(),
            param_decls: param_decls
                .iter()
                .map(|param_decl| {
                    let ParamDecl { name, type_ } = param_decl;
                    ParamDecl {
                        name: name.clone(),
                        type_: replace_auto_with_vars_in_type(type_, context),
                    }
                })
                .collect(),
            return_type: return_type
                .as_ref()
                .map(|type_| replace_auto_with_vars_in_type(type_, context)),
            throws_types: throws_types
                .iter()
                .map(|type_| replace_auto_with_vars_in_type(type_, context))
                .collect(),
            local_decls: local_decls
                .iter()
                .map(|decl| decl_with_type_variables(decl, context))
                .collect(),
            return_expr: replace_auto_with_vars_in_expr(return_expr, context),
        },
        Decl::DeclExceptionType(type_) => {
            Decl::DeclExceptionType(replace_auto_with_vars_in_type(type_, context))
        }
        Decl::DeclExceptionVariant { name, type_ } => Decl::DeclExceptionVariant {
            name: name.clone(),
            type_: replace_auto_with_vars_in_type(type_, context),
        },
        Decl::DeclGenericFun { .. } => unimplemented!(),
        Decl::DeclTypeAlias { .. } => unimplemented!(),
    }
}

// TODO: PATTERNS ALOOOOOO
fn replace_auto_with_vars_in_expr(expr: &Expr, context: &Context) -> Expr {
    match expr {
        Expr::ConstUnit
        | Expr::ConstFalse
        | Expr::ConstTrue
        | Expr::ConstInt(_)
        | Expr::Var(_)
        | Expr::ConstMemory(_)
        | Expr::Panic => expr.clone(),
        Expr::Sequence(preceding, following) => Expr::Sequence(
            Box::new(replace_auto_with_vars_in_expr(preceding, context)),
            Box::new(replace_auto_with_vars_in_expr(following, context)),
        ),
        Expr::Succ(expr) => Expr::Succ(Box::new(replace_auto_with_vars_in_expr(expr, context))),
        Expr::NatIsZero(expr) => {
            Expr::NatIsZero(Box::new(replace_auto_with_vars_in_expr(expr, context)))
        }
        Expr::If(condition, then_, else_) => Expr::If(
            Box::new(replace_auto_with_vars_in_expr(condition, context)),
            Box::new(replace_auto_with_vars_in_expr(then_, context)),
            Box::new(replace_auto_with_vars_in_expr(else_, context)),
        ),
        Expr::Abstraction(params, return_expr) => Expr::Abstraction(
            params
                .iter()
                .map(|ParamDecl { name, type_ }| ParamDecl {
                    name: name.clone(),
                    type_: replace_auto_with_vars_in_type(type_, context),
                })
                .collect(),
            Box::new(replace_auto_with_vars_in_expr(return_expr, context)),
        ),
        Expr::Application(fun_actual, args) => Expr::Application(
            Box::new(replace_auto_with_vars_in_expr(fun_actual, context)),
            args.iter()
                .map(|arg| replace_auto_with_vars_in_expr(arg, context))
                .collect(),
        ),
        Expr::NatRec(n, z, s) => Expr::NatRec(
            Box::new(replace_auto_with_vars_in_expr(n, context)),
            Box::new(replace_auto_with_vars_in_expr(z, context)),
            Box::new(replace_auto_with_vars_in_expr(s, context)),
        ),
        Expr::Tuple(fields) => Expr::Tuple(
            fields
                .iter()
                .map(|field| replace_auto_with_vars_in_expr(field, context))
                .collect(),
        ),
        Expr::DotTuple(expr, index) => Expr::DotTuple(
            Box::new(replace_auto_with_vars_in_expr(expr, context)),
            *index,
        ),
        Expr::Record(fields) => Expr::Record(
            fields
                .iter()
                .map(|field| Binding {
                    name: field.name.clone(),
                    expr: replace_auto_with_vars_in_expr(&field.expr, context),
                })
                .collect(),
        ),
        Expr::DotRecord(expr, label) => Expr::DotRecord(
            Box::new(replace_auto_with_vars_in_expr(expr, context)),
            label.clone(),
        ),
        Expr::Inl(expr) => Expr::Inl(Box::new(replace_auto_with_vars_in_expr(expr, context))),
        Expr::Inr(expr) => Expr::Inr(Box::new(replace_auto_with_vars_in_expr(expr, context))),
        Expr::Variant(label, expr) => Expr::Variant(
            label.clone(),
            expr.as_ref()
                .map(|expr| Box::new(replace_auto_with_vars_in_expr(expr, context))),
        ),
        Expr::List(exprs) => Expr::List(
            exprs
                .iter()
                .map(|expr| replace_auto_with_vars_in_expr(expr, context))
                .collect(),
        ),
        Expr::Cons(head, tail) => Expr::Cons(
            Box::new(replace_auto_with_vars_in_expr(head, context)),
            Box::new(replace_auto_with_vars_in_expr(tail, context)),
        ),
        Expr::ListHead(expr) => {
            Expr::ListHead(Box::new(replace_auto_with_vars_in_expr(expr, context)))
        }
        Expr::ListTail(expr) => {
            Expr::ListTail(Box::new(replace_auto_with_vars_in_expr(expr, context)))
        }
        Expr::ListIsEmpty(expr) => {
            Expr::ListIsEmpty(Box::new(replace_auto_with_vars_in_expr(expr, context)))
        }
        Expr::Let(bindings, expr) => {
            Expr::Let(
                bindings
                    .iter()
                    .map(|binding| PatternBinding {
                        // todo: replace_auto_with_vars_in_pattern
                        pattern: binding.pattern.clone(),
                        rhs: replace_auto_with_vars_in_expr(&binding.rhs, context),
                    })
                    .collect(),
                Box::new(replace_auto_with_vars_in_expr(expr, context)),
            )
        }
        // replace_auto_with_vars_in_pattern(&binding.pattern, context)
        Expr::LetRec(bindings, expr) => Expr::LetRec(
            bindings
                .iter()
                .map(|binding| PatternBinding {
                    pattern: binding.pattern.clone(),
                    rhs: replace_auto_with_vars_in_expr(&binding.rhs, context),
                })
                .collect(),
            Box::new(replace_auto_with_vars_in_expr(expr, context)),
        ),
        Expr::Match(expr, cases) => Expr::Match(
            Box::new(replace_auto_with_vars_in_expr(expr, context)),
            cases
                .iter()
                .map(|MatchCase { pattern, expr }| MatchCase {
                    pattern: pattern.clone(),
                    expr: replace_auto_with_vars_in_expr(expr, context),
                })
                .collect(),
        ),
        Expr::TypeAscription(expr, type_) => Expr::TypeAscription(
            Box::new(replace_auto_with_vars_in_expr(expr, context)),
            replace_auto_with_vars_in_type(type_, context),
        ),
        Expr::Fix(expr) => Expr::Fix(Box::new(replace_auto_with_vars_in_expr(expr, context))),
        Expr::Throw(expr) => Expr::Throw(Box::new(replace_auto_with_vars_in_expr(expr, context))),
        Expr::TryWith(try_, catch) => Expr::TryWith(
            Box::new(replace_auto_with_vars_in_expr(try_, context)),
            Box::new(replace_auto_with_vars_in_expr(catch, context)),
        ),
        Expr::TryCatch(try_, pattern, catch) => Expr::TryCatch(
            Box::new(replace_auto_with_vars_in_expr(try_, context)),
            pattern.clone(),
            Box::new(replace_auto_with_vars_in_expr(catch, context)),
        ),
        Expr::TypeCast(expr, type_) => Expr::TypeCast(
            Box::new(replace_auto_with_vars_in_expr(expr, context)),
            replace_auto_with_vars_in_type(type_, context),
        ),
        Expr::TryCastAs {
            try_,
            to,
            casted_pattern,
            casted_arm,
            fallback_arm,
        } => {
            Expr::TryCastAs {
                try_: Box::new(replace_auto_with_vars_in_expr(try_, context)),
                to: replace_auto_with_vars_in_type(to, context),
                casted_pattern: casted_pattern.clone(),
                casted_arm: Box::new(replace_auto_with_vars_in_expr(casted_arm, context)),
                fallback_arm: Box::new(replace_auto_with_vars_in_expr(fallback_arm, context)),
            };
            todo!()
        }
        Expr::Reference(expr) => {
            Expr::Reference(Box::new(replace_auto_with_vars_in_expr(expr, context)))
        }
        Expr::Dereference(expr) => {
            Expr::Dereference(Box::new(replace_auto_with_vars_in_expr(expr, context)))
        }
        Expr::Assignment(ref_, value) => Expr::Assignment(
            Box::new(replace_auto_with_vars_in_expr(ref_, context)),
            Box::new(replace_auto_with_vars_in_expr(value, context)),
        ),
        _ => unreachable!("{:?}", expr),
    }
}

// fn replace_auto_with_vars_in_pattern(pattern: &Pattern, context: &Context) -> Pattern {
//     match pattern {
//         Pattern::Unit | Pattern::False | Pattern::True | Pattern::Int(_) => pattern.clone(),
//         Pattern::Succ(pattern) => replace_auto_with_vars_in_pattern(pattern, context),
//         Pattern::Var(_) => todo!(),
//         Pattern::Inl(_) => todo!(),
//         Pattern::Inr(_) => todo!(),
//         Pattern::Tuple(_) => todo!(),
//         Pattern::Record(_) => todo!(),
//         Pattern::List(_) => todo!(),
//         Pattern::Cons(_, _) => todo!(),
//         Pattern::Variant(_,_) => todo!(),
//         Pattern::Ascription(_, _) | Pattern::CastAs(_, _) => unreachable!("{:?}", pattern),
//     }
// }

fn replace_auto_with_vars_in_type(type_: &Type, context: &Context) -> Type {
    match type_ {
        Type::Unit | Type::Bool | Type::Nat | Type::Var(_) => type_.clone(),
        Type::Auto => Type::TypeVar(context.constraints.borrow_mut().new_var()),
        Type::Fun(params, return_) => Type::Fun(
            params
                .iter()
                .map(|param| replace_auto_with_vars_in_type(param, context))
                .collect(),
            Box::new(replace_auto_with_vars_in_type(return_, context)),
        ),
        Type::Record(fields) => Type::Record(
            fields
                .iter()
                .map(|field| RecordFieldType {
                    label: field.label.clone(),
                    type_: replace_auto_with_vars_in_type(&field.type_, context),
                })
                .collect(),
        ),
        Type::Tuple(fields) => Type::Tuple(
            fields
                .iter()
                .map(|field| replace_auto_with_vars_in_type(field, context))
                .collect(),
        ),
        Type::Variant(fields) => Type::Variant(
            fields
                .iter()
                .map(|field| VariantFieldType {
                    label: field.label.clone(),
                    type_: field
                        .type_
                        .as_ref()
                        .map(|type_| replace_auto_with_vars_in_type(type_, context)),
                })
                .collect(),
        ),
        Type::Sum(left, right) => Type::Sum(
            Box::new(replace_auto_with_vars_in_type(left, context)),
            Box::new(replace_auto_with_vars_in_type(right, context)),
        ),
        Type::List(inner) => Type::List(Box::new(replace_auto_with_vars_in_type(inner, context))),
        Type::Ref(inner) => Type::Ref(Box::new(replace_auto_with_vars_in_type(inner, context))),
        Type::ForAll(_, _) => todo!(),
        Type::Rec(_, _) | Type::Top | Type::Bottom | Type::TypeVar(_) => {
            unreachable!("{:?}", type_)
        }
    }
}

fn check_infinite_type(id: &TypeVarID, type_: &Type, context: &Context) -> Result<(), TypeError> {
    match type_ {
        Type::Bool | Type::Nat | Type::Unit => Ok(()),
        Type::Var(name) => {
            let type_ = context.get(name)?;
            check_infinite_type(id, type_, context)
        }
        Type::Fun(args, return_) => {
            args.iter()
                .try_for_each(|arg| check_infinite_type(id, arg, context))?;
            check_infinite_type(id, return_, context)
        }
        Type::Tuple(fields) => fields
            .iter()
            .try_for_each(|field| check_infinite_type(id, field, context)),
        Type::Record(fields) => fields
            .iter()
            .try_for_each(|field| check_infinite_type(id, &field.type_, context)),
        Type::List(inner) => check_infinite_type(id, inner, context),
        Type::Sum(left, right) => {
            check_infinite_type(id, left, context)?;
            check_infinite_type(id, right, context)
        }
        Type::Variant(fields) => fields.iter().try_for_each(|field| {
            if let Some(type_) = &field.type_ {
                check_infinite_type(id, type_, context)
            } else {
                Ok(())
            }
        }),
        Type::TypeVar(id_) => {
            if id == id_ {
                Err(TypeError::InfiniteType(*id, type_.clone()))
            } else {
                Ok(())
            }
        }
        Type::Ref(inner) => check_infinite_type(id, inner, context),
        Type::ForAll(_, _) | Type::Rec(_, _) | Type::Top | Type::Bottom | Type::Auto => {
            unimplemented!("{:?} {:?}", id, type_)
        }
    }
}

fn unify(actual: &Type, expected: &Type, context: &Context) -> Result<(), TypeError> {
    let actual = context.constraints.borrow().substitute(actual);
    let expected = context.constraints.borrow().substitute(expected);

    match (&expected, &actual) {
        (Type::TypeVar(id), Type::TypeVar(id_)) if id == id_ => Ok(()),
        (Type::TypeVar(id), type_) | (type_, Type::TypeVar(id)) => {
            dbg!(&actual, &expected);
            check_infinite_type(id, type_, context)?;
            context.constraints.borrow_mut().update(*id, type_);
            Ok(())
        }
        _ if actual == expected => Ok(()),
        (
            Type::Fun(expected_args, expected_return_type),
            Type::Fun(actual_args, actual_return_type),
        ) => {
            if actual_args.len() == expected_args.len() {
                actual_args.iter().zip(expected_args.iter()).try_for_each(
                    |(actual_arg, expected_arg)| unify(actual_arg, expected_arg, context),
                )?;
                unify(actual_return_type, expected_return_type, context)
            } else {
                Err(TypeError::IncorrectNumberOfArguments {
                    expected: expected_args.len(),
                    actual: actual_args.len(),
                })
            }
        }
        (Type::Tuple(expected_fields), Type::Tuple(actual_fields)) => {
            if actual_fields.len() != expected_fields.len() {
                return Err(TypeError::UnexpectedTupleLength(
                    expected_fields.len(),
                    actual_fields.len(),
                ));
            }
            actual_fields
                .iter()
                .zip(expected_fields.iter())
                .try_for_each(|(actual_field, expected_field)| {
                    unify(actual_field, expected_field, context)
                })
        }
        (Type::Record(expected_fields), Type::Record(actual_fields)) => {
            expected_fields.iter().try_for_each(|expected| {
                let actual = actual_fields
                    .iter()
                    .find(|actual| actual.label == expected.label)
                    .ok_or(if actual_fields.len() == 1 {
                        TypeError::UnexpectedFieldAccess
                    } else {
                        TypeError::MissingRecordFields
                    })?;
                unify(&actual.type_, &expected.type_, context)
            })
        }
        (Type::List(expected_inner), Type::List(actual_inner)) => {
            unify(actual_inner, expected_inner, context)
        }
        (Type::Sum(expected_left, expected_right), Type::Sum(actual_left, actual_right)) => {
            unify(actual_left, expected_left, context)?;
            unify(actual_right, expected_right, context)
        }
        // (Type::Variant(expected_fields), Type::Variant(actual_fields)) => {
        //     expected_fields.iter().try_for_each(|expected| {
        //         let actual = actual_fields
        //             .iter()
        //             .find(|actual| actual.label == expected.label)
        //             .ok_or(TypeError::UnexpectedVariantLabel)?;
        //         if let (Some(expected), Some(actual)) = (&expected.type_, &actual.type_) {
        //             unify(expected, actual, context)
        //         } else {
        //             Ok(())
        //         }
        //     })
        // }
        (Type::Ref(expected_inner), Type::Ref(actual_inner)) => {
            unify(actual_inner, expected_inner, context)
        }
        _ => {
            dbg!(&actual, &expected);

            dbg!(&context.constraints);
            Err(TypeError::UnexpectedTypeForExpression {
                expected: expected,
                actual: Some(actual),
            })
        }
    }
}

pub fn collect_constraints(
    actual: &Expr,
    expected: &Type,
    context: &Context,
) -> Result<(), TypeError> {
    match actual {
        Expr::ConstUnit => unify(&Type::Unit, expected, context),
        Expr::ConstFalse | Expr::ConstTrue => unify(&Type::Bool, expected, context),
        Expr::ConstInt(_) => unify(&Type::Nat, expected, context),
        Expr::Succ(expr) => {
            unify(&Type::Nat, expected, context)?;
            collect_constraints(expr, &Type::Nat, context)
        }
        Expr::NatIsZero(expr) => {
            unify(&Type::Bool, expected, context)?;
            collect_constraints(expr, &Type::Nat, context)
        }
        Expr::Var(name) => {
            let type_ = context.get(name)?;
            unify(type_, expected, context)
        }
        Expr::Sequence(preceding, following) => {
            collect_constraints(preceding, &Type::Unit, context)?;
            collect_constraints(following, expected, context)
        }
        Expr::If(condition, then_, else_) => {
            collect_constraints(condition, &Type::Bool, context)?;
            collect_constraints(then_, expected, context)?;
            collect_constraints(else_, expected, context)
        }
        Expr::Abstraction(params, return_expr) => {
            let local = context.with_params(params);
            let return_type = Type::TypeVar(local.constraints.borrow_mut().new_var());
            let fun_type = Type::Fun(
                params
                    .iter()
                    .map(|ParamDecl { name: _, type_ }| type_.clone())
                    .collect(),
                Box::new(return_type.clone()),
            );
            collect_constraints(return_expr, &return_type, &local)?;
            unify(&fun_type, expected, context)
        }
        Expr::Application(fun_actual, args) => {
            let args_with_typevars: Vec<_> = args
                .iter()
                .map(|arg| {
                    (
                        arg,
                        Type::TypeVar(context.constraints.borrow_mut().new_var()),
                    )
                })
                .collect();

            args_with_typevars
                .iter()
                .try_for_each(|(arg, type_)| collect_constraints(arg, &type_, context))?;

            let fun_expected = Type::Fun(
                args_with_typevars
                    .iter()
                    .map(|(_, type_)| type_.clone())
                    .collect(),
                Box::new(expected.clone()),
            );
            collect_constraints(fun_actual, &fun_expected, context)
            // unify(fun_expected, some_new_type_var, context, from=fun_application)
        }
        Expr::NatRec(n, z, s) => {
            let fun = Type::Fun(
                vec![Type::Nat],
                Box::new(Type::Fun(
                    vec![expected.clone()],
                    Box::new(expected.clone()),
                )),
            );
            collect_constraints(n, &Type::Nat, context)?;
            collect_constraints(z, expected, context)?;
            collect_constraints(s, &fun, context)
        }
        Expr::Tuple(field_exprs) => {
            let field_types: Vec<Type> = field_exprs
                .iter()
                .map(|_| Type::TypeVar(context.constraints.borrow_mut().new_var()))
                .collect();

            let tuple = Type::Tuple(field_types.clone());
            unify(&tuple, expected, context)?;
            field_exprs
                .iter()
                .zip(field_types)
                .try_for_each(|(expr, type_)| collect_constraints(expr, &type_, context))
        }
        Expr::DotTuple(expr, index) => {
            let a = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let b = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let fields = vec![a, b];
            let field = fields.get(index - 1);
            match field {
                Some(field) => unify(field, expected, context),
                None => Err(TypeError::TupleIndexOutOfBounds(2, *index)),
            }?;
            collect_constraints(expr, &Type::Tuple(fields), context)
        }
        Expr::Record(fields) => {
            let field_types: Vec<Type> = fields
                .iter()
                .map(|_| Type::TypeVar(context.constraints.borrow_mut().new_var()))
                .collect();

            let record = Type::Record(
                fields
                    .iter()
                    .zip(&field_types)
                    .map(|(field, type_)| RecordFieldType {
                        label: field.name.clone(),
                        type_: type_.clone(),
                    })
                    .collect(),
            );
            unify(&record, expected, context)?;
            fields
                .iter()
                .zip(field_types)
                .try_for_each(|(binding, type_)| {
                    collect_constraints(&binding.expr, &type_, context)
                })
        }
        Expr::DotRecord(expr, label) => {
            let type_ = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let fields = vec![RecordFieldType {
                label: label.clone(),
                type_: type_.clone(),
            }];
            unify(&type_, expected, context)?;
            collect_constraints(expr, &Type::Record(fields), context)
        }
        Expr::List(exprs) => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let list = Type::List(Box::new(inner.clone()));
            unify(&list, expected, context)?;
            exprs
                .iter()
                .try_for_each(|expr| collect_constraints(expr, &inner, context))
        }
        Expr::Cons(head, tail) => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let list = Type::List(Box::new(inner.clone()));
            unify(&list, expected, context)?;
            collect_constraints(head, &inner, context)?;
            collect_constraints(tail, &list, context)
        }
        Expr::ListHead(expr) => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let list = Type::List(Box::new(inner.clone()));
            unify(&inner, expected, context)?;
            collect_constraints(expr, &list, context)
        }
        Expr::ListTail(expr) => collect_constraints(expr, expected, context),
        Expr::ListIsEmpty(expr) => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let list = Type::List(Box::new(inner.clone()));
            unify(&Type::Bool, expected, context)?;
            collect_constraints(expr, &list, context)
        }
        Expr::Let(bindings, expr) => {
            let local = bindings.iter().try_fold(
                context.clone(),
                |context, PatternBinding { pattern, rhs }| {
                    let local = collect_constraints_for_pattern(pattern, expected, &context)?;
                    collect_constraints(rhs, expected, &local)?;
                    Ok(local.clone())
                },
            )?;
            collect_constraints(expr, expected, &local)
        }
        Expr::Match(_, cases) if cases.is_empty() => Err(TypeError::IllegalEmptyMatching),
        Expr::Match(matched_expr, cases) => {
            let matched_type = Type::TypeVar(context.constraints.borrow_mut().new_var());
            collect_constraints(matched_expr, &matched_type, context)?;
            cases.iter().try_for_each(|MatchCase { pattern, expr }| {
                let local = collect_constraints_for_pattern(pattern, &matched_type, context)?;
                collect_constraints(expr, expected, &local)
            })
        }
        Expr::Inl(expr) => {
            let left = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let right = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let sum = Type::Sum(Box::new(left.clone()), Box::new(right.clone()));
            unify(&sum, expected, context)?;
            collect_constraints(expr, &left, context)
        }
        Expr::Inr(expr) => {
            let left = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let right = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let sum = Type::Sum(Box::new(left.clone()), Box::new(right.clone()));
            unify(&sum, expected, context)?;
            collect_constraints(expr, &right, context)
        }
        Expr::Reference(expr) => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let ref_ = Type::Ref(Box::new(inner.clone()));
            unify(&ref_, expected, context)?;
            collect_constraints(expr, &inner, context)
        }
        Expr::Dereference(expr) => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let ref_ = Type::Ref(Box::new(inner.clone()));
            unify(&inner, expected, context)?;
            collect_constraints(expr, &ref_, context)
        }
        Expr::Assignment(ref_, value) => {
            let value_type = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let ref_type = Type::Ref(Box::new(value_type.clone()));
            unify(&Type::Unit, expected, context)?;
            collect_constraints(ref_, &ref_type, context)?;
            collect_constraints(value, &value_type, context)
        }
        Expr::ConstMemory(_) => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let ref_ = Type::Ref(Box::new(inner.clone()));
            unify(&ref_, expected, context)
        }
        Expr::Throw(expr) => {
            let exception = get_exception_type(context)?;
            let return_type = Type::TypeVar(context.constraints.borrow_mut().new_var());
            unify(&return_type, expected, context)?;
            collect_constraints(expr, &exception, context)
        }
        Expr::TryWith(try_, catch) => {
            collect_constraints(try_, expected, context)?;
            collect_constraints(catch, expected, context)
        }
        Expr::TryCatch(try_, pattern, catch) => {
            collect_constraints(try_, expected, context)?;
            let exception = get_exception_type(context)?;
            let local = collect_constraints_for_pattern(pattern, &exception, context)?;
            collect_constraints(catch, expected, &local)
        }
        Expr::Panic => {
            let inner = Type::TypeVar(context.constraints.borrow_mut().new_var());
            unify(&inner, expected, context)
        }
        _ => unimplemented!("{:?}", actual),
    }
}

fn collect_constraints_for_pattern<'a>(
    pattern: &Pattern,
    matched_type: &Type,
    context: &Context,
) -> Result<Context, TypeError> {
    dbg!(pattern, matched_type);
    match pattern {
        Pattern::False | Pattern::True => {
            unify(&Type::Bool, matched_type, context)?;
            Ok(context.clone())
        }
        Pattern::Unit => {
            unify(&Type::Unit, matched_type, context)?;
            Ok(context.clone())
        }
        Pattern::Int(_) => {
            unify(&Type::Nat, matched_type, context)?;
            Ok(context.clone())
        }
        Pattern::Succ(inner) => {
            unify(&Type::Nat, matched_type, context)?;
            collect_constraints_for_pattern(inner, &Type::Nat, context)
        }
        Pattern::Var(name) => {
            let local = context.with(name, &matched_type);
            Ok(local)
        }
        Pattern::Tuple(_) => todo!(),
        Pattern::Record(_) => todo!(),
        Pattern::Inl(inner) => {
            let left = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let right = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let sum = Type::Sum(Box::new(left.clone()), Box::new(right.clone()));
            unify(&sum, matched_type, context)?;
            collect_constraints_for_pattern(inner, &left, context)
        }
        Pattern::Inr(inner) => {
            let left = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let right = Type::TypeVar(context.constraints.borrow_mut().new_var());
            let sum = Type::Sum(Box::new(left.clone()), Box::new(right.clone()));
            unify(&sum, matched_type, context)?;
            collect_constraints_for_pattern(inner, &right, context)
        }
        Pattern::Variant(_, _) => todo!(),
        Pattern::List(_) => todo!(),
        Pattern::Cons(_, _) => todo!(),
        Pattern::Ascription(_, _) | Pattern::CastAs(_, _) => unimplemented!("{:?}", pattern),
    }
}
