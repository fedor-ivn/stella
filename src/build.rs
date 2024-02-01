use std::rc::Rc;

use antlr_rust::parser_rule_context::BaseParserRuleContext;

use crate::{
    ast::*,
    stellaparser::{
        DeclContextAll, ExprContextAll, ExtensionContextAll, ParamDeclContextExt,
        ProgramContextExt, StellatypeContextAll,
    },
};

fn build_expr_box(expr: &Option<Rc<ExprContextAll>>) -> Box<Expr> {
    Box::new(build_expr(expr.as_ref().unwrap()))
}

fn token_name<'a>(
    token: &'a Option<<antlr_rust::token_factory::CommonTokenFactory as antlr_rust::token_factory::TokenFactory>::Tok>,
) -> std::borrow::Cow<'a, str> {
    std::borrow::Cow::Borrowed(&token.as_ref().unwrap().text)
}

fn build_param_decl<'input>(
    ctx: &BaseParserRuleContext<'input, ParamDeclContextExt<'input>>,
) -> ParamDecl {
    return ParamDecl {
        name: token_name(&ctx.name).to_string(),
        type_: build_type(ctx.paramType.as_ref().unwrap()),
    };
}

pub fn build_expr(ctx: &ExprContextAll) -> Expr {
    match ctx {
        // TODO: return error to the caller
        ExprContextAll::Error(_) => todo!(),

        ExprContextAll::DotRecordContext(ctx) => Expr::DotRecord(
            build_expr_box(&ctx.expr_),
            token_name(&ctx.label).into_owned(),
        ),
        ExprContextAll::DotTupleContext(ctx) => Expr::DotTuple(
            build_expr_box(&ctx.expr_),
            token_name(&ctx.index).parse().unwrap(),
        ),

        ExprContextAll::ConstTrueContext(_) => Expr::ConstTrue,
        ExprContextAll::ConstFalseContext(_) => Expr::ConstFalse,
        ExprContextAll::ConstUnitContext(_) => Expr::ConstUnit,

        ExprContextAll::ConstIntContext(ctx) => Expr::ConstInt(token_name(&ctx.n).parse().unwrap()),
        ExprContextAll::ConstMemoryContext(_) => todo!(),
        ExprContextAll::VarContext(ctx) => Expr::Var(token_name(&ctx.name).into_owned()),

        ExprContextAll::PanicContext(_) => Expr::Panic,
        ExprContextAll::ThrowContext(ctx) => Expr::Throw(build_expr_box(&ctx.expr_)),
        ExprContextAll::TryCatchContext(_) => todo!(),
        ExprContextAll::TryWithContext(_) => todo!(),

        ExprContextAll::InlContext(ctx) => Expr::Inl(build_expr_box(&ctx.expr_)),
        ExprContextAll::InrContext(ctx) => Expr::Inr(build_expr_box(&ctx.expr_)),

        ExprContextAll::ConsListContext(ctx) => {
            Expr::Cons(build_expr_box(&ctx.head), build_expr_box(&ctx.tail))
        }
        ExprContextAll::HeadContext(ctx) => Expr::ListHead(build_expr_box(&ctx.list)),
        ExprContextAll::IsEmptyContext(ctx) => Expr::ListIsEmpty(build_expr_box(&ctx.list)),
        ExprContextAll::TailContext(ctx) => Expr::ListTail(build_expr_box(&ctx.list)),
        ExprContextAll::SuccContext(ctx) => Expr::Succ(build_expr_box(&ctx.n)),
        ExprContextAll::LogicNotContext(ctx) => Expr::LogicalNot(build_expr_box(&ctx.expr_)),
        ExprContextAll::PredContext(ctx) => Expr::NatPred(build_expr_box(&ctx.n)),
        ExprContextAll::IsZeroContext(ctx) => Expr::NatIsZero(build_expr_box(&ctx.n)),
        ExprContextAll::FixContext(ctx) => Expr::Fix(build_expr_box(&ctx.expr_)),
        ExprContextAll::NatRecContext(ctx) => Expr::NatRec(
            build_expr_box(&ctx.n),
            build_expr_box(&ctx.initial),
            build_expr_box(&ctx.step),
        ),

        ExprContextAll::FoldContext(ctx) => Expr::Fold(
            build_type(ctx.type_.as_ref().unwrap()),
            build_expr_box(&ctx.expr_),
        ),
        ExprContextAll::UnfoldContext(ctx) => Expr::Unfold(
            build_type(ctx.type_.as_ref().unwrap()),
            build_expr_box(&ctx.expr_),
        ),

        ExprContextAll::ApplicationContext(ctx) => Expr::Application(
            build_expr_box(&ctx.fun),
            ctx.args.iter().map(|expr| build_expr(expr)).collect(),
        ),
        ExprContextAll::TypeApplicationContext(_) => todo!(),

        ExprContextAll::MultiplyContext(ctx) => {
            Expr::Multiply(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::DivideContext(ctx) => {
            Expr::Divide(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::LogicAndContext(ctx) => {
            Expr::LogicalAnd(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::RefContext(ctx) => Expr::Reference(build_expr_box(&ctx.expr_)),
        ExprContextAll::DerefContext(ctx) => Expr::Dereference(build_expr_box(&ctx.expr_)),

        ExprContextAll::AddContext(ctx) => {
            Expr::Add(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::SubtractContext(ctx) => {
            Expr::Subtract(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::LogicOrContext(ctx) => {
            Expr::LogicalOr(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::TypeAscContext(ctx) => Expr::TypeAscription(
            build_expr_box(&ctx.expr_),
            build_type(&ctx.type_.as_ref().unwrap()),
        ),
        ExprContextAll::TypeCastContext(ctx) => Expr::TypeCast(
            build_expr_box(&ctx.expr_),
            build_type(&ctx.type_.as_ref().unwrap()),
        ),

        ExprContextAll::AbstractionContext(ctx) => Expr::Abstraction(
            ctx.paramDecls
                .iter()
                .map(|param_decl| build_param_decl(param_decl))
                .collect(),
            build_expr_box(&ctx.returnExpr),
        ),

        ExprContextAll::TupleContext(ctx) => {
            Expr::Tuple(ctx.exprs.iter().map(|x| build_expr(x)).collect())
        }
        ExprContextAll::RecordContext(_) => todo!(),
        ExprContextAll::VariantContext(_) => todo!(),
        ExprContextAll::MatchContext(_) => todo!(),
        ExprContextAll::ListContext(ctx) => {
            Expr::List(ctx.exprs.iter().map(|x| build_expr(x)).collect())
        }

        ExprContextAll::LessThanContext(ctx) => {
            Expr::LessThan(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::LessThanOrEqualContext(ctx) => {
            Expr::LessThanOrEqual(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::GreaterThanContext(ctx) => {
            Expr::GreaterThan(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::GreaterThanOrEqualContext(ctx) => {
            Expr::GreaterThanOrEqual(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::EqualContext(ctx) => {
            Expr::Equal(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }
        ExprContextAll::NotEqualContext(ctx) => {
            Expr::NotEqual(build_expr_box(&ctx.left), build_expr_box(&ctx.right))
        }

        ExprContextAll::AssignContext(ctx) => {
            Expr::Assignment(build_expr_box(&ctx.lhs), build_expr_box(&ctx.rhs))
        }
        ExprContextAll::IfContext(ctx) => Expr::If(
            build_expr_box(&ctx.condition),
            build_expr_box(&ctx.thenExpr),
            build_expr_box(&ctx.elseExpr),
        ),
        ExprContextAll::LetContext(_) => todo!(),
        ExprContextAll::LetRecContext(_) => todo!(),
        ExprContextAll::TypeAbstractionContext(_) => todo!(),
        ExprContextAll::ParenthesisedExprContext(ctx) => build_expr(ctx.expr_.as_ref().unwrap()),
        ExprContextAll::SequenceContext(ctx) => Expr::Sequence(
            build_expr_box(&ctx.expr1),
            ctx.expr2.as_ref().map(|x| Box::new(build_expr(&*x))),
        ),
    }
}

fn build_type_box(ctx: &Option<Rc<StellatypeContextAll<'_>>>) -> Box<Type> {
    Box::new(build_type(ctx.as_ref().unwrap()))
}

fn build_type(ctx: &StellatypeContextAll) -> Type {
    match ctx {
        StellatypeContextAll::Error(_) => todo!(),
        StellatypeContextAll::TypeBoolContext(_) => Type::Bool,
        StellatypeContextAll::TypeNatContext(_) => Type::Nat,
        StellatypeContextAll::TypeFunContext(ctx) => Type::Fun(
            ctx.paramTypes
                .iter()
                .map(|type_| build_type(type_))
                .collect(),
            Box::new(build_type(ctx.returnType.as_ref().unwrap())),
        ),
        StellatypeContextAll::TypeForAllContext(_) => todo!(),
        StellatypeContextAll::TypeRecContext(_) => todo!(),
        StellatypeContextAll::TypeSumContext(ctx) => {
            Type::Sum(build_type_box(&ctx.left), build_type_box(&ctx.right))
        }
        StellatypeContextAll::TypeTupleContext(ctx) => {
            Type::Tuple(ctx.types.iter().map(|x| build_type(x)).collect())
        }
        StellatypeContextAll::TypeRecordContext(_) => todo!(),
        StellatypeContextAll::TypeVariantContext(_) => todo!(),
        StellatypeContextAll::TypeListContext(ctx) => {
            Type::List(ctx.types.iter().map(|x| build_type(x)).collect())
        }
        StellatypeContextAll::TypeUnitContext(_) => Type::Unit,
        StellatypeContextAll::TypeTopContext(_) => Type::Top,
        StellatypeContextAll::TypeRefContext(ctx) => Type::Ref(build_type_box(&ctx.type_)),
        StellatypeContextAll::TypeBottomContext(_) => Type::Bottom,
        StellatypeContextAll::TypeVarContext(ctx) => Type::Var(token_name(&ctx.name).into_owned()),
        StellatypeContextAll::TypeParensContext(ctx) => build_type(ctx.type_.as_ref().unwrap()),
    }
}

fn build_decl(ctx: &DeclContextAll) -> Decl {
    match ctx {
        DeclContextAll::Error(_) => todo!(),

        DeclContextAll::DeclFunContext(ctx) => Decl::DeclFun {
            annotations: Vec::new(), // TODO: convert annotations
            name: token_name(&ctx.name).to_string(),
            param_decls: ctx
                .paramDecls
                .iter()
                .map(|param_decl| build_param_decl(param_decl))
                .collect(),
            return_type: ctx.returnType.as_ref().map(|type_| build_type(type_)),
            throws_types: ctx
                .throwTypes
                .iter()
                .map(|type_| build_type(type_))
                .collect(),
            local_decls: ctx.localDecls.iter().map(|decl| build_decl(decl)).collect(),
            return_expr: build_expr(ctx.returnExpr.as_ref().unwrap()),
        },
        DeclContextAll::DeclFunGenericContext(_) => todo!(),

        DeclContextAll::DeclTypeAliasContext(ctx) => Decl::DeclTypeAlias {
            name: token_name(&ctx.name).to_string(),
            type_: build_type(ctx.atype.as_ref().unwrap()),
        },
        DeclContextAll::DeclExceptionTypeContext(_) => todo!(),
        DeclContextAll::DeclExceptionVariantContext(_) => todo!(),
    }
}

fn build_extension(ctx: &ExtensionContextAll) -> Extension {
    match ctx {
        ExtensionContextAll::Error(_) => todo!(),
        ExtensionContextAll::AnExtensionContext(ctx) => Extension {
            extension_names: ctx
                .extensionNames
                .iter()
                .map(|x| (*x.text).to_owned())
                .collect(),
        },
    }
}

pub fn build_program<'input>(
    ctx: &BaseParserRuleContext<'input, ProgramContextExt<'input>>,
) -> Program {
    Program {
        language_decl: LanguageDecl::LanguageCore, // TODO: language decl
        extensions: ctx
            .extensions
            .iter()
            .map(|extension| build_extension(extension))
            .collect(),
        decls: ctx.decls.iter().map(|decl| build_decl(decl)).collect(),
    }
}
