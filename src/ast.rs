pub type ExtensionName = String;

pub struct Program {
    pub language_decl: LanguageDecl,
    pub extensions: Vec<Extension>,
    pub decls: Vec<Decl>,
}

pub enum LanguageDecl {
    LanguageCore,
}

pub struct Extension {
    pub extension_names: Vec<ExtensionName>,
}

pub enum Decl {
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

pub enum Annotation {
    InlineAnnotation,
}

pub struct ParamDecl {
    pub name: String,
    pub type_: Type,
}

pub enum Expr {
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Abstraction(Vec<ParamDecl>, Box<Expr>),
    Succ(Box<Expr>),
    NatRec(Box<Expr>, Box<Expr>, Box<Expr>),
    ConstTrue,
    ConstFalse,
    ConstInt(isize),
    Var(String),
}

pub enum Type {
    Fun(Vec<Type>, Box<Type>),
    Bool,
    Nat,
}
