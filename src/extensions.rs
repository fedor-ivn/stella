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

    #[error("Tuples not enabled. Consider adding `extend with #tuples`.")]
    TuplesNotEnabled,

    #[error("Records not enabled. Consider adding `extend with #records`.")]
    RecordsNotEnabled,

    #[error("Let bindings not enabled. Consider adding `extend with #let-bindings`.")]
    LetBindingsNotEnabled,

    #[error(
        "Structural patterns not enabled. Consider adding `extend with #structural-patterns`."
    )]
    StructuralPatternsNotEnabled,

    #[error("Type ascriptions not enabled. Consider adding `extend with #type-ascriptions`.")]
    TypeAscriptionsNotEnabled,

    #[error("Sum types not enabled. Consider adding `extend with #sum-types`.")]
    SumTypesNotEnabled,

    #[error("Unsupported extension: {0}")]
    UnsupportedExtension(String),

    #[error("Lists not enabled. Consider adding `extend with #lists`.")]
    ListsNotEnabled,

    #[error("Variants not enabled. Consider adding `extend with #variants`.")]
    VariantsNotEnabled,

    #[error(
        "Nullary variant labels not enabled. Consider adding `extend with #nullary-variant-labels`."
    )]
    NullaryVariantLabelsNotEnabled,

    #[error(
        "Fixpoint combinator not enabled. Consider adding `extend with #fixpoint-combinator`."
    )]
    FixpointCombinatorNotEnabled,

    #[error("Letrec bindings not enabled. Consider adding `extend with #letrec-bindings`.")]
    LetRecBindingsNotEnabled,

    #[error("Sequencing not enabled. Consider adding `extend with #sequencing`.")]
    SequencingNotEnabled,

    #[error("References not enabled. Consider adding `extend with #references`.")]
    ReferencesNotEnabled,

    #[error("Panic not enabled. Consider adding `extend with #panic`.")]
    PanicNotEnabled,

    #[error("Exceptions not enabled. Consider adding `extend with #exceptions`.")]
    ExceptionsNotEnabled,

    #[error("Type declarations for exceptions not enabled. Consider adding `extend with #exception-type-declaration`.")]
    ExceptionTypeDeclarationNotEnabled,

    #[error(
        "Open variant exceptions not enabled. Consider adding `extend with #open-variant-expressions`."
    )]
    OpenVariantExceptionsNotEnabled,

    // #[error(
    //     "Structural subtyping not enabled. Consider adding `extend with #structural-subtyping`."
    // )]
    // StructuralSubtypingNotEnabled,
    #[error("Structural subtyping not enabled. Consider adding `extend with #type-cast`.")]
    TypeCastNotEnabled,
    // #[error("Structural subtyping not enabled. Consider adding `extend with #top-type`.")]
    // TopTypeNotEnabled,
    // #[error("Structural subtyping not enabled. Consider adding `extend with #bottom-type`.")]
    // BottomTypeNotEnabled,
    #[error("Try cast as not enabled. Consider adding `extend with #try-cast-as`.")]
    TryCastAsNotEnabled,
    #[error("Type cast patterns not enabled. Consider adding `extend with #type-cast-patterns`.")]
    TypeCastPatternsNotEnabled,

    #[error(
        "Type reconstruction not enabled. Consider adding `extend with #type-reconstruction`."
    )]
    TypeReconstructionNotEnabled,
}

#[derive(Default, Clone)]
pub struct Extensions {
    natural_literals: bool,
    nullary_functions: bool,
    multi_parameter_functions: bool,
    nested_function_declarations: bool,
    unit_type: bool,
    pairs: bool,
    tuples: bool,
    records: bool,
    let_bindings: bool,
    let_patterns: bool,
    structural_patterns: bool,
    type_ascriptions: bool,
    sum_types: bool,
    lists: bool,
    variants: bool,
    nullary_variant_labels: bool,
    fixpoint_combinator: bool,
    letrec_bindings: bool,
    sequencing: bool,
    references: bool,
    panic: bool,
    exceptions: bool,
    exception_type_declaration: bool,
    open_variant_exceptions: bool,
    pub structural_subtyping: bool,
    pub ambiguous_type_as_bottom: bool,
    type_cast: bool,
    top_type: bool,
    bottom_type: bool,
    try_cast_as: bool,
    type_cast_patterns: bool,
    pub type_reconstruction: bool,
}

pub fn parse_extensions(program: &Program) -> Result<Extensions, ExtensionError> {
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
                "#tuples" => {
                    extensions.tuples = true;
                }
                "#records" => {
                    extensions.records = true;
                }
                "#let-bindings" => {
                    extensions.let_bindings = true;
                }
                "#let-patterns" => {
                    extensions.let_patterns = true;
                }
                "#structural-patterns" => {
                    extensions.structural_patterns = true;
                }
                "#type-ascriptions" => {
                    extensions.type_ascriptions = true;
                }
                "#sum-types" => {
                    extensions.sum_types = true;
                    extensions.structural_patterns = true;
                }
                "#lists" => {
                    extensions.lists = true;
                }
                "#general-recursion" => {
                    // todo: implement some checking for general recursion
                }
                "#variants" => {
                    extensions.variants = true;
                    extensions.structural_patterns = true;
                }
                "#nullary-variant-labels" => {
                    extensions.nullary_variant_labels = true;
                }
                "#fixpoint-combinator" => {
                    extensions.fixpoint_combinator = true;
                }
                "#letrec-bindings" | "#letrec-many-bindings" => {
                    extensions.letrec_bindings = true;
                }
                "#sequencing" => {
                    extensions.sequencing = true;
                }
                "#references" => {
                    extensions.references = true;
                }
                "#panic" => {
                    extensions.panic = true;
                }
                "#exceptions" => {
                    extensions.exceptions = true;
                }
                "#exception-type-declaration" => {
                    extensions.exception_type_declaration = true;
                }
                "#open-variant-exceptions" => {
                    extensions.open_variant_exceptions = true;
                }
                "#structural-subtyping" => {
                    extensions.structural_subtyping = true;
                }
                "#ambiguous-type-as-bottom" => {
                    extensions.ambiguous_type_as_bottom = true;
                }
                "#type-cast" => {
                    extensions.type_cast = true;
                }
                "#top-type" => {
                    extensions.top_type = true;
                }
                "#bottom-type" => {
                    extensions.bottom_type = true;
                }
                "#try-cast-as" => {
                    extensions.try_cast_as = true;
                }
                "#type-cast-patterns" => {
                    extensions.type_cast_patterns = true;
                    extensions.structural_patterns = true;
                }
                "#type-reconstruction" => {
                    extensions.type_reconstruction = true;
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
        Expr::Sequence(first, second) => {
            if !extensions.sequencing {
                return Err(ExtensionError::SequencingNotEnabled);
            }
            check_expr(first, extensions)?;
            check_expr(second, extensions)
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
            if extensions.pairs || extensions.tuples {
                exprs
                    .iter()
                    .try_for_each(|expr| check_expr(expr, extensions))
            } else if exprs.len() <= 2 {
                Err(ExtensionError::PairsNotEnabled)
            } else {
                Err(ExtensionError::TuplesNotEnabled)
            }
        }
        Expr::DotTuple(expr, index) => {
            if extensions.pairs || extensions.tuples {
                check_expr(expr, extensions)
            } else if *index <= 2 {
                Err(ExtensionError::PairsNotEnabled)
            } else {
                Err(ExtensionError::TuplesNotEnabled)
            }
        }
        Expr::Record(bindings) => {
            if !extensions.records {
                return Err(ExtensionError::RecordsNotEnabled);
            }
            bindings
                .iter()
                .try_for_each(|binding| check_expr(&binding.expr, extensions))
        }
        Expr::DotRecord(expr, _) => {
            if !extensions.records {
                return Err(ExtensionError::RecordsNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::Let(bindings, expr) => {
            if !extensions.let_bindings {
                return Err(ExtensionError::LetBindingsNotEnabled);
            }
            bindings.iter().try_for_each(|binding| {
                check_pattern(&binding.pattern, extensions)?;
                check_expr(&binding.rhs, extensions)
            })?;
            check_expr(expr, extensions)
        }
        Expr::Match(expr, cases) => {
            if !extensions.structural_patterns {
                return Err(ExtensionError::StructuralPatternsNotEnabled);
            }
            check_expr(expr, extensions)?;
            cases
                .iter()
                .try_for_each(|case| check_expr(&case.expr, extensions))
        }
        Expr::Inl(expr) | Expr::Inr(expr) => {
            if !extensions.sum_types {
                return Err(ExtensionError::SumTypesNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::TypeAscription(expr, _) => {
            if !extensions.type_ascriptions {
                return Err(ExtensionError::TypeAscriptionsNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::List(exprs) => {
            if !extensions.lists {
                return Err(ExtensionError::ListsNotEnabled);
            }
            exprs
                .iter()
                .try_for_each(|expr| check_expr(expr, extensions))
        }
        Expr::Variant(_, Some(expr)) => {
            if !extensions.variants {
                return Err(ExtensionError::VariantsNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::Variant(_, None) => {
            if !extensions.nullary_variant_labels {
                return Err(ExtensionError::NullaryVariantLabelsNotEnabled);
            }
            Ok(())
        }
        Expr::Cons(head, tail) => {
            if !extensions.lists {
                return Err(ExtensionError::ListsNotEnabled);
            }
            check_expr(head, extensions)?;
            check_expr(tail, extensions)
        }
        Expr::ListHead(expr) | Expr::ListIsEmpty(expr) | Expr::ListTail(expr) => {
            if !extensions.lists {
                return Err(ExtensionError::ListsNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::Fix(expr) => {
            if !extensions.fixpoint_combinator {
                return Err(ExtensionError::FixpointCombinatorNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::LetRec(bindings, expr) => {
            if !extensions.letrec_bindings {
                return Err(ExtensionError::LetRecBindingsNotEnabled);
            }
            bindings.iter().try_for_each(|binding| {
                check_pattern(&binding.pattern, extensions)?;
                check_expr(&binding.rhs, extensions)
            })?;
            check_expr(expr, extensions)
        }
        Expr::ConstMemory(_) => {
            if !extensions.references {
                return Err(ExtensionError::ReferencesNotEnabled);
            }
            Ok(())
        }
        Expr::Reference(expr) | Expr::Dereference(expr) => {
            if !extensions.references {
                return Err(ExtensionError::ReferencesNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::Assignment(lhs, rhs) => {
            if !extensions.references {
                return Err(ExtensionError::ReferencesNotEnabled);
            }
            check_expr(lhs, extensions)?;
            check_expr(rhs, extensions)
        }
        Expr::Panic => {
            if !extensions.panic {
                return Err(ExtensionError::PanicNotEnabled);
            }
            Ok(())
        }
        Expr::Throw(expr) => {
            if !extensions.exceptions {
                return Err(ExtensionError::ExceptionsNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::TryCatch(error_prone, pattern, handler) => {
            if !extensions.exceptions {
                return Err(ExtensionError::ExceptionsNotEnabled);
            }
            check_expr(error_prone, extensions)?;
            check_pattern(pattern, extensions)?;
            check_expr(handler, extensions)
        }
        Expr::TryWith(error_prone, fallback_value) => {
            if !extensions.exceptions {
                return Err(ExtensionError::ExceptionsNotEnabled);
            }
            check_expr(error_prone, extensions)?;
            check_expr(fallback_value, extensions)
        }
        Expr::TypeCast(expr, _) => {
            if !extensions.type_cast {
                return Err(ExtensionError::TypeCastNotEnabled);
            }
            check_expr(expr, extensions)
        }
        Expr::TryCastAs {
            try_,
            casted_pattern,
            casted_arm,
            fallback_arm,
            ..
        } => {
            if !extensions.try_cast_as {
                return Err(ExtensionError::TryCastAsNotEnabled);
            }
            check_expr(try_, extensions)?;
            check_pattern(casted_pattern, extensions)?;
            check_expr(casted_arm, extensions)?;
            check_expr(fallback_arm, extensions)
        }
        expr => {
            dbg!(expr);
            todo!("Oops... This expression is not yet supported.")
        }
    }
}

fn check_pattern(pattern: &Pattern, extensions: &Extensions) -> Result<(), ExtensionError> {
    match pattern {
        Pattern::True | Pattern::False => Ok(()),
        Pattern::Unit if extensions.unit_type => Ok(()),
        Pattern::Unit => Err(ExtensionError::UnitTypeNotEnabled),
        Pattern::Var(_) => Ok(()),
        Pattern::Int(0) => Ok(()),
        Pattern::Int(_) if extensions.natural_literals => Ok(()),
        Pattern::Int(_) => Err(ExtensionError::NaturalLiteralsNotEnabled),
        Pattern::Tuple(patterns) => {
            if extensions.pairs || extensions.tuples {
                patterns
                    .iter()
                    .try_for_each(|pattern| check_pattern(pattern, extensions))
            } else if patterns.len() <= 2 {
                Err(ExtensionError::PairsNotEnabled)
            } else {
                Err(ExtensionError::TuplesNotEnabled)
            }
        }
        Pattern::Record(bindings) => {
            if !extensions.records {
                return Err(ExtensionError::RecordsNotEnabled);
            }
            bindings
                .iter()
                .try_for_each(|binding| check_pattern(&binding.pattern, extensions))
        }
        Pattern::Inl(pattern) | Pattern::Inr(pattern) => {
            if !extensions.sum_types {
                return Err(ExtensionError::SumTypesNotEnabled);
            }
            check_pattern(pattern, extensions)
        }
        Pattern::CastAs(pattern, _) => {
            if !extensions.type_cast_patterns {
                return Err(ExtensionError::TypeCastPatternsNotEnabled);
            }
            check_pattern(pattern, extensions)
        }
        pattern => {
            dbg!(pattern);
            todo!("Oops... This pattern is not yet supported.")
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
        Decl::DeclExceptionType(_) if !extensions.exception_type_declaration => {
            Err(ExtensionError::ExceptionTypeDeclarationNotEnabled)
        }
        Decl::DeclExceptionVariant { .. } if !extensions.open_variant_exceptions => {
            Err(ExtensionError::OpenVariantExceptionsNotEnabled)
        }
        _ => Ok(()),
    }
}

pub fn check_extensions(program: &Program, extensions: &Extensions) -> Result<(), ExtensionError> {
    program
        .decls
        .iter()
        .try_for_each(|decl| check_decl(decl.clone(), &extensions))
}
