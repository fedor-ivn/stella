#![allow(dead_code)] // A lot of stuff here is yet to be used

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
        throws_types: Vec<Type>,
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

pub struct Binding {
    name: String,
    expr: Expr,
}

pub struct MatchCase {
    pattern: Pattern,
    expr: Expr,
}

pub enum Expr {
    DotRecord(Box<Expr>, String),
    DotTuple(Box<Expr>, usize),
    ConstTrue,
    ConstFalse,
    ConstUnit,
    ConstInt(isize),
    ConstMemory(String),
    Var(String),
    Inl(Box<Expr>),
    Inr(Box<Expr>),
    Cons(Box<Expr>, Box<Expr>),
    ListHead(Box<Expr>),
    ListIsEmpty(Box<Expr>),
    ListTail(Box<Expr>),
    Succ(Box<Expr>),
    LogicalNot(Box<Expr>),
    NatPred(Box<Expr>),
    NatIsZero(Box<Expr>),
    Fix(Box<Expr>),
    NatRec(Box<Expr>, Box<Expr>, Box<Expr>),
    Fold(Type, Box<Expr>),
    Unfold(Type, Box<Expr>),
    Application(Box<Expr>, Vec<Expr>),
    TypeApplication(Box<Expr>, Type),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    LogicalAnd(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    LogicalOr(Box<Expr>, Box<Expr>),
    TypeAscription(Box<Expr>, Type),
    Abstraction(Vec<ParamDecl>, Box<Expr>),
    TypeAbstraction(Vec<String>, Box<Expr>),
    Tuple(Vec<Expr>),
    Record(Vec<Binding>),
    Variant(String, Box<Expr>),
    Match(Box<Expr>, Vec<MatchCase>),
    List(Vec<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    LessThanOrEqual(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
    GreaterThanOrEqual(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    Assignment(Box<Expr>, Box<Expr>),
    TypeCast(Box<Expr>, Type),
    Reference(Box<Expr>),
    Dereference(Box<Expr>),
    Panic,
    Throw(Box<Expr>),
    TryCatch(Box<Expr>, Pattern, Box<Expr>),
    TryWith(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<PatternBinding>, Box<Expr>),
    LetRec(Vec<PatternBinding>, Box<Expr>),
    Sequence(Box<Expr>, Option<Box<Expr>>),
}

pub struct RecordFieldType {
    label: String,
    type_: Type,
}

pub struct VariantFieldType {
    label: String,
    type_: Option<Type>,
}

pub enum Type {
    Bool,
    Nat,
    Fun(Vec<Type>, Box<Type>),
    ForAll(Vec<String>, Box<Type>),
    Rec(String, Box<Type>),
    Sum(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Record(Vec<RecordFieldType>),
    Variant(Vec<VariantFieldType>),
    List(Vec<Type>),
    Unit,
    Top,
    Ref(Box<Type>),
    Bottom,
    Var(String),
}

pub struct PatternBinding {
    pattern: Pattern,
    rhs: Expr,
}

pub struct LabelledPattern {
    label: String,
    pattern: Pattern,
}

pub enum Pattern {
    Variant(String, Option<Box<Pattern>>),
    Inl(Box<Pattern>),
    Inr(Box<Pattern>),
    Tuple(Vec<Pattern>),
    Labelled(Box<LabelledPattern>),
    Record(Vec<LabelledPattern>),
    List(Vec<Pattern>),
    Cons(Box<Pattern>, Box<Pattern>),
    False,
    True,
    Unit,
    Int(isize),
    Succ(Box<Pattern>),
    Var(String),
}
