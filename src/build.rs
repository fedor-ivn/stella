use antlr_rust::parser_rule_context::BaseParserRuleContext;

use crate::{
    ast::*,
    stellaparser::{
        DeclContextAll, ExprContextAll, ParamDeclContextExt, ProgramContextExt,
        StellatypeContextAll,
    },
};

fn build_param_decl<'input>(
    ctx: &BaseParserRuleContext<'input, ParamDeclContextExt<'input>>,
) -> ParamDecl {
    return ParamDecl {
        name: ctx.name.as_ref().unwrap().to_string(),
        type_: build_type(ctx.paramType.as_ref().unwrap()),
    };
}

fn build_expr(ctx: &ExprContextAll) -> Expr {
    match ctx {
        ExprContextAll::Error(_) => todo!(),

        ExprContextAll::ConstTrueContext(_) => Expr::ConstTrue,
        ExprContextAll::ConstFalseContext(_) => Expr::ConstFalse,
        ExprContextAll::IfContext(ctx) => Expr::If(
            Box::new(build_expr(ctx.condition.as_ref().unwrap())),
            Box::new(build_expr(ctx.thenExpr.as_ref().unwrap())),
            Box::new(build_expr(ctx.elseExpr.as_ref().unwrap())),
        ),

        ExprContextAll::SuccContext(ctx) => {
            Expr::Succ(Box::new(build_expr(ctx.n.as_ref().unwrap())))
        }
        ExprContextAll::NatRecContext(ctx) => Expr::NatRec(
            Box::new(build_expr(ctx.n.as_ref().unwrap())),
            Box::new(build_expr(ctx.initial.as_ref().unwrap())),
            Box::new(build_expr(ctx.step.as_ref().unwrap())),
        ),
        ExprContextAll::ConstIntContext(ctx) => {
            Expr::ConstInt(ctx.n.as_ref().unwrap().text.parse().unwrap())
        }
        ExprContextAll::VarContext(ctx) => Expr::Var(ctx.name.as_ref().unwrap().to_string()),
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
            name: ctx.name.as_ref().unwrap().text.to_string(),
            type_: build_type(ctx.atype.as_ref().unwrap()),
        },
        DeclContextAll::DeclFunContext(ctx) => Decl::DeclFun {
            annotations: Vec::new(), // TODO: convert annotations
            name: ctx.name.as_ref().unwrap().to_string(),
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
