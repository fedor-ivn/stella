use std::rc::Rc;

use antlr_rust::parser_rule_context::BaseParserRuleContext;

use crate::{
    ast::*,
    stellaparser::{
        DeclContextAll, ExprContextAll, ParamDeclContextExt, ProgramContextExt,
        StellatypeContextAll,
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

fn build_expr(ctx: &ExprContextAll) -> Expr {
    match ctx {
        ExprContextAll::Error(_) => todo!(),

        ExprContextAll::ConstTrueContext(_) => Expr::ConstTrue,
        ExprContextAll::ConstFalseContext(_) => Expr::ConstFalse,
        ExprContextAll::IfContext(ctx) => Expr::If(
            build_expr_box(&ctx.condition),
            build_expr_box(&ctx.thenExpr),
            build_expr_box(&ctx.elseExpr),
        ),

        ExprContextAll::SuccContext(ctx) => Expr::Succ(build_expr_box(&ctx.n)),
        ExprContextAll::NatRecContext(ctx) => Expr::NatRec(
            build_expr_box(&ctx.n),
            build_expr_box(&ctx.initial),
            build_expr_box(&ctx.step),
        ),
        ExprContextAll::ConstIntContext(ctx) => Expr::ConstInt(token_name(&ctx.n).parse().unwrap()),
        ExprContextAll::VarContext(ctx) => Expr::Var(token_name(&ctx.name).to_string()),
    }
}

fn build_type(ctx: &StellatypeContextAll) -> Type {
    match ctx {
        StellatypeContextAll::TypeBoolContext(_) => Type::Bool,
        StellatypeContextAll::TypeFunContext(ctx) => Type::Fun(
            ctx.paramTypes
                .iter()
                .map(|type_| build_type(type_))
                .collect(),
            Box::new(build_type(ctx.returnType.as_ref().unwrap())),
        ),
        StellatypeContextAll::Error(_) => todo!(),
        StellatypeContextAll::TypeNatContext(_) => Type::Nat,
    }
}

fn build_decl(ctx: &DeclContextAll) -> Decl {
    match ctx {
        DeclContextAll::DeclTypeAliasContext(ctx) => Decl::DeclTypeAlias {
            name: token_name(&ctx.name).to_string(),
            type_: build_type(ctx.atype.as_ref().unwrap()),
        },
        DeclContextAll::DeclFunContext(ctx) => Decl::DeclFun {
            annotations: Vec::new(), // TODO: convert annotations
            name: token_name(&ctx.name).to_string(),
            param_decls: ctx
                .paramDecls
                .iter()
                .map(|param_decl| build_param_decl(param_decl))
                .collect(),
            return_type: ctx.returnType.as_ref().map(|type_| build_type(type_)),
            throws_type: ctx.throwType.as_ref().map(|type_| build_type(type_)),
            local_decls: ctx.localDecls.iter().map(|decl| build_decl(decl)).collect(),
            return_expr: build_expr(ctx.returnExpr.as_ref().unwrap()),
        },
        DeclContextAll::Error(_) => todo!(),
    }
}

pub fn build_program<'input>(
    ctx: &BaseParserRuleContext<'input, ProgramContextExt<'input>>,
) -> Program {
    Program {
        language_decl: LanguageDecl::LanguageCore, // TODO: language decl
        extensions: Vec::new(),                    // TODO: extensions
        decls: ctx.decls.iter().map(|decl| build_decl(decl)).collect(),
    }
}
