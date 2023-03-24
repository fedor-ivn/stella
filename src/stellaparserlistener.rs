#![allow(nonstandard_style)]
// Generated from src/stellaParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::stellaparser::*;

pub trait stellaParserListener<'input> : ParseTreeListener<'input,stellaParserContextType>{
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListStellaIdent}.
 * @param ctx the parse tree
 */
fn enter_start_ListStellaIdent(&mut self, _ctx: &Start_ListStellaIdentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListStellaIdent}.
 * @param ctx the parse tree
 */
fn exit_start_ListStellaIdent(&mut self, _ctx: &Start_ListStellaIdentContext<'input>) { }
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
 * Enter a parse tree produced by {@link stellaParser#start_LanguageDecl}.
 * @param ctx the parse tree
 */
fn enter_start_LanguageDecl(&mut self, _ctx: &Start_LanguageDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_LanguageDecl}.
 * @param ctx the parse tree
 */
fn exit_start_LanguageDecl(&mut self, _ctx: &Start_LanguageDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Extension}.
 * @param ctx the parse tree
 */
fn enter_start_Extension(&mut self, _ctx: &Start_ExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Extension}.
 * @param ctx the parse tree
 */
fn exit_start_Extension(&mut self, _ctx: &Start_ExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListExtensionName}.
 * @param ctx the parse tree
 */
fn enter_start_ListExtensionName(&mut self, _ctx: &Start_ListExtensionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListExtensionName}.
 * @param ctx the parse tree
 */
fn exit_start_ListExtensionName(&mut self, _ctx: &Start_ListExtensionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListExtension}.
 * @param ctx the parse tree
 */
fn enter_start_ListExtension(&mut self, _ctx: &Start_ListExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListExtension}.
 * @param ctx the parse tree
 */
fn exit_start_ListExtension(&mut self, _ctx: &Start_ListExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Decl}.
 * @param ctx the parse tree
 */
fn enter_start_Decl(&mut self, _ctx: &Start_DeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Decl}.
 * @param ctx the parse tree
 */
fn exit_start_Decl(&mut self, _ctx: &Start_DeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListDecl}.
 * @param ctx the parse tree
 */
fn enter_start_ListDecl(&mut self, _ctx: &Start_ListDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListDecl}.
 * @param ctx the parse tree
 */
fn exit_start_ListDecl(&mut self, _ctx: &Start_ListDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_LocalDecl}.
 * @param ctx the parse tree
 */
fn enter_start_LocalDecl(&mut self, _ctx: &Start_LocalDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_LocalDecl}.
 * @param ctx the parse tree
 */
fn exit_start_LocalDecl(&mut self, _ctx: &Start_LocalDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListLocalDecl}.
 * @param ctx the parse tree
 */
fn enter_start_ListLocalDecl(&mut self, _ctx: &Start_ListLocalDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListLocalDecl}.
 * @param ctx the parse tree
 */
fn exit_start_ListLocalDecl(&mut self, _ctx: &Start_ListLocalDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Annotation}.
 * @param ctx the parse tree
 */
fn enter_start_Annotation(&mut self, _ctx: &Start_AnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Annotation}.
 * @param ctx the parse tree
 */
fn exit_start_Annotation(&mut self, _ctx: &Start_AnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListAnnotation}.
 * @param ctx the parse tree
 */
fn enter_start_ListAnnotation(&mut self, _ctx: &Start_ListAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListAnnotation}.
 * @param ctx the parse tree
 */
fn exit_start_ListAnnotation(&mut self, _ctx: &Start_ListAnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ParamDecl}.
 * @param ctx the parse tree
 */
fn enter_start_ParamDecl(&mut self, _ctx: &Start_ParamDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ParamDecl}.
 * @param ctx the parse tree
 */
fn exit_start_ParamDecl(&mut self, _ctx: &Start_ParamDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListParamDecl}.
 * @param ctx the parse tree
 */
fn enter_start_ListParamDecl(&mut self, _ctx: &Start_ListParamDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListParamDecl}.
 * @param ctx the parse tree
 */
fn exit_start_ListParamDecl(&mut self, _ctx: &Start_ListParamDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ReturnType}.
 * @param ctx the parse tree
 */
fn enter_start_ReturnType(&mut self, _ctx: &Start_ReturnTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ReturnType}.
 * @param ctx the parse tree
 */
fn exit_start_ReturnType(&mut self, _ctx: &Start_ReturnTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ThrowType}.
 * @param ctx the parse tree
 */
fn enter_start_ThrowType(&mut self, _ctx: &Start_ThrowTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ThrowType}.
 * @param ctx the parse tree
 */
fn exit_start_ThrowType(&mut self, _ctx: &Start_ThrowTypeContext<'input>) { }
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
 * Enter a parse tree produced by {@link stellaParser#start_ListExpr}.
 * @param ctx the parse tree
 */
fn enter_start_ListExpr(&mut self, _ctx: &Start_ListExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListExpr}.
 * @param ctx the parse tree
 */
fn exit_start_ListExpr(&mut self, _ctx: &Start_ListExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_MatchCase}.
 * @param ctx the parse tree
 */
fn enter_start_MatchCase(&mut self, _ctx: &Start_MatchCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_MatchCase}.
 * @param ctx the parse tree
 */
fn exit_start_MatchCase(&mut self, _ctx: &Start_MatchCaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListMatchCase}.
 * @param ctx the parse tree
 */
fn enter_start_ListMatchCase(&mut self, _ctx: &Start_ListMatchCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListMatchCase}.
 * @param ctx the parse tree
 */
fn exit_start_ListMatchCase(&mut self, _ctx: &Start_ListMatchCaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_OptionalTyping}.
 * @param ctx the parse tree
 */
fn enter_start_OptionalTyping(&mut self, _ctx: &Start_OptionalTypingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_OptionalTyping}.
 * @param ctx the parse tree
 */
fn exit_start_OptionalTyping(&mut self, _ctx: &Start_OptionalTypingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_PatternData}.
 * @param ctx the parse tree
 */
fn enter_start_PatternData(&mut self, _ctx: &Start_PatternDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_PatternData}.
 * @param ctx the parse tree
 */
fn exit_start_PatternData(&mut self, _ctx: &Start_PatternDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ExprData}.
 * @param ctx the parse tree
 */
fn enter_start_ExprData(&mut self, _ctx: &Start_ExprDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ExprData}.
 * @param ctx the parse tree
 */
fn exit_start_ExprData(&mut self, _ctx: &Start_ExprDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Pattern}.
 * @param ctx the parse tree
 */
fn enter_start_Pattern(&mut self, _ctx: &Start_PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Pattern}.
 * @param ctx the parse tree
 */
fn exit_start_Pattern(&mut self, _ctx: &Start_PatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListPattern}.
 * @param ctx the parse tree
 */
fn enter_start_ListPattern(&mut self, _ctx: &Start_ListPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListPattern}.
 * @param ctx the parse tree
 */
fn exit_start_ListPattern(&mut self, _ctx: &Start_ListPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_LabelledPattern}.
 * @param ctx the parse tree
 */
fn enter_start_LabelledPattern(&mut self, _ctx: &Start_LabelledPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_LabelledPattern}.
 * @param ctx the parse tree
 */
fn exit_start_LabelledPattern(&mut self, _ctx: &Start_LabelledPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListLabelledPattern}.
 * @param ctx the parse tree
 */
fn enter_start_ListLabelledPattern(&mut self, _ctx: &Start_ListLabelledPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListLabelledPattern}.
 * @param ctx the parse tree
 */
fn exit_start_ListLabelledPattern(&mut self, _ctx: &Start_ListLabelledPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Binding}.
 * @param ctx the parse tree
 */
fn enter_start_Binding(&mut self, _ctx: &Start_BindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Binding}.
 * @param ctx the parse tree
 */
fn exit_start_Binding(&mut self, _ctx: &Start_BindingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListBinding}.
 * @param ctx the parse tree
 */
fn enter_start_ListBinding(&mut self, _ctx: &Start_ListBindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListBinding}.
 * @param ctx the parse tree
 */
fn exit_start_ListBinding(&mut self, _ctx: &Start_ListBindingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Expr1}.
 * @param ctx the parse tree
 */
fn enter_start_Expr1(&mut self, _ctx: &Start_Expr1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Expr1}.
 * @param ctx the parse tree
 */
fn exit_start_Expr1(&mut self, _ctx: &Start_Expr1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Expr2}.
 * @param ctx the parse tree
 */
fn enter_start_Expr2(&mut self, _ctx: &Start_Expr2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Expr2}.
 * @param ctx the parse tree
 */
fn exit_start_Expr2(&mut self, _ctx: &Start_Expr2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Expr3}.
 * @param ctx the parse tree
 */
fn enter_start_Expr3(&mut self, _ctx: &Start_Expr3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Expr3}.
 * @param ctx the parse tree
 */
fn exit_start_Expr3(&mut self, _ctx: &Start_Expr3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Expr4}.
 * @param ctx the parse tree
 */
fn enter_start_Expr4(&mut self, _ctx: &Start_Expr4Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Expr4}.
 * @param ctx the parse tree
 */
fn exit_start_Expr4(&mut self, _ctx: &Start_Expr4Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Expr5}.
 * @param ctx the parse tree
 */
fn enter_start_Expr5(&mut self, _ctx: &Start_Expr5Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Expr5}.
 * @param ctx the parse tree
 */
fn exit_start_Expr5(&mut self, _ctx: &Start_Expr5Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Expr6}.
 * @param ctx the parse tree
 */
fn enter_start_Expr6(&mut self, _ctx: &Start_Expr6Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Expr6}.
 * @param ctx the parse tree
 */
fn exit_start_Expr6(&mut self, _ctx: &Start_Expr6Context<'input>) { }
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
 * Enter a parse tree produced by {@link stellaParser#start_Type1}.
 * @param ctx the parse tree
 */
fn enter_start_Type1(&mut self, _ctx: &Start_Type1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Type1}.
 * @param ctx the parse tree
 */
fn exit_start_Type1(&mut self, _ctx: &Start_Type1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Type2}.
 * @param ctx the parse tree
 */
fn enter_start_Type2(&mut self, _ctx: &Start_Type2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Type2}.
 * @param ctx the parse tree
 */
fn exit_start_Type2(&mut self, _ctx: &Start_Type2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Type3}.
 * @param ctx the parse tree
 */
fn enter_start_Type3(&mut self, _ctx: &Start_Type3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Type3}.
 * @param ctx the parse tree
 */
fn exit_start_Type3(&mut self, _ctx: &Start_Type3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListType}.
 * @param ctx the parse tree
 */
fn enter_start_ListType(&mut self, _ctx: &Start_ListTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListType}.
 * @param ctx the parse tree
 */
fn exit_start_ListType(&mut self, _ctx: &Start_ListTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_VariantFieldType}.
 * @param ctx the parse tree
 */
fn enter_start_VariantFieldType(&mut self, _ctx: &Start_VariantFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_VariantFieldType}.
 * @param ctx the parse tree
 */
fn exit_start_VariantFieldType(&mut self, _ctx: &Start_VariantFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListVariantFieldType}.
 * @param ctx the parse tree
 */
fn enter_start_ListVariantFieldType(&mut self, _ctx: &Start_ListVariantFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListVariantFieldType}.
 * @param ctx the parse tree
 */
fn exit_start_ListVariantFieldType(&mut self, _ctx: &Start_ListVariantFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_RecordFieldType}.
 * @param ctx the parse tree
 */
fn enter_start_RecordFieldType(&mut self, _ctx: &Start_RecordFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_RecordFieldType}.
 * @param ctx the parse tree
 */
fn exit_start_RecordFieldType(&mut self, _ctx: &Start_RecordFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_ListRecordFieldType}.
 * @param ctx the parse tree
 */
fn enter_start_ListRecordFieldType(&mut self, _ctx: &Start_ListRecordFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_ListRecordFieldType}.
 * @param ctx the parse tree
 */
fn exit_start_ListRecordFieldType(&mut self, _ctx: &Start_ListRecordFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#start_Typing}.
 * @param ctx the parse tree
 */
fn enter_start_Typing(&mut self, _ctx: &Start_TypingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#start_Typing}.
 * @param ctx the parse tree
 */
fn exit_start_Typing(&mut self, _ctx: &Start_TypingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listStellaIdent}.
 * @param ctx the parse tree
 */
fn enter_listStellaIdent(&mut self, _ctx: &ListStellaIdentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listStellaIdent}.
 * @param ctx the parse tree
 */
fn exit_listStellaIdent(&mut self, _ctx: &ListStellaIdentContext<'input>) { }
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
 * Enter a parse tree produced by {@link stellaParser#languageDecl}.
 * @param ctx the parse tree
 */
fn enter_languageDecl(&mut self, _ctx: &LanguageDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#languageDecl}.
 * @param ctx the parse tree
 */
fn exit_languageDecl(&mut self, _ctx: &LanguageDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#extension}.
 * @param ctx the parse tree
 */
fn enter_extension(&mut self, _ctx: &ExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#extension}.
 * @param ctx the parse tree
 */
fn exit_extension(&mut self, _ctx: &ExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listExtensionName}.
 * @param ctx the parse tree
 */
fn enter_listExtensionName(&mut self, _ctx: &ListExtensionNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listExtensionName}.
 * @param ctx the parse tree
 */
fn exit_listExtensionName(&mut self, _ctx: &ListExtensionNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listExtension}.
 * @param ctx the parse tree
 */
fn enter_listExtension(&mut self, _ctx: &ListExtensionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listExtension}.
 * @param ctx the parse tree
 */
fn exit_listExtension(&mut self, _ctx: &ListExtensionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#decl}.
 * @param ctx the parse tree
 */
fn enter_decl(&mut self, _ctx: &DeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#decl}.
 * @param ctx the parse tree
 */
fn exit_decl(&mut self, _ctx: &DeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listDecl}.
 * @param ctx the parse tree
 */
fn enter_listDecl(&mut self, _ctx: &ListDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listDecl}.
 * @param ctx the parse tree
 */
fn exit_listDecl(&mut self, _ctx: &ListDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#localDecl}.
 * @param ctx the parse tree
 */
fn enter_localDecl(&mut self, _ctx: &LocalDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#localDecl}.
 * @param ctx the parse tree
 */
fn exit_localDecl(&mut self, _ctx: &LocalDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listLocalDecl}.
 * @param ctx the parse tree
 */
fn enter_listLocalDecl(&mut self, _ctx: &ListLocalDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listLocalDecl}.
 * @param ctx the parse tree
 */
fn exit_listLocalDecl(&mut self, _ctx: &ListLocalDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#annotation}.
 * @param ctx the parse tree
 */
fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#annotation}.
 * @param ctx the parse tree
 */
fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listAnnotation}.
 * @param ctx the parse tree
 */
fn enter_listAnnotation(&mut self, _ctx: &ListAnnotationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listAnnotation}.
 * @param ctx the parse tree
 */
fn exit_listAnnotation(&mut self, _ctx: &ListAnnotationContext<'input>) { }
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
 * Enter a parse tree produced by {@link stellaParser#listParamDecl}.
 * @param ctx the parse tree
 */
fn enter_listParamDecl(&mut self, _ctx: &ListParamDeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listParamDecl}.
 * @param ctx the parse tree
 */
fn exit_listParamDecl(&mut self, _ctx: &ListParamDeclContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#returnType}.
 * @param ctx the parse tree
 */
fn enter_returnType(&mut self, _ctx: &ReturnTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#returnType}.
 * @param ctx the parse tree
 */
fn exit_returnType(&mut self, _ctx: &ReturnTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#throwType}.
 * @param ctx the parse tree
 */
fn enter_throwType(&mut self, _ctx: &ThrowTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#throwType}.
 * @param ctx the parse tree
 */
fn exit_throwType(&mut self, _ctx: &ThrowTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listExpr}.
 * @param ctx the parse tree
 */
fn enter_listExpr(&mut self, _ctx: &ListExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listExpr}.
 * @param ctx the parse tree
 */
fn exit_listExpr(&mut self, _ctx: &ListExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#matchCase}.
 * @param ctx the parse tree
 */
fn enter_matchCase(&mut self, _ctx: &MatchCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#matchCase}.
 * @param ctx the parse tree
 */
fn exit_matchCase(&mut self, _ctx: &MatchCaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listMatchCase}.
 * @param ctx the parse tree
 */
fn enter_listMatchCase(&mut self, _ctx: &ListMatchCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listMatchCase}.
 * @param ctx the parse tree
 */
fn exit_listMatchCase(&mut self, _ctx: &ListMatchCaseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#optionalTyping}.
 * @param ctx the parse tree
 */
fn enter_optionalTyping(&mut self, _ctx: &OptionalTypingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#optionalTyping}.
 * @param ctx the parse tree
 */
fn exit_optionalTyping(&mut self, _ctx: &OptionalTypingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#patternData}.
 * @param ctx the parse tree
 */
fn enter_patternData(&mut self, _ctx: &PatternDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#patternData}.
 * @param ctx the parse tree
 */
fn exit_patternData(&mut self, _ctx: &PatternDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#exprData}.
 * @param ctx the parse tree
 */
fn enter_exprData(&mut self, _ctx: &ExprDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#exprData}.
 * @param ctx the parse tree
 */
fn exit_exprData(&mut self, _ctx: &ExprDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#pattern}.
 * @param ctx the parse tree
 */
fn enter_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#pattern}.
 * @param ctx the parse tree
 */
fn exit_pattern(&mut self, _ctx: &PatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listPattern}.
 * @param ctx the parse tree
 */
fn enter_listPattern(&mut self, _ctx: &ListPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listPattern}.
 * @param ctx the parse tree
 */
fn exit_listPattern(&mut self, _ctx: &ListPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#labelledPattern}.
 * @param ctx the parse tree
 */
fn enter_labelledPattern(&mut self, _ctx: &LabelledPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#labelledPattern}.
 * @param ctx the parse tree
 */
fn exit_labelledPattern(&mut self, _ctx: &LabelledPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listLabelledPattern}.
 * @param ctx the parse tree
 */
fn enter_listLabelledPattern(&mut self, _ctx: &ListLabelledPatternContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listLabelledPattern}.
 * @param ctx the parse tree
 */
fn exit_listLabelledPattern(&mut self, _ctx: &ListLabelledPatternContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#binding}.
 * @param ctx the parse tree
 */
fn enter_binding(&mut self, _ctx: &BindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#binding}.
 * @param ctx the parse tree
 */
fn exit_binding(&mut self, _ctx: &BindingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listBinding}.
 * @param ctx the parse tree
 */
fn enter_listBinding(&mut self, _ctx: &ListBindingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listBinding}.
 * @param ctx the parse tree
 */
fn exit_listBinding(&mut self, _ctx: &ListBindingContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#expr1}.
 * @param ctx the parse tree
 */
fn enter_expr1(&mut self, _ctx: &Expr1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#expr1}.
 * @param ctx the parse tree
 */
fn exit_expr1(&mut self, _ctx: &Expr1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#expr2}.
 * @param ctx the parse tree
 */
fn enter_expr2(&mut self, _ctx: &Expr2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#expr2}.
 * @param ctx the parse tree
 */
fn exit_expr2(&mut self, _ctx: &Expr2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#expr3}.
 * @param ctx the parse tree
 */
fn enter_expr3(&mut self, _ctx: &Expr3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#expr3}.
 * @param ctx the parse tree
 */
fn exit_expr3(&mut self, _ctx: &Expr3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#expr4}.
 * @param ctx the parse tree
 */
fn enter_expr4(&mut self, _ctx: &Expr4Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#expr4}.
 * @param ctx the parse tree
 */
fn exit_expr4(&mut self, _ctx: &Expr4Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#expr5}.
 * @param ctx the parse tree
 */
fn enter_expr5(&mut self, _ctx: &Expr5Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#expr5}.
 * @param ctx the parse tree
 */
fn exit_expr5(&mut self, _ctx: &Expr5Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#expr6}.
 * @param ctx the parse tree
 */
fn enter_expr6(&mut self, _ctx: &Expr6Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#expr6}.
 * @param ctx the parse tree
 */
fn exit_expr6(&mut self, _ctx: &Expr6Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn enter_stellatype(&mut self, _ctx: &StellatypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#stellatype}.
 * @param ctx the parse tree
 */
fn exit_stellatype(&mut self, _ctx: &StellatypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#type1}.
 * @param ctx the parse tree
 */
fn enter_type1(&mut self, _ctx: &Type1Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#type1}.
 * @param ctx the parse tree
 */
fn exit_type1(&mut self, _ctx: &Type1Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#type2}.
 * @param ctx the parse tree
 */
fn enter_type2(&mut self, _ctx: &Type2Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#type2}.
 * @param ctx the parse tree
 */
fn exit_type2(&mut self, _ctx: &Type2Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#type3}.
 * @param ctx the parse tree
 */
fn enter_type3(&mut self, _ctx: &Type3Context<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#type3}.
 * @param ctx the parse tree
 */
fn exit_type3(&mut self, _ctx: &Type3Context<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listType}.
 * @param ctx the parse tree
 */
fn enter_listType(&mut self, _ctx: &ListTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listType}.
 * @param ctx the parse tree
 */
fn exit_listType(&mut self, _ctx: &ListTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#variantFieldType}.
 * @param ctx the parse tree
 */
fn enter_variantFieldType(&mut self, _ctx: &VariantFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#variantFieldType}.
 * @param ctx the parse tree
 */
fn exit_variantFieldType(&mut self, _ctx: &VariantFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listVariantFieldType}.
 * @param ctx the parse tree
 */
fn enter_listVariantFieldType(&mut self, _ctx: &ListVariantFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listVariantFieldType}.
 * @param ctx the parse tree
 */
fn exit_listVariantFieldType(&mut self, _ctx: &ListVariantFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#recordFieldType}.
 * @param ctx the parse tree
 */
fn enter_recordFieldType(&mut self, _ctx: &RecordFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#recordFieldType}.
 * @param ctx the parse tree
 */
fn exit_recordFieldType(&mut self, _ctx: &RecordFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#listRecordFieldType}.
 * @param ctx the parse tree
 */
fn enter_listRecordFieldType(&mut self, _ctx: &ListRecordFieldTypeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#listRecordFieldType}.
 * @param ctx the parse tree
 */
fn exit_listRecordFieldType(&mut self, _ctx: &ListRecordFieldTypeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link stellaParser#typing}.
 * @param ctx the parse tree
 */
fn enter_typing(&mut self, _ctx: &TypingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link stellaParser#typing}.
 * @param ctx the parse tree
 */
fn exit_typing(&mut self, _ctx: &TypingContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : stellaParserListener<'input> }


