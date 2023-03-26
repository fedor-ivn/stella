#![allow(nonstandard_style)]
// Generated from src/stellaParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::stellaparser::*;

pub trait stellaParserListener<'input> : ParseTreeListener<'input,stellaParserContextType>{
/**
 * Enter a parse tree produced by {@link stellaParser#start_Program}.
 * @param ctx the parse tree
 */
fn enter_start_Program(&mut self, _ctx: &Start_ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Program}.
 * @param ctx the parse tree
 */
fn exit_start_Program(&mut self, _ctx: &Start_ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Expr}.
 * @param ctx the parse tree
 */
fn enter_start_Expr(&mut self, _ctx: &Start_ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Expr}.
 * @param ctx the parse tree
 */
fn exit_start_Expr(&mut self, _ctx: &Start_ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Type}.
 * @param ctx the parse tree
 */
fn enter_start_Type(&mut self, _ctx: &Start_TypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Type}.
 * @param ctx the parse tree
 */
fn exit_start_Type(&mut self, _ctx: &Start_TypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#program}.
 * @param ctx the parse tree
 */
fn enter_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#program}.
 * @param ctx the parse tree
 */
fn exit_program(&mut self, _ctx: &ProgramContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LanguageCore}
 * labeled alternative in {@link stellaParser#languageDecl}.
 * @param ctx the parse tree
 */
fn enter_LanguageCore(&mut self, _ctx: &LanguageCoreContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LanguageCore}
 * labeled alternative in {@link stellaParser#languageDecl}.
 * @param ctx the parse tree
 */
fn exit_LanguageCore(&mut self, _ctx: &LanguageCoreContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AnExtension}
 * labeled alternative in {@link stellaParser#extension}.
 * @param ctx the parse tree
 */
fn enter_AnExtension(&mut self, _ctx: &AnExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AnExtension}
 * labeled alternative in {@link stellaParser#extension}.
 * @param ctx the parse tree
 */
fn exit_AnExtension(&mut self, _ctx: &AnExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DeclFun}
 * labeled alternative in {@link stellaParser#decl}.
 * @param ctx the parse tree
 */
fn enter_DeclFun(&mut self, _ctx: &DeclFunContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DeclFun}
 * labeled alternative in {@link stellaParser#decl}.
 * @param ctx the parse tree
 */
fn exit_DeclFun(&mut self, _ctx: &DeclFunContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DeclTypeAlias}
 * labeled alternative in {@link stellaParser#decl}.
 * @param ctx the parse tree
 */
fn enter_DeclTypeAlias(&mut self, _ctx: &DeclTypeAliasContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DeclTypeAlias}
 * labeled alternative in {@link stellaParser#decl}.
 * @param ctx the parse tree
 */
fn exit_DeclTypeAlias(&mut self, _ctx: &DeclTypeAliasContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code InlineAnnotation}
 * labeled alternative in {@link stellaParser#annotation}.
 * @param ctx the parse tree
 */
fn enter_InlineAnnotation(&mut self, _ctx: &InlineAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code InlineAnnotation}
 * labeled alternative in {@link stellaParser#annotation}.
 * @param ctx the parse tree
 */
fn exit_InlineAnnotation(&mut self, _ctx: &InlineAnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#paramDecl}.
 * @param ctx the parse tree
 */
fn enter_paramDecl(&mut self, _ctx: &ParamDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#paramDecl}.
 * @param ctx the parse tree
 */
fn exit_paramDecl(&mut self, _ctx: &ParamDeclContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ConstTrue}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ConstTrue(&mut self, _ctx: &ConstTrueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ConstTrue}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ConstTrue(&mut self, _ctx: &ConstTrueContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Succ}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_Succ(&mut self, _ctx: &SuccContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Succ}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_Succ(&mut self, _ctx: &SuccContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Var}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_Var(&mut self, _ctx: &VarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Var}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_Var(&mut self, _ctx: &VarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprParens}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ExprParens(&mut self, _ctx: &ExprParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprParens}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ExprParens(&mut self, _ctx: &ExprParensContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code NatRec}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_NatRec(&mut self, _ctx: &NatRecContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code NatRec}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_NatRec(&mut self, _ctx: &NatRecContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ConstFalse}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ConstFalse(&mut self, _ctx: &ConstFalseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ConstFalse}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ConstFalse(&mut self, _ctx: &ConstFalseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Abstraction}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_Abstraction(&mut self, _ctx: &AbstractionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Abstraction}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_Abstraction(&mut self, _ctx: &AbstractionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code If}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_If(&mut self, _ctx: &IfContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code If}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_If(&mut self, _ctx: &IfContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ConstInt}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ConstInt(&mut self, _ctx: &ConstIntContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ConstInt}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ConstInt(&mut self, _ctx: &ConstIntContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Application}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_Application(&mut self, _ctx: &ApplicationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Application}
 * labeled alternative in {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_Application(&mut self, _ctx: &ApplicationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeBool}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn enter_TypeBool(&mut self, _ctx: &TypeBoolContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeBool}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn exit_TypeBool(&mut self, _ctx: &TypeBoolContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeNat}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn enter_TypeNat(&mut self, _ctx: &TypeNatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeNat}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn exit_TypeNat(&mut self, _ctx: &TypeNatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeFun}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn enter_TypeFun(&mut self, _ctx: &TypeFunContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeFun}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn exit_TypeFun(&mut self, _ctx: &TypeFunContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TypeParens}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn enter_TypeParens(&mut self, _ctx: &TypeParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TypeParens}
 * labeled alternative in {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn exit_TypeParens(&mut self, _ctx: &TypeParensContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : stellaParserListener<'input> }


