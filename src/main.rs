use std::borrow::Borrow;

use antlr_rust::parser_rule_context::BaseParserRuleContext;
use stellaparser::*;

use crate::stellaparser::ProgramContextExt;

mod stellalexer;
mod stellaparser;
mod stellaparserlistener;

enum Program {
    AProgram {
        language_decl: LanguageDecl,
        extensions: Vec<Extension>,
        decls: Vec<Decl>,
    },
}

enum LanguageDecl {
    LanguageCore,
}

enum Extension {
    AnExtension { extension_names: Vec<ExtensionName> },
}

enum Decl {
    DeclFun {
        annotations: Vec<Annotation>,
        name: String,
        param_decls: Vec<ParamDecl>,
        return_type: Option<Type>,
        throws_type: Option<Type>,
        local_decls: Vec<Decl>,
        return_expr: Expr,
    },
    DeclTypeAlias {
        name: String,
        type_: Type,
    },
}

enum Annotation {
    InlineAnnotation,
}

enum ParamDecl {
    AParamDecl { name: String, type_: Type },
}

enum Expr {
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Abstraction(Vec<ParamDecl>, Box<Expr>),
    Succ(Box<Expr>),
    NatRec(Box<Expr>, Box<Expr>, Box<Expr>),
    ConstTrue,
    ConstFalse,
    ConstInt(isize),
    Var(String),
}

enum Type {
    TypeFun(Vec<Type>, Box<Type>),
    TypeBool,
    TypeNat,
}

type ExtensionName = String;

fn build_param_decl<'input>(
    ctx: &BaseParserRuleContext<'input, ParamDeclContextExt<'input>>,
) -> ParamDecl {
    return ParamDecl::AParamDecl {
        name: ctx.name.as_ref().unwrap().to_string(),
        type_: build_type(ctx.paramType.as_ref().unwrap()),
    };
}

fn build_expr<'input>(ctx: &ExprContextAll<'input>) -> Expr {
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

fn build_type<'input>(ctx: &StellatypeContextAll<'input>) -> Type {
    match ctx {
        StellatypeContextAll::TypeBoolContext(_) => Type::TypeBool,
        StellatypeContextAll::TypeFunContext(ctx) => Type::TypeFun(
            ctx.paramTypes
                .iter()
                .map(|type_| build_type(type_))
                .collect(),
            Box::new(build_type(ctx.returnType.as_ref().unwrap())),
        ),
        StellatypeContextAll::Error(_) => todo!(),
        StellatypeContextAll::TypeNatContext(_) => Type::TypeNat,
    }
}

fn build_decl<'input>(ctx: &DeclContextAll<'input>) -> Decl {
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

fn build_program<'input>(
    ctx: &BaseParserRuleContext<'input, ProgramContextExt<'input>>,
) -> Program {
    let ctx: &ProgramContextExt = (*ctx).borrow();
    Program::AProgram {
        language_decl: LanguageDecl::LanguageCore, // TODO: language decl
        extensions: Vec::new(),                    // TODO: extensions
        decls: ctx.decls.iter().map(|decl| build_decl(decl)).collect(),
    }
}

fn typecheck_decl(decl: &Decl) -> Result<(), String> {
    match decl {
        Decl::DeclFun {
            annotations,
            name,
            param_decls,
            return_type,
            throws_type,
            local_decls,
            return_expr,
        } => Ok(()),
        Decl::DeclTypeAlias { name, type_ } => todo!(),
    }
}

fn typecheck_program(program: &Program) -> Result<(), String> {
    match program {
        Program::AProgram {
            language_decl,
            extensions,
            decls,
        } => decls.iter().map(typecheck_decl).collect(),
    }
}

fn main() {
    println!("test started");
    // let input_data = "language core; fn main(x : Nat) -> Nat { return succ(x); }";
    let input_data: std::io::Result<String> = std::io::stdin().lines().collect();
    let input_stream = antlr_rust::InputStream::new(input_data.as_ref().unwrap().as_str());
    let lexer = stellalexer::stellaLexer::new(input_stream);
    let token_stream = antlr_rust::common_token_stream::CommonTokenStream::new(lexer);
    let mut parser = stellaparser::stellaParser::new(token_stream);
    println!("\nstart parsing parser_test_csv");
    let result = parser.program();

    let program = build_program(&*result.expect("Parse Error"));

    typecheck_program(&program).expect("Type Error");

    println!("Everything looks fine!");
}
