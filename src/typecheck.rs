use crate::ast::*;

fn typecheck_decl(decl: &Decl) -> Result<(), String> {
    match decl {
        Decl::DeclFun { .. } => Ok(()),
        Decl::DeclTypeAlias { name: _, type_: _ } => todo!(),
    }
}

pub fn typecheck_program(program: &Program) -> Result<(), String> {
    match program {
        Program::AProgram { decls, .. } => decls.iter().try_for_each(typecheck_decl),
    }
}
