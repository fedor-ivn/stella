pub type ExtensionName = String;

pub enum Program {
    AProgram {
        language_decl: LanguageDecl,
        extensions: Vec<Extension>,
        decls: Vec<Decl>,
    },
}

pub enum LanguageDecl {
    LanguageCore,
}

pub enum Extension {
    AnExtension { extension_names: Vec<ExtensionName> },
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

pub enum ParamDecl {
    AParamDecl { name: String, type_: Type },
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
    TypeFun(Vec<Type>, Box<Type>),
    TypeBool,
    TypeNat,
}
