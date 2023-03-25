use crate::ast::*;

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

pub fn typecheck_program(program: &Program) -> Result<(), String> {
    match program {
        Program::AProgram {
            language_decl,
            extensions,
            decls,
        } => decls.iter().map(typecheck_decl).collect(),
    }
}
