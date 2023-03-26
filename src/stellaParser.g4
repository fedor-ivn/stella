parser grammar stellaParser;

options {
	tokenVocab = stellaLexer;
}

start_Program: x = program EOF;
start_Expr: x = expr EOF;
start_Type: x = stellatype EOF;

program:
	languageDecl (extensions += extension)* (decls += decl)*;

languageDecl: 'language' 'core' ';' # LanguageCore;

extension:
	EXTEND WITH extensionNames += ExtensionName (
		',' extensionNames += ExtensionName
	) ';' # AnExtension;

decl: (annotations += annotation)* 'fn' name = StellaIdent '(' (
		paramDecls += paramDecl (',' paramDecls += paramDecl)*
	)? ')' ('->' returnType = stellatype)? (
		'throws' throwType = stellatype
	)? '{' (localDecls += decl)* 'return' returnExpr = expr ';' '}'	# DeclFun
	| 'type' name = StellaIdent '=' atype = stellatype ';'			# DeclTypeAlias;

annotation: 'inline' # InlineAnnotation;
paramDecl: name = StellaIdent ':' paramType = stellatype;

expr:
	'if' condition = expr 'then' thenExpr = expr 'else' elseExpr = expr	# If
	| 'true'															# ConstTrue
	| 'false'															# ConstFalse
	| n = INTEGER														# ConstInt
	| 'succ' '(' n = expr ')'											# Succ
	| 'Nat::rec' '(' n = expr ',' initial = expr ',' step = expr ')'	# NatRec
	| name = StellaIdent												# Var
	| 'fn' '('  (
		paramDecls += paramDecl (',' paramDecls += paramDecl)*
	)?  ')' '{' 'return' returnExpr=expr ';' '}' # Abstraction
	| fun=expr '(' (args+=expr (',' args+=expr)*)? ')' # Application
    | '(' expr_=expr ')' # ExprParens
	;

stellatype:
	'Bool'	# TypeBool
	| 'Nat'	# TypeNat
	| 'fn' '(' (
		paramTypes += stellatype (',' paramTypes += stellatype)*
	)? ')' '->' returnType = stellatype # TypeFun
    | '(' type_ = stellatype ')' # TypeParens;
