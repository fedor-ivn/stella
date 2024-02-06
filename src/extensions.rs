use thiserror::Error;

use crate::ast::*;

#[derive(Debug, Error)]
pub enum ExtensionError {
    #[error("Natural literals not enabled. Consider adding `extend with #natural-literals`.")]
    NaturalLiteralsNotEnabled,
    #[error("Nullary functions not enabled. Consider adding `extend with #nullary-functions`.")]
    NullaryFunctionsNotEnabled,
    #[error("Multi-parameter functions not enabled. Consider adding `extend with #multiparameter-functions`.")]
    MultiParameterFunctionsNotEnabled,
    #[error("Nested function declarations not enabled. Consider adding `extend with #nested-function-declarations`.")]
    NestedFunctionDeclarationsNotEnabled,
    #[error("Unit type not enabled. Consider adding `extend with #unit-type`.")]
    UnitTypeNotEnabled,
    #[error("Pairs not enabled. Consider adding `extend with #pairs`.")]
    PairsNotEnabled,
    #[error("Unsupported extension: {0}")]
    UnsupportedExtension(String),
}

#[derive(Default)]
struct Extensions {
    natural_literals: bool,
    nullary_functions: bool,
    multi_parameter_functions: bool,
    nested_function_declarations: bool,
    unit_type: bool,
    pairs: bool,
}

fn parse_extensions(program: &Program) -> Result<Extensions, ExtensionError> {
    program
        .extensions
        .iter()
        .flat_map(|ext| ext.extension_names.as_slice())
        .try_fold(Extensions::default(), |mut extensions, name| {
            match name.as_str() {
                "#natural-literals" => {
                    extensions.natural_literals = true;
                }
                "#nullary-functions" => {
                    extensions.nullary_functions = true;
                }
                "#multiparameter-functions" => {
                    extensions.multi_parameter_functions = true;
                }
                "#nested-function-declarations" => {
                    extensions.nested_function_declarations = true;
                }
                "#unit-type" => {
                    extensions.unit_type = true;
                }
                "#pairs" => {
                    extensions.pairs = true;
                }
                name => return Err(ExtensionError::UnsupportedExtension(name.to_owned())),
            };
            Ok(extensions)
        })
}

fn check_expr(expr: &Expr, extensions: &Extensions) -> Result<(), ExtensionError> {
    match expr {
        Expr::ConstTrue | Expr::ConstFalse => Ok(()),
        Expr::ConstInt(0) => Ok(()),
        Expr::ConstInt(_) => {
            if extensions.natural_literals {
                Ok(())
            } else {
                Err(ExtensionError::NaturalLiteralsNotEnabled)
            }
        }
        Expr::ConstUnit => {
            if extensions.unit_type {
                Ok(())
            } else {
                Err(ExtensionError::UnitTypeNotEnabled)
            }
        }
        Expr::Sequence(expr, None) => check_expr(expr, extensions),
        Expr::Sequence(_, _) => {
            todo!("Oops... `Sequence` with a `Some` is not yet supported.")
        }
        Expr::Var(_) => Ok(()),
        Expr::Succ(expr) => check_expr(expr, extensions),
        Expr::NatIsZero(expr) => check_expr(expr, extensions),
        Expr::NatRec(n, z, s) => {
            check_expr(n, extensions)?;
            check_expr(z, extensions)?;
            check_expr(s, extensions)
        }
        Expr::Application(fun, args) => {
            check_expr(fun, extensions)?;
            args.iter().try_for_each(|arg| check_expr(arg, extensions))
        }
        Expr::Abstraction(params, return_) => {
            if params.is_empty() && !extensions.nullary_functions {
                return Err(ExtensionError::NullaryFunctionsNotEnabled);
            }
            if params.len() > 1 && !extensions.multi_parameter_functions {
                return Err(ExtensionError::MultiParameterFunctionsNotEnabled);
            }
            check_expr(return_, extensions)
        }
        Expr::If(cond, then, else_) => {
            check_expr(cond, extensions)?;
            check_expr(then, extensions)?;
            check_expr(else_, extensions)
        }
        Expr::Tuple(exprs) => {
            if extensions.pairs && exprs.len() <= 2 {
                exprs
                    .iter()
                    .try_for_each(|expr| check_expr(expr, extensions))
            } else {
                Err(ExtensionError::PairsNotEnabled)
            }
        }
        Expr::DotTuple(expr, index) => {
            if extensions.pairs && *index <= 2 {
                check_expr(expr, extensions)
            } else {
                Err(ExtensionError::PairsNotEnabled)
            }
        }
        expr => {
            dbg!(expr);
            todo!("Oops... This expression is not yet supported.")
        }
    }
}

fn check_decl(decl: Decl, extensions: &Extensions) -> Result<(), ExtensionError> {
    match decl {
        Decl::DeclFun { local_decls, .. }
            if !local_decls.is_empty() && !extensions.nested_function_declarations =>
        {
            Err(ExtensionError::NestedFunctionDeclarationsNotEnabled)
        }
        Decl::DeclFun {
            param_decls,
            return_expr,
            ..
        } => check_expr(
            &Expr::Abstraction(param_decls, Box::new(return_expr)),
            extensions,
        ),
        _ => Ok(()),
    }
}

pub fn check_program(program: &Program) -> Result<(), ExtensionError> {
    let extensions = parse_extensions(program)?;
    program
        .decls
        .iter()
        .try_for_each(|decl| check_decl(decl.clone(), &extensions))
}
