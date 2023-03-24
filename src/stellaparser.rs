// Generated from src/stellaParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::stellaparserlistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const Surrogate_id_SYMB_0:isize=1; 
		pub const Surrogate_id_SYMB_1:isize=2; 
		pub const Surrogate_id_SYMB_2:isize=3; 
		pub const Surrogate_id_SYMB_3:isize=4; 
		pub const Surrogate_id_SYMB_4:isize=5; 
		pub const Surrogate_id_SYMB_5:isize=6; 
		pub const Surrogate_id_SYMB_6:isize=7; 
		pub const Surrogate_id_SYMB_7:isize=8; 
		pub const Surrogate_id_SYMB_8:isize=9; 
		pub const Surrogate_id_SYMB_9:isize=10; 
		pub const Surrogate_id_SYMB_10:isize=11; 
		pub const Surrogate_id_SYMB_11:isize=12; 
		pub const Surrogate_id_SYMB_12:isize=13; 
		pub const Surrogate_id_SYMB_13:isize=14; 
		pub const Surrogate_id_SYMB_14:isize=15; 
		pub const Surrogate_id_SYMB_15:isize=16; 
		pub const Surrogate_id_SYMB_16:isize=17; 
		pub const Surrogate_id_SYMB_17:isize=18; 
		pub const Surrogate_id_SYMB_18:isize=19; 
		pub const Surrogate_id_SYMB_19:isize=20; 
		pub const Surrogate_id_SYMB_20:isize=21; 
		pub const Surrogate_id_SYMB_21:isize=22; 
		pub const Surrogate_id_SYMB_22:isize=23; 
		pub const Surrogate_id_SYMB_23:isize=24; 
		pub const Surrogate_id_SYMB_24:isize=25; 
		pub const Surrogate_id_SYMB_25:isize=26; 
		pub const Surrogate_id_SYMB_26:isize=27; 
		pub const Surrogate_id_SYMB_27:isize=28; 
		pub const Surrogate_id_SYMB_28:isize=29; 
		pub const Surrogate_id_SYMB_29:isize=30; 
		pub const Surrogate_id_SYMB_30:isize=31; 
		pub const Surrogate_id_SYMB_31:isize=32; 
		pub const Surrogate_id_SYMB_32:isize=33; 
		pub const Surrogate_id_SYMB_33:isize=34; 
		pub const Surrogate_id_SYMB_34:isize=35; 
		pub const Surrogate_id_SYMB_35:isize=36; 
		pub const Surrogate_id_SYMB_36:isize=37; 
		pub const Surrogate_id_SYMB_37:isize=38; 
		pub const Surrogate_id_SYMB_38:isize=39; 
		pub const Surrogate_id_SYMB_39:isize=40; 
		pub const Surrogate_id_SYMB_40:isize=41; 
		pub const Surrogate_id_SYMB_41:isize=42; 
		pub const Surrogate_id_SYMB_42:isize=43; 
		pub const Surrogate_id_SYMB_43:isize=44; 
		pub const Surrogate_id_SYMB_44:isize=45; 
		pub const Surrogate_id_SYMB_45:isize=46; 
		pub const Surrogate_id_SYMB_46:isize=47; 
		pub const Surrogate_id_SYMB_47:isize=48; 
		pub const Surrogate_id_SYMB_48:isize=49; 
		pub const Surrogate_id_SYMB_49:isize=50; 
		pub const Surrogate_id_SYMB_50:isize=51; 
		pub const Surrogate_id_SYMB_51:isize=52; 
		pub const Surrogate_id_SYMB_52:isize=53; 
		pub const Surrogate_id_SYMB_53:isize=54; 
		pub const Surrogate_id_SYMB_54:isize=55; 
		pub const Surrogate_id_SYMB_55:isize=56; 
		pub const Surrogate_id_SYMB_56:isize=57; 
		pub const Surrogate_id_SYMB_57:isize=58; 
		pub const Surrogate_id_SYMB_58:isize=59; 
		pub const Surrogate_id_SYMB_59:isize=60; 
		pub const Surrogate_id_SYMB_60:isize=61; 
		pub const COMMENT_antlr_builtin:isize=62; 
		pub const MULTICOMMENT_antlr_builtin:isize=63; 
		pub const StellaIdent:isize=64; 
		pub const ExtensionName:isize=65; 
		pub const INTEGER:isize=66; 
		pub const WS:isize=67; 
		pub const ErrorToken:isize=68;
	pub const RULE_start_ListStellaIdent:usize = 0; 
	pub const RULE_start_Program:usize = 1; 
	pub const RULE_start_LanguageDecl:usize = 2; 
	pub const RULE_start_Extension:usize = 3; 
	pub const RULE_start_ListExtensionName:usize = 4; 
	pub const RULE_start_ListExtension:usize = 5; 
	pub const RULE_start_Decl:usize = 6; 
	pub const RULE_start_ListDecl:usize = 7; 
	pub const RULE_start_LocalDecl:usize = 8; 
	pub const RULE_start_ListLocalDecl:usize = 9; 
	pub const RULE_start_Annotation:usize = 10; 
	pub const RULE_start_ListAnnotation:usize = 11; 
	pub const RULE_start_ParamDecl:usize = 12; 
	pub const RULE_start_ListParamDecl:usize = 13; 
	pub const RULE_start_ReturnType:usize = 14; 
	pub const RULE_start_ThrowType:usize = 15; 
	pub const RULE_start_Expr:usize = 16; 
	pub const RULE_start_ListExpr:usize = 17; 
	pub const RULE_start_MatchCase:usize = 18; 
	pub const RULE_start_ListMatchCase:usize = 19; 
	pub const RULE_start_OptionalTyping:usize = 20; 
	pub const RULE_start_PatternData:usize = 21; 
	pub const RULE_start_ExprData:usize = 22; 
	pub const RULE_start_Pattern:usize = 23; 
	pub const RULE_start_ListPattern:usize = 24; 
	pub const RULE_start_LabelledPattern:usize = 25; 
	pub const RULE_start_ListLabelledPattern:usize = 26; 
	pub const RULE_start_Binding:usize = 27; 
	pub const RULE_start_ListBinding:usize = 28; 
	pub const RULE_start_Expr1:usize = 29; 
	pub const RULE_start_Expr2:usize = 30; 
	pub const RULE_start_Expr3:usize = 31; 
	pub const RULE_start_Expr4:usize = 32; 
	pub const RULE_start_Expr5:usize = 33; 
	pub const RULE_start_Expr6:usize = 34; 
	pub const RULE_start_Type:usize = 35; 
	pub const RULE_start_Type1:usize = 36; 
	pub const RULE_start_Type2:usize = 37; 
	pub const RULE_start_Type3:usize = 38; 
	pub const RULE_start_ListType:usize = 39; 
	pub const RULE_start_VariantFieldType:usize = 40; 
	pub const RULE_start_ListVariantFieldType:usize = 41; 
	pub const RULE_start_RecordFieldType:usize = 42; 
	pub const RULE_start_ListRecordFieldType:usize = 43; 
	pub const RULE_start_Typing:usize = 44; 
	pub const RULE_listStellaIdent:usize = 45; 
	pub const RULE_program:usize = 46; 
	pub const RULE_languageDecl:usize = 47; 
	pub const RULE_extension:usize = 48; 
	pub const RULE_listExtensionName:usize = 49; 
	pub const RULE_listExtension:usize = 50; 
	pub const RULE_decl:usize = 51; 
	pub const RULE_listDecl:usize = 52; 
	pub const RULE_localDecl:usize = 53; 
	pub const RULE_listLocalDecl:usize = 54; 
	pub const RULE_annotation:usize = 55; 
	pub const RULE_listAnnotation:usize = 56; 
	pub const RULE_paramDecl:usize = 57; 
	pub const RULE_listParamDecl:usize = 58; 
	pub const RULE_returnType:usize = 59; 
	pub const RULE_throwType:usize = 60; 
	pub const RULE_expr:usize = 61; 
	pub const RULE_listExpr:usize = 62; 
	pub const RULE_matchCase:usize = 63; 
	pub const RULE_listMatchCase:usize = 64; 
	pub const RULE_optionalTyping:usize = 65; 
	pub const RULE_patternData:usize = 66; 
	pub const RULE_exprData:usize = 67; 
	pub const RULE_pattern:usize = 68; 
	pub const RULE_listPattern:usize = 69; 
	pub const RULE_labelledPattern:usize = 70; 
	pub const RULE_listLabelledPattern:usize = 71; 
	pub const RULE_binding:usize = 72; 
	pub const RULE_listBinding:usize = 73; 
	pub const RULE_expr1:usize = 74; 
	pub const RULE_expr2:usize = 75; 
	pub const RULE_expr3:usize = 76; 
	pub const RULE_expr4:usize = 77; 
	pub const RULE_expr5:usize = 78; 
	pub const RULE_expr6:usize = 79; 
	pub const RULE_stellatype:usize = 80; 
	pub const RULE_type1:usize = 81; 
	pub const RULE_type2:usize = 82; 
	pub const RULE_type3:usize = 83; 
	pub const RULE_listType:usize = 84; 
	pub const RULE_variantFieldType:usize = 85; 
	pub const RULE_listVariantFieldType:usize = 86; 
	pub const RULE_recordFieldType:usize = 87; 
	pub const RULE_listRecordFieldType:usize = 88; 
	pub const RULE_typing:usize = 89;
	pub const ruleNames: [&'static str; 90] =  [
		"start_ListStellaIdent", "start_Program", "start_LanguageDecl", "start_Extension", 
		"start_ListExtensionName", "start_ListExtension", "start_Decl", "start_ListDecl", 
		"start_LocalDecl", "start_ListLocalDecl", "start_Annotation", "start_ListAnnotation", 
		"start_ParamDecl", "start_ListParamDecl", "start_ReturnType", "start_ThrowType", 
		"start_Expr", "start_ListExpr", "start_MatchCase", "start_ListMatchCase", 
		"start_OptionalTyping", "start_PatternData", "start_ExprData", "start_Pattern", 
		"start_ListPattern", "start_LabelledPattern", "start_ListLabelledPattern", 
		"start_Binding", "start_ListBinding", "start_Expr1", "start_Expr2", "start_Expr3", 
		"start_Expr4", "start_Expr5", "start_Expr6", "start_Type", "start_Type1", 
		"start_Type2", "start_Type3", "start_ListType", "start_VariantFieldType", 
		"start_ListVariantFieldType", "start_RecordFieldType", "start_ListRecordFieldType", 
		"start_Typing", "listStellaIdent", "program", "languageDecl", "extension", 
		"listExtensionName", "listExtension", "decl", "listDecl", "localDecl", 
		"listLocalDecl", "annotation", "listAnnotation", "paramDecl", "listParamDecl", 
		"returnType", "throwType", "expr", "listExpr", "matchCase", "listMatchCase", 
		"optionalTyping", "patternData", "exprData", "pattern", "listPattern", 
		"labelledPattern", "listLabelledPattern", "binding", "listBinding", "expr1", 
		"expr2", "expr3", "expr4", "expr5", "expr6", "stellatype", "type1", "type2", 
		"type3", "listType", "variantFieldType", "listVariantFieldType", "recordFieldType", 
		"listRecordFieldType", "typing"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;62] = [
		None, Some("','"), Some("';'"), Some("'('"), Some("')'"), Some("'{'"), 
		Some("'}'"), Some("'='"), Some("':'"), Some("'->'"), Some("'=>'"), Some("'<|'"), 
		Some("'|>'"), Some("'['"), Some("']'"), Some("'<'"), Some("'<='"), Some("'>'"), 
		Some("'>='"), Some("'=='"), Some("'!='"), Some("'+'"), Some("'*'"), Some("'List::head'"), 
		Some("'List::isempty'"), Some("'List::tail'"), Some("'Nat::pred'"), Some("'Nat::iszero'"), 
		Some("'Nat::rec'"), Some("'.'"), Some("'Bool'"), Some("'Nat'"), Some("'Unit'"), 
		Some("'and'"), Some("'as'"), Some("'cons'"), Some("'core'"), Some("'else'"), 
		Some("'extend'"), Some("'false'"), Some("'fix'"), Some("'fn'"), Some("'fold'"), 
		Some("'if'"), Some("'in'"), Some("'inline'"), Some("'language'"), Some("'let'"), 
		Some("'match'"), Some("'not'"), Some("'or'"), Some("'record'"), Some("'return'"), 
		Some("'succ'"), Some("'then'"), Some("'throws'"), Some("'true'"), Some("'type'"), 
		Some("'unfold'"), Some("'variant'"), Some("'with'"), Some("'\u{00B5}'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;69]  = [
		None, Some("Surrogate_id_SYMB_0"), Some("Surrogate_id_SYMB_1"), Some("Surrogate_id_SYMB_2"), 
		Some("Surrogate_id_SYMB_3"), Some("Surrogate_id_SYMB_4"), Some("Surrogate_id_SYMB_5"), 
		Some("Surrogate_id_SYMB_6"), Some("Surrogate_id_SYMB_7"), Some("Surrogate_id_SYMB_8"), 
		Some("Surrogate_id_SYMB_9"), Some("Surrogate_id_SYMB_10"), Some("Surrogate_id_SYMB_11"), 
		Some("Surrogate_id_SYMB_12"), Some("Surrogate_id_SYMB_13"), Some("Surrogate_id_SYMB_14"), 
		Some("Surrogate_id_SYMB_15"), Some("Surrogate_id_SYMB_16"), Some("Surrogate_id_SYMB_17"), 
		Some("Surrogate_id_SYMB_18"), Some("Surrogate_id_SYMB_19"), Some("Surrogate_id_SYMB_20"), 
		Some("Surrogate_id_SYMB_21"), Some("Surrogate_id_SYMB_22"), Some("Surrogate_id_SYMB_23"), 
		Some("Surrogate_id_SYMB_24"), Some("Surrogate_id_SYMB_25"), Some("Surrogate_id_SYMB_26"), 
		Some("Surrogate_id_SYMB_27"), Some("Surrogate_id_SYMB_28"), Some("Surrogate_id_SYMB_29"), 
		Some("Surrogate_id_SYMB_30"), Some("Surrogate_id_SYMB_31"), Some("Surrogate_id_SYMB_32"), 
		Some("Surrogate_id_SYMB_33"), Some("Surrogate_id_SYMB_34"), Some("Surrogate_id_SYMB_35"), 
		Some("Surrogate_id_SYMB_36"), Some("Surrogate_id_SYMB_37"), Some("Surrogate_id_SYMB_38"), 
		Some("Surrogate_id_SYMB_39"), Some("Surrogate_id_SYMB_40"), Some("Surrogate_id_SYMB_41"), 
		Some("Surrogate_id_SYMB_42"), Some("Surrogate_id_SYMB_43"), Some("Surrogate_id_SYMB_44"), 
		Some("Surrogate_id_SYMB_45"), Some("Surrogate_id_SYMB_46"), Some("Surrogate_id_SYMB_47"), 
		Some("Surrogate_id_SYMB_48"), Some("Surrogate_id_SYMB_49"), Some("Surrogate_id_SYMB_50"), 
		Some("Surrogate_id_SYMB_51"), Some("Surrogate_id_SYMB_52"), Some("Surrogate_id_SYMB_53"), 
		Some("Surrogate_id_SYMB_54"), Some("Surrogate_id_SYMB_55"), Some("Surrogate_id_SYMB_56"), 
		Some("Surrogate_id_SYMB_57"), Some("Surrogate_id_SYMB_58"), Some("Surrogate_id_SYMB_59"), 
		Some("Surrogate_id_SYMB_60"), Some("COMMENT_antlr_builtin"), Some("MULTICOMMENT_antlr_builtin"), 
		Some("StellaIdent"), Some("ExtensionName"), Some("INTEGER"), Some("WS"), 
		Some("ErrorToken")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,stellaParserExt<'input>, I, stellaParserContextType , dyn stellaParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type stellaParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, stellaParserContextType , dyn stellaParserListener<'input> + 'a>;

/// Parser for stellaParser grammar
pub struct stellaParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				stellaParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> stellaParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> stellaParser<'input, I, DefaultErrorStrategy<'input,stellaParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for stellaParser
pub trait stellaParserContext<'input>:
	for<'x> Listenable<dyn stellaParserListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=stellaParserContextType>
{}

antlr_rust::coerce_from!{ 'input : stellaParserContext<'input> }

impl<'input> stellaParserContext<'input> for TerminalNode<'input,stellaParserContextType> {}
impl<'input> stellaParserContext<'input> for ErrorNode<'input,stellaParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn stellaParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn stellaParserListener<'input> + 'input }

pub struct stellaParserContextType;
antlr_rust::tid!{stellaParserContextType}

impl<'input> ParserNodeType<'input> for stellaParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn stellaParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct stellaParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> stellaParserExt<'input>{
}
antlr_rust::tid! { stellaParserExt<'a> }

impl<'input> TokenAware<'input> for stellaParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for stellaParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for stellaParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "stellaParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn stellaParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					50 => stellaParser::<'input,I,_>::listExtension_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					52 => stellaParser::<'input,I,_>::listDecl_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					54 => stellaParser::<'input,I,_>::listLocalDecl_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					56 => stellaParser::<'input,I,_>::listAnnotation_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					75 => stellaParser::<'input,I,_>::expr2_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					76 => stellaParser::<'input,I,_>::expr3_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					77 => stellaParser::<'input,I,_>::expr4_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					79 => stellaParser::<'input,I,_>::expr6_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> stellaParser<'input, I, DefaultErrorStrategy<'input,stellaParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn listExtension_sempred(_localctx: Option<&ListExtensionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn listDecl_sempred(_localctx: Option<&ListDeclContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn listLocalDecl_sempred(_localctx: Option<&ListLocalDeclContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn listAnnotation_sempred(_localctx: Option<&ListAnnotationContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn expr2_sempred(_localctx: Option<&Expr2Context<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				4=>{
					recog.precpred(None, 10)
				}
				5=>{
					recog.precpred(None, 3)
				}
				6=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn expr3_sempred(_localctx: Option<&Expr3Context<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				7=>{
					recog.precpred(None, 3)
				}
				8=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn expr4_sempred(_localctx: Option<&Expr4Context<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				9=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn expr6_sempred(_localctx: Option<&Expr6Context<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				10=>{
					recog.precpred(None, 7)
				}
				11=>{
					recog.precpred(None, 6)
				}
			_ => true
		}
	}
}
//------------------- start_ListStellaIdent ----------------
pub type Start_ListStellaIdentContextAll<'input> = Start_ListStellaIdentContext<'input>;


pub type Start_ListStellaIdentContext<'input> = BaseParserRuleContext<'input,Start_ListStellaIdentContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListStellaIdentContextExt<'input>{
	pub x: Option<Rc<ListStellaIdentContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListStellaIdentContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListStellaIdentContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListStellaIdent(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListStellaIdent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListStellaIdentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListStellaIdent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListStellaIdent }
}
antlr_rust::tid!{Start_ListStellaIdentContextExt<'a>}

impl<'input> Start_ListStellaIdentContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListStellaIdentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListStellaIdentContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListStellaIdentContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListStellaIdentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listStellaIdent(&self) -> Option<Rc<ListStellaIdentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListStellaIdentContextAttrs<'input> for Start_ListStellaIdentContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListStellaIdent(&mut self,)
	-> Result<Rc<Start_ListStellaIdentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListStellaIdentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_start_ListStellaIdent);
        let mut _localctx: Rc<Start_ListStellaIdentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listStellaIdent*/
			recog.base.set_state(180);
			let tmp = recog.listStellaIdent()?;
			 cast_mut::<_,Start_ListStellaIdentContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(181);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Program ----------------
pub type Start_ProgramContextAll<'input> = Start_ProgramContext<'input>;


pub type Start_ProgramContext<'input> = BaseParserRuleContext<'input,Start_ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ProgramContextExt<'input>{
	pub x: Option<Rc<ProgramContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Program(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Program }
}
antlr_rust::tid!{Start_ProgramContextExt<'a>}

impl<'input> Start_ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ProgramContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ProgramContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ProgramContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn program(&self) -> Option<Rc<ProgramContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ProgramContextAttrs<'input> for Start_ProgramContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Program(&mut self,)
	-> Result<Rc<Start_ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_start_Program);
        let mut _localctx: Rc<Start_ProgramContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule program*/
			recog.base.set_state(183);
			let tmp = recog.program()?;
			 cast_mut::<_,Start_ProgramContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(184);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_LanguageDecl ----------------
pub type Start_LanguageDeclContextAll<'input> = Start_LanguageDeclContext<'input>;


pub type Start_LanguageDeclContext<'input> = BaseParserRuleContext<'input,Start_LanguageDeclContextExt<'input>>;

#[derive(Clone)]
pub struct Start_LanguageDeclContextExt<'input>{
	pub x: Option<Rc<LanguageDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_LanguageDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_LanguageDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_LanguageDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_LanguageDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_LanguageDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_LanguageDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_LanguageDecl }
}
antlr_rust::tid!{Start_LanguageDeclContextExt<'a>}

impl<'input> Start_LanguageDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_LanguageDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_LanguageDeclContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_LanguageDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_LanguageDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn languageDecl(&self) -> Option<Rc<LanguageDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_LanguageDeclContextAttrs<'input> for Start_LanguageDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_LanguageDecl(&mut self,)
	-> Result<Rc<Start_LanguageDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_LanguageDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_start_LanguageDecl);
        let mut _localctx: Rc<Start_LanguageDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule languageDecl*/
			recog.base.set_state(186);
			let tmp = recog.languageDecl()?;
			 cast_mut::<_,Start_LanguageDeclContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(187);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Extension ----------------
pub type Start_ExtensionContextAll<'input> = Start_ExtensionContext<'input>;


pub type Start_ExtensionContext<'input> = BaseParserRuleContext<'input,Start_ExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ExtensionContextExt<'input>{
	pub x: Option<Rc<ExtensionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Extension(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Extension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Extension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Extension }
}
antlr_rust::tid!{Start_ExtensionContextExt<'a>}

impl<'input> Start_ExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ExtensionContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ExtensionContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn extension(&self) -> Option<Rc<ExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ExtensionContextAttrs<'input> for Start_ExtensionContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Extension(&mut self,)
	-> Result<Rc<Start_ExtensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ExtensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_start_Extension);
        let mut _localctx: Rc<Start_ExtensionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule extension*/
			recog.base.set_state(189);
			let tmp = recog.extension()?;
			 cast_mut::<_,Start_ExtensionContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(190);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListExtensionName ----------------
pub type Start_ListExtensionNameContextAll<'input> = Start_ListExtensionNameContext<'input>;


pub type Start_ListExtensionNameContext<'input> = BaseParserRuleContext<'input,Start_ListExtensionNameContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListExtensionNameContextExt<'input>{
	pub x: Option<Rc<ListExtensionNameContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListExtensionNameContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListExtensionNameContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListExtensionName(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListExtensionName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListExtensionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListExtensionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListExtensionName }
}
antlr_rust::tid!{Start_ListExtensionNameContextExt<'a>}

impl<'input> Start_ListExtensionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListExtensionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListExtensionNameContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListExtensionNameContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListExtensionNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listExtensionName(&self) -> Option<Rc<ListExtensionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListExtensionNameContextAttrs<'input> for Start_ListExtensionNameContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListExtensionName(&mut self,)
	-> Result<Rc<Start_ListExtensionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListExtensionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_start_ListExtensionName);
        let mut _localctx: Rc<Start_ListExtensionNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listExtensionName*/
			recog.base.set_state(192);
			let tmp = recog.listExtensionName()?;
			 cast_mut::<_,Start_ListExtensionNameContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(193);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListExtension ----------------
pub type Start_ListExtensionContextAll<'input> = Start_ListExtensionContext<'input>;


pub type Start_ListExtensionContext<'input> = BaseParserRuleContext<'input,Start_ListExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListExtensionContextExt<'input>{
	pub x: Option<Rc<ListExtensionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListExtension(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListExtension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListExtension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListExtension }
}
antlr_rust::tid!{Start_ListExtensionContextExt<'a>}

impl<'input> Start_ListExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListExtensionContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListExtensionContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listExtension(&self) -> Option<Rc<ListExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListExtensionContextAttrs<'input> for Start_ListExtensionContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListExtension(&mut self,)
	-> Result<Rc<Start_ListExtensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListExtensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_start_ListExtension);
        let mut _localctx: Rc<Start_ListExtensionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listExtension*/
			recog.base.set_state(195);
			let tmp = recog.listExtension_rec(0)?;
			 cast_mut::<_,Start_ListExtensionContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(196);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Decl ----------------
pub type Start_DeclContextAll<'input> = Start_DeclContext<'input>;


pub type Start_DeclContext<'input> = BaseParserRuleContext<'input,Start_DeclContextExt<'input>>;

#[derive(Clone)]
pub struct Start_DeclContextExt<'input>{
	pub x: Option<Rc<DeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_DeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_DeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Decl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Decl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_DeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Decl }
}
antlr_rust::tid!{Start_DeclContextExt<'a>}

impl<'input> Start_DeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_DeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_DeclContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_DeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_DeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn decl(&self) -> Option<Rc<DeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_DeclContextAttrs<'input> for Start_DeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Decl(&mut self,)
	-> Result<Rc<Start_DeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_DeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_start_Decl);
        let mut _localctx: Rc<Start_DeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule decl*/
			recog.base.set_state(198);
			let tmp = recog.decl()?;
			 cast_mut::<_,Start_DeclContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(199);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListDecl ----------------
pub type Start_ListDeclContextAll<'input> = Start_ListDeclContext<'input>;


pub type Start_ListDeclContext<'input> = BaseParserRuleContext<'input,Start_ListDeclContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListDeclContextExt<'input>{
	pub x: Option<Rc<ListDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListDecl }
}
antlr_rust::tid!{Start_ListDeclContextExt<'a>}

impl<'input> Start_ListDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListDeclContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listDecl(&self) -> Option<Rc<ListDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListDeclContextAttrs<'input> for Start_ListDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListDecl(&mut self,)
	-> Result<Rc<Start_ListDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_start_ListDecl);
        let mut _localctx: Rc<Start_ListDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listDecl*/
			recog.base.set_state(201);
			let tmp = recog.listDecl_rec(0)?;
			 cast_mut::<_,Start_ListDeclContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(202);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_LocalDecl ----------------
pub type Start_LocalDeclContextAll<'input> = Start_LocalDeclContext<'input>;


pub type Start_LocalDeclContext<'input> = BaseParserRuleContext<'input,Start_LocalDeclContextExt<'input>>;

#[derive(Clone)]
pub struct Start_LocalDeclContextExt<'input>{
	pub x: Option<Rc<LocalDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_LocalDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_LocalDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_LocalDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_LocalDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_LocalDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_LocalDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_LocalDecl }
}
antlr_rust::tid!{Start_LocalDeclContextExt<'a>}

impl<'input> Start_LocalDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_LocalDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_LocalDeclContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_LocalDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_LocalDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn localDecl(&self) -> Option<Rc<LocalDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_LocalDeclContextAttrs<'input> for Start_LocalDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_LocalDecl(&mut self,)
	-> Result<Rc<Start_LocalDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_LocalDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_start_LocalDecl);
        let mut _localctx: Rc<Start_LocalDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule localDecl*/
			recog.base.set_state(204);
			let tmp = recog.localDecl()?;
			 cast_mut::<_,Start_LocalDeclContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(205);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListLocalDecl ----------------
pub type Start_ListLocalDeclContextAll<'input> = Start_ListLocalDeclContext<'input>;


pub type Start_ListLocalDeclContext<'input> = BaseParserRuleContext<'input,Start_ListLocalDeclContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListLocalDeclContextExt<'input>{
	pub x: Option<Rc<ListLocalDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListLocalDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListLocalDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListLocalDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListLocalDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListLocalDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListLocalDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListLocalDecl }
}
antlr_rust::tid!{Start_ListLocalDeclContextExt<'a>}

impl<'input> Start_ListLocalDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListLocalDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListLocalDeclContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListLocalDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListLocalDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listLocalDecl(&self) -> Option<Rc<ListLocalDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListLocalDeclContextAttrs<'input> for Start_ListLocalDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListLocalDecl(&mut self,)
	-> Result<Rc<Start_ListLocalDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListLocalDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_start_ListLocalDecl);
        let mut _localctx: Rc<Start_ListLocalDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listLocalDecl*/
			recog.base.set_state(207);
			let tmp = recog.listLocalDecl_rec(0)?;
			 cast_mut::<_,Start_ListLocalDeclContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(208);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Annotation ----------------
pub type Start_AnnotationContextAll<'input> = Start_AnnotationContext<'input>;


pub type Start_AnnotationContext<'input> = BaseParserRuleContext<'input,Start_AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct Start_AnnotationContextExt<'input>{
	pub x: Option<Rc<AnnotationContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_AnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_AnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Annotation(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Annotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_AnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Annotation }
}
antlr_rust::tid!{Start_AnnotationContextExt<'a>}

impl<'input> Start_AnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_AnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_AnnotationContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_AnnotationContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_AnnotationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn annotation(&self) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_AnnotationContextAttrs<'input> for Start_AnnotationContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Annotation(&mut self,)
	-> Result<Rc<Start_AnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_AnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_start_Annotation);
        let mut _localctx: Rc<Start_AnnotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule annotation*/
			recog.base.set_state(210);
			let tmp = recog.annotation()?;
			 cast_mut::<_,Start_AnnotationContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(211);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListAnnotation ----------------
pub type Start_ListAnnotationContextAll<'input> = Start_ListAnnotationContext<'input>;


pub type Start_ListAnnotationContext<'input> = BaseParserRuleContext<'input,Start_ListAnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListAnnotationContextExt<'input>{
	pub x: Option<Rc<ListAnnotationContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListAnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListAnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListAnnotation(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListAnnotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListAnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListAnnotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListAnnotation }
}
antlr_rust::tid!{Start_ListAnnotationContextExt<'a>}

impl<'input> Start_ListAnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListAnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListAnnotationContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListAnnotationContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListAnnotationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listAnnotation(&self) -> Option<Rc<ListAnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListAnnotationContextAttrs<'input> for Start_ListAnnotationContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListAnnotation(&mut self,)
	-> Result<Rc<Start_ListAnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListAnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_start_ListAnnotation);
        let mut _localctx: Rc<Start_ListAnnotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listAnnotation*/
			recog.base.set_state(213);
			let tmp = recog.listAnnotation_rec(0)?;
			 cast_mut::<_,Start_ListAnnotationContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(214);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ParamDecl ----------------
pub type Start_ParamDeclContextAll<'input> = Start_ParamDeclContext<'input>;


pub type Start_ParamDeclContext<'input> = BaseParserRuleContext<'input,Start_ParamDeclContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ParamDeclContextExt<'input>{
	pub x: Option<Rc<ParamDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ParamDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ParamDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ParamDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ParamDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ParamDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ParamDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ParamDecl }
}
antlr_rust::tid!{Start_ParamDeclContextExt<'a>}

impl<'input> Start_ParamDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ParamDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ParamDeclContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ParamDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ParamDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn paramDecl(&self) -> Option<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ParamDeclContextAttrs<'input> for Start_ParamDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ParamDecl(&mut self,)
	-> Result<Rc<Start_ParamDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ParamDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_start_ParamDecl);
        let mut _localctx: Rc<Start_ParamDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule paramDecl*/
			recog.base.set_state(216);
			let tmp = recog.paramDecl()?;
			 cast_mut::<_,Start_ParamDeclContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(217);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListParamDecl ----------------
pub type Start_ListParamDeclContextAll<'input> = Start_ListParamDeclContext<'input>;


pub type Start_ListParamDeclContext<'input> = BaseParserRuleContext<'input,Start_ListParamDeclContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListParamDeclContextExt<'input>{
	pub x: Option<Rc<ListParamDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListParamDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListParamDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListParamDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListParamDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListParamDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListParamDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListParamDecl }
}
antlr_rust::tid!{Start_ListParamDeclContextExt<'a>}

impl<'input> Start_ListParamDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListParamDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListParamDeclContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListParamDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListParamDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listParamDecl(&self) -> Option<Rc<ListParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListParamDeclContextAttrs<'input> for Start_ListParamDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListParamDecl(&mut self,)
	-> Result<Rc<Start_ListParamDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListParamDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_start_ListParamDecl);
        let mut _localctx: Rc<Start_ListParamDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listParamDecl*/
			recog.base.set_state(219);
			let tmp = recog.listParamDecl()?;
			 cast_mut::<_,Start_ListParamDeclContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(220);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ReturnType ----------------
pub type Start_ReturnTypeContextAll<'input> = Start_ReturnTypeContext<'input>;


pub type Start_ReturnTypeContext<'input> = BaseParserRuleContext<'input,Start_ReturnTypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ReturnTypeContextExt<'input>{
	pub x: Option<Rc<ReturnTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ReturnTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ReturnTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ReturnType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ReturnType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ReturnTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ReturnType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ReturnType }
}
antlr_rust::tid!{Start_ReturnTypeContextExt<'a>}

impl<'input> Start_ReturnTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ReturnTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ReturnTypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ReturnTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ReturnTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn returnType(&self) -> Option<Rc<ReturnTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ReturnTypeContextAttrs<'input> for Start_ReturnTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ReturnType(&mut self,)
	-> Result<Rc<Start_ReturnTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ReturnTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_start_ReturnType);
        let mut _localctx: Rc<Start_ReturnTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule returnType*/
			recog.base.set_state(222);
			let tmp = recog.returnType()?;
			 cast_mut::<_,Start_ReturnTypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(223);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ThrowType ----------------
pub type Start_ThrowTypeContextAll<'input> = Start_ThrowTypeContext<'input>;


pub type Start_ThrowTypeContext<'input> = BaseParserRuleContext<'input,Start_ThrowTypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ThrowTypeContextExt<'input>{
	pub x: Option<Rc<ThrowTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ThrowTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ThrowTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ThrowType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ThrowType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ThrowTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ThrowType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ThrowType }
}
antlr_rust::tid!{Start_ThrowTypeContextExt<'a>}

impl<'input> Start_ThrowTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ThrowTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ThrowTypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ThrowTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ThrowTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn throwType(&self) -> Option<Rc<ThrowTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ThrowTypeContextAttrs<'input> for Start_ThrowTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ThrowType(&mut self,)
	-> Result<Rc<Start_ThrowTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ThrowTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_start_ThrowType);
        let mut _localctx: Rc<Start_ThrowTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule throwType*/
			recog.base.set_state(225);
			let tmp = recog.throwType()?;
			 cast_mut::<_,Start_ThrowTypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(226);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Expr ----------------
pub type Start_ExprContextAll<'input> = Start_ExprContext<'input>;


pub type Start_ExprContext<'input> = BaseParserRuleContext<'input,Start_ExprContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ExprContextExt<'input>{
	pub x: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ExprContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Expr(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Expr }
}
antlr_rust::tid!{Start_ExprContextExt<'a>}

impl<'input> Start_ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ExprContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ExprContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ExprContextAttrs<'input> for Start_ExprContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Expr(&mut self,)
	-> Result<Rc<Start_ExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_start_Expr);
        let mut _localctx: Rc<Start_ExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr*/
			recog.base.set_state(228);
			let tmp = recog.expr()?;
			 cast_mut::<_,Start_ExprContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(229);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListExpr ----------------
pub type Start_ListExprContextAll<'input> = Start_ListExprContext<'input>;


pub type Start_ListExprContext<'input> = BaseParserRuleContext<'input,Start_ListExprContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListExprContextExt<'input>{
	pub x: Option<Rc<ListExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListExprContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListExprContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListExpr(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListExpr }
}
antlr_rust::tid!{Start_ListExprContextExt<'a>}

impl<'input> Start_ListExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListExprContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListExprContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listExpr(&self) -> Option<Rc<ListExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListExprContextAttrs<'input> for Start_ListExprContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListExpr(&mut self,)
	-> Result<Rc<Start_ListExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_start_ListExpr);
        let mut _localctx: Rc<Start_ListExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listExpr*/
			recog.base.set_state(231);
			let tmp = recog.listExpr()?;
			 cast_mut::<_,Start_ListExprContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(232);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_MatchCase ----------------
pub type Start_MatchCaseContextAll<'input> = Start_MatchCaseContext<'input>;


pub type Start_MatchCaseContext<'input> = BaseParserRuleContext<'input,Start_MatchCaseContextExt<'input>>;

#[derive(Clone)]
pub struct Start_MatchCaseContextExt<'input>{
	pub x: Option<Rc<MatchCaseContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_MatchCaseContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_MatchCaseContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_MatchCase(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_MatchCase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_MatchCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_MatchCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_MatchCase }
}
antlr_rust::tid!{Start_MatchCaseContextExt<'a>}

impl<'input> Start_MatchCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_MatchCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_MatchCaseContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_MatchCaseContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_MatchCaseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn matchCase(&self) -> Option<Rc<MatchCaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_MatchCaseContextAttrs<'input> for Start_MatchCaseContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_MatchCase(&mut self,)
	-> Result<Rc<Start_MatchCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_MatchCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_start_MatchCase);
        let mut _localctx: Rc<Start_MatchCaseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule matchCase*/
			recog.base.set_state(234);
			let tmp = recog.matchCase()?;
			 cast_mut::<_,Start_MatchCaseContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(235);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListMatchCase ----------------
pub type Start_ListMatchCaseContextAll<'input> = Start_ListMatchCaseContext<'input>;


pub type Start_ListMatchCaseContext<'input> = BaseParserRuleContext<'input,Start_ListMatchCaseContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListMatchCaseContextExt<'input>{
	pub x: Option<Rc<ListMatchCaseContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListMatchCaseContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListMatchCaseContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListMatchCase(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListMatchCase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListMatchCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListMatchCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListMatchCase }
}
antlr_rust::tid!{Start_ListMatchCaseContextExt<'a>}

impl<'input> Start_ListMatchCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListMatchCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListMatchCaseContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListMatchCaseContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListMatchCaseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listMatchCase(&self) -> Option<Rc<ListMatchCaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListMatchCaseContextAttrs<'input> for Start_ListMatchCaseContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListMatchCase(&mut self,)
	-> Result<Rc<Start_ListMatchCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListMatchCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_start_ListMatchCase);
        let mut _localctx: Rc<Start_ListMatchCaseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listMatchCase*/
			recog.base.set_state(237);
			let tmp = recog.listMatchCase()?;
			 cast_mut::<_,Start_ListMatchCaseContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(238);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_OptionalTyping ----------------
pub type Start_OptionalTypingContextAll<'input> = Start_OptionalTypingContext<'input>;


pub type Start_OptionalTypingContext<'input> = BaseParserRuleContext<'input,Start_OptionalTypingContextExt<'input>>;

#[derive(Clone)]
pub struct Start_OptionalTypingContextExt<'input>{
	pub x: Option<Rc<OptionalTypingContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_OptionalTypingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_OptionalTypingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_OptionalTyping(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_OptionalTyping(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_OptionalTypingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_OptionalTyping }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_OptionalTyping }
}
antlr_rust::tid!{Start_OptionalTypingContextExt<'a>}

impl<'input> Start_OptionalTypingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_OptionalTypingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_OptionalTypingContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_OptionalTypingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_OptionalTypingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn optionalTyping(&self) -> Option<Rc<OptionalTypingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_OptionalTypingContextAttrs<'input> for Start_OptionalTypingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_OptionalTyping(&mut self,)
	-> Result<Rc<Start_OptionalTypingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_OptionalTypingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_start_OptionalTyping);
        let mut _localctx: Rc<Start_OptionalTypingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule optionalTyping*/
			recog.base.set_state(240);
			let tmp = recog.optionalTyping()?;
			 cast_mut::<_,Start_OptionalTypingContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(241);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_PatternData ----------------
pub type Start_PatternDataContextAll<'input> = Start_PatternDataContext<'input>;


pub type Start_PatternDataContext<'input> = BaseParserRuleContext<'input,Start_PatternDataContextExt<'input>>;

#[derive(Clone)]
pub struct Start_PatternDataContextExt<'input>{
	pub x: Option<Rc<PatternDataContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_PatternDataContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_PatternDataContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_PatternData(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_PatternData(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_PatternDataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_PatternData }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_PatternData }
}
antlr_rust::tid!{Start_PatternDataContextExt<'a>}

impl<'input> Start_PatternDataContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_PatternDataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_PatternDataContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_PatternDataContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_PatternDataContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn patternData(&self) -> Option<Rc<PatternDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_PatternDataContextAttrs<'input> for Start_PatternDataContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_PatternData(&mut self,)
	-> Result<Rc<Start_PatternDataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_PatternDataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_start_PatternData);
        let mut _localctx: Rc<Start_PatternDataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule patternData*/
			recog.base.set_state(243);
			let tmp = recog.patternData()?;
			 cast_mut::<_,Start_PatternDataContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(244);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ExprData ----------------
pub type Start_ExprDataContextAll<'input> = Start_ExprDataContext<'input>;


pub type Start_ExprDataContext<'input> = BaseParserRuleContext<'input,Start_ExprDataContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ExprDataContextExt<'input>{
	pub x: Option<Rc<ExprDataContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ExprDataContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ExprDataContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ExprData(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ExprData(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ExprDataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ExprData }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ExprData }
}
antlr_rust::tid!{Start_ExprDataContextExt<'a>}

impl<'input> Start_ExprDataContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ExprDataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ExprDataContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ExprDataContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ExprDataContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn exprData(&self) -> Option<Rc<ExprDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ExprDataContextAttrs<'input> for Start_ExprDataContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ExprData(&mut self,)
	-> Result<Rc<Start_ExprDataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ExprDataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_start_ExprData);
        let mut _localctx: Rc<Start_ExprDataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule exprData*/
			recog.base.set_state(246);
			let tmp = recog.exprData()?;
			 cast_mut::<_,Start_ExprDataContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(247);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Pattern ----------------
pub type Start_PatternContextAll<'input> = Start_PatternContext<'input>;


pub type Start_PatternContext<'input> = BaseParserRuleContext<'input,Start_PatternContextExt<'input>>;

#[derive(Clone)]
pub struct Start_PatternContextExt<'input>{
	pub x: Option<Rc<PatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_PatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_PatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Pattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Pattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Pattern }
}
antlr_rust::tid!{Start_PatternContextExt<'a>}

impl<'input> Start_PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_PatternContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_PatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_PatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_PatternContextAttrs<'input> for Start_PatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Pattern(&mut self,)
	-> Result<Rc<Start_PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_start_Pattern);
        let mut _localctx: Rc<Start_PatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule pattern*/
			recog.base.set_state(249);
			let tmp = recog.pattern()?;
			 cast_mut::<_,Start_PatternContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(250);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListPattern ----------------
pub type Start_ListPatternContextAll<'input> = Start_ListPatternContext<'input>;


pub type Start_ListPatternContext<'input> = BaseParserRuleContext<'input,Start_ListPatternContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListPatternContextExt<'input>{
	pub x: Option<Rc<ListPatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListPatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListPattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListPattern }
}
antlr_rust::tid!{Start_ListPatternContextExt<'a>}

impl<'input> Start_ListPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListPatternContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListPatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListPatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listPattern(&self) -> Option<Rc<ListPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListPatternContextAttrs<'input> for Start_ListPatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListPattern(&mut self,)
	-> Result<Rc<Start_ListPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_start_ListPattern);
        let mut _localctx: Rc<Start_ListPatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listPattern*/
			recog.base.set_state(252);
			let tmp = recog.listPattern()?;
			 cast_mut::<_,Start_ListPatternContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(253);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_LabelledPattern ----------------
pub type Start_LabelledPatternContextAll<'input> = Start_LabelledPatternContext<'input>;


pub type Start_LabelledPatternContext<'input> = BaseParserRuleContext<'input,Start_LabelledPatternContextExt<'input>>;

#[derive(Clone)]
pub struct Start_LabelledPatternContextExt<'input>{
	pub x: Option<Rc<LabelledPatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_LabelledPatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_LabelledPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_LabelledPattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_LabelledPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_LabelledPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_LabelledPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_LabelledPattern }
}
antlr_rust::tid!{Start_LabelledPatternContextExt<'a>}

impl<'input> Start_LabelledPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_LabelledPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_LabelledPatternContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_LabelledPatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_LabelledPatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn labelledPattern(&self) -> Option<Rc<LabelledPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_LabelledPatternContextAttrs<'input> for Start_LabelledPatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_LabelledPattern(&mut self,)
	-> Result<Rc<Start_LabelledPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_LabelledPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_start_LabelledPattern);
        let mut _localctx: Rc<Start_LabelledPatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule labelledPattern*/
			recog.base.set_state(255);
			let tmp = recog.labelledPattern()?;
			 cast_mut::<_,Start_LabelledPatternContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(256);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListLabelledPattern ----------------
pub type Start_ListLabelledPatternContextAll<'input> = Start_ListLabelledPatternContext<'input>;


pub type Start_ListLabelledPatternContext<'input> = BaseParserRuleContext<'input,Start_ListLabelledPatternContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListLabelledPatternContextExt<'input>{
	pub x: Option<Rc<ListLabelledPatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListLabelledPatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListLabelledPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListLabelledPattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListLabelledPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListLabelledPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListLabelledPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListLabelledPattern }
}
antlr_rust::tid!{Start_ListLabelledPatternContextExt<'a>}

impl<'input> Start_ListLabelledPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListLabelledPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListLabelledPatternContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListLabelledPatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListLabelledPatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listLabelledPattern(&self) -> Option<Rc<ListLabelledPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListLabelledPatternContextAttrs<'input> for Start_ListLabelledPatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListLabelledPattern(&mut self,)
	-> Result<Rc<Start_ListLabelledPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListLabelledPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_start_ListLabelledPattern);
        let mut _localctx: Rc<Start_ListLabelledPatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listLabelledPattern*/
			recog.base.set_state(258);
			let tmp = recog.listLabelledPattern()?;
			 cast_mut::<_,Start_ListLabelledPatternContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(259);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Binding ----------------
pub type Start_BindingContextAll<'input> = Start_BindingContext<'input>;


pub type Start_BindingContext<'input> = BaseParserRuleContext<'input,Start_BindingContextExt<'input>>;

#[derive(Clone)]
pub struct Start_BindingContextExt<'input>{
	pub x: Option<Rc<BindingContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_BindingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_BindingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Binding(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Binding(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_BindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Binding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Binding }
}
antlr_rust::tid!{Start_BindingContextExt<'a>}

impl<'input> Start_BindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_BindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_BindingContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_BindingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_BindingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn binding(&self) -> Option<Rc<BindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_BindingContextAttrs<'input> for Start_BindingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Binding(&mut self,)
	-> Result<Rc<Start_BindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_BindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_start_Binding);
        let mut _localctx: Rc<Start_BindingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule binding*/
			recog.base.set_state(261);
			let tmp = recog.binding()?;
			 cast_mut::<_,Start_BindingContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(262);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListBinding ----------------
pub type Start_ListBindingContextAll<'input> = Start_ListBindingContext<'input>;


pub type Start_ListBindingContext<'input> = BaseParserRuleContext<'input,Start_ListBindingContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListBindingContextExt<'input>{
	pub x: Option<Rc<ListBindingContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListBindingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListBindingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListBinding(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListBinding(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListBindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListBinding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListBinding }
}
antlr_rust::tid!{Start_ListBindingContextExt<'a>}

impl<'input> Start_ListBindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListBindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListBindingContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListBindingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListBindingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listBinding(&self) -> Option<Rc<ListBindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListBindingContextAttrs<'input> for Start_ListBindingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListBinding(&mut self,)
	-> Result<Rc<Start_ListBindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListBindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_start_ListBinding);
        let mut _localctx: Rc<Start_ListBindingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listBinding*/
			recog.base.set_state(264);
			let tmp = recog.listBinding()?;
			 cast_mut::<_,Start_ListBindingContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(265);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Expr1 ----------------
pub type Start_Expr1ContextAll<'input> = Start_Expr1Context<'input>;


pub type Start_Expr1Context<'input> = BaseParserRuleContext<'input,Start_Expr1ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Expr1ContextExt<'input>{
	pub x: Option<Rc<Expr1ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Expr1Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Expr1Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Expr1(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Expr1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Expr1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Expr1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Expr1 }
}
antlr_rust::tid!{Start_Expr1ContextExt<'a>}

impl<'input> Start_Expr1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Expr1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Expr1ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Expr1ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Expr1ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn expr1(&self) -> Option<Rc<Expr1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Expr1ContextAttrs<'input> for Start_Expr1Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Expr1(&mut self,)
	-> Result<Rc<Start_Expr1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Expr1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_start_Expr1);
        let mut _localctx: Rc<Start_Expr1ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr1*/
			recog.base.set_state(267);
			let tmp = recog.expr1()?;
			 cast_mut::<_,Start_Expr1Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(268);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Expr2 ----------------
pub type Start_Expr2ContextAll<'input> = Start_Expr2Context<'input>;


pub type Start_Expr2Context<'input> = BaseParserRuleContext<'input,Start_Expr2ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Expr2ContextExt<'input>{
	pub x: Option<Rc<Expr2ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Expr2Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Expr2Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Expr2(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Expr2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Expr2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Expr2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Expr2 }
}
antlr_rust::tid!{Start_Expr2ContextExt<'a>}

impl<'input> Start_Expr2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Expr2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Expr2ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Expr2ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Expr2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn expr2(&self) -> Option<Rc<Expr2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Expr2ContextAttrs<'input> for Start_Expr2Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Expr2(&mut self,)
	-> Result<Rc<Start_Expr2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Expr2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_start_Expr2);
        let mut _localctx: Rc<Start_Expr2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr2*/
			recog.base.set_state(270);
			let tmp = recog.expr2_rec(0)?;
			 cast_mut::<_,Start_Expr2Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(271);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Expr3 ----------------
pub type Start_Expr3ContextAll<'input> = Start_Expr3Context<'input>;


pub type Start_Expr3Context<'input> = BaseParserRuleContext<'input,Start_Expr3ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Expr3ContextExt<'input>{
	pub x: Option<Rc<Expr3ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Expr3Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Expr3Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Expr3(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Expr3(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Expr3ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Expr3 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Expr3 }
}
antlr_rust::tid!{Start_Expr3ContextExt<'a>}

impl<'input> Start_Expr3ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Expr3ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Expr3ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Expr3ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Expr3ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn expr3(&self) -> Option<Rc<Expr3ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Expr3ContextAttrs<'input> for Start_Expr3Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Expr3(&mut self,)
	-> Result<Rc<Start_Expr3ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Expr3ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_start_Expr3);
        let mut _localctx: Rc<Start_Expr3ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr3*/
			recog.base.set_state(273);
			let tmp = recog.expr3_rec(0)?;
			 cast_mut::<_,Start_Expr3Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(274);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Expr4 ----------------
pub type Start_Expr4ContextAll<'input> = Start_Expr4Context<'input>;


pub type Start_Expr4Context<'input> = BaseParserRuleContext<'input,Start_Expr4ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Expr4ContextExt<'input>{
	pub x: Option<Rc<Expr4ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Expr4Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Expr4Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Expr4(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Expr4(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Expr4ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Expr4 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Expr4 }
}
antlr_rust::tid!{Start_Expr4ContextExt<'a>}

impl<'input> Start_Expr4ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Expr4ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Expr4ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Expr4ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Expr4ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn expr4(&self) -> Option<Rc<Expr4ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Expr4ContextAttrs<'input> for Start_Expr4Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Expr4(&mut self,)
	-> Result<Rc<Start_Expr4ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Expr4ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_start_Expr4);
        let mut _localctx: Rc<Start_Expr4ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr4*/
			recog.base.set_state(276);
			let tmp = recog.expr4_rec(0)?;
			 cast_mut::<_,Start_Expr4Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(277);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Expr5 ----------------
pub type Start_Expr5ContextAll<'input> = Start_Expr5Context<'input>;


pub type Start_Expr5Context<'input> = BaseParserRuleContext<'input,Start_Expr5ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Expr5ContextExt<'input>{
	pub x: Option<Rc<Expr5ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Expr5Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Expr5Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Expr5(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Expr5(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Expr5ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Expr5 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Expr5 }
}
antlr_rust::tid!{Start_Expr5ContextExt<'a>}

impl<'input> Start_Expr5ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Expr5ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Expr5ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Expr5ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Expr5ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn expr5(&self) -> Option<Rc<Expr5ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Expr5ContextAttrs<'input> for Start_Expr5Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Expr5(&mut self,)
	-> Result<Rc<Start_Expr5ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Expr5ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_start_Expr5);
        let mut _localctx: Rc<Start_Expr5ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr5*/
			recog.base.set_state(279);
			let tmp = recog.expr5()?;
			 cast_mut::<_,Start_Expr5Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(280);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Expr6 ----------------
pub type Start_Expr6ContextAll<'input> = Start_Expr6Context<'input>;


pub type Start_Expr6Context<'input> = BaseParserRuleContext<'input,Start_Expr6ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Expr6ContextExt<'input>{
	pub x: Option<Rc<Expr6ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Expr6Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Expr6Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Expr6(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Expr6(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Expr6ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Expr6 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Expr6 }
}
antlr_rust::tid!{Start_Expr6ContextExt<'a>}

impl<'input> Start_Expr6ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Expr6ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Expr6ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Expr6ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Expr6ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn expr6(&self) -> Option<Rc<Expr6ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Expr6ContextAttrs<'input> for Start_Expr6Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Expr6(&mut self,)
	-> Result<Rc<Start_Expr6ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Expr6ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_start_Expr6);
        let mut _localctx: Rc<Start_Expr6ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr6*/
			recog.base.set_state(282);
			let tmp = recog.expr6_rec(0)?;
			 cast_mut::<_,Start_Expr6Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(283);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Type ----------------
pub type Start_TypeContextAll<'input> = Start_TypeContext<'input>;


pub type Start_TypeContext<'input> = BaseParserRuleContext<'input,Start_TypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_TypeContextExt<'input>{
	pub x: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_TypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_TypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Type(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Type(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_TypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Type }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Type }
}
antlr_rust::tid!{Start_TypeContextExt<'a>}

impl<'input> Start_TypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_TypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_TypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_TypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_TypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_TypeContextAttrs<'input> for Start_TypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Type(&mut self,)
	-> Result<Rc<Start_TypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_TypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_start_Type);
        let mut _localctx: Rc<Start_TypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule stellatype*/
			recog.base.set_state(285);
			let tmp = recog.stellatype()?;
			 cast_mut::<_,Start_TypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(286);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Type1 ----------------
pub type Start_Type1ContextAll<'input> = Start_Type1Context<'input>;


pub type Start_Type1Context<'input> = BaseParserRuleContext<'input,Start_Type1ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Type1ContextExt<'input>{
	pub x: Option<Rc<Type1ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Type1Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Type1Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Type1(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Type1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Type1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Type1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Type1 }
}
antlr_rust::tid!{Start_Type1ContextExt<'a>}

impl<'input> Start_Type1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Type1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Type1ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Type1ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Type1ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn type1(&self) -> Option<Rc<Type1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Type1ContextAttrs<'input> for Start_Type1Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Type1(&mut self,)
	-> Result<Rc<Start_Type1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Type1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_start_Type1);
        let mut _localctx: Rc<Start_Type1ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type1*/
			recog.base.set_state(288);
			let tmp = recog.type1()?;
			 cast_mut::<_,Start_Type1Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(289);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Type2 ----------------
pub type Start_Type2ContextAll<'input> = Start_Type2Context<'input>;


pub type Start_Type2Context<'input> = BaseParserRuleContext<'input,Start_Type2ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Type2ContextExt<'input>{
	pub x: Option<Rc<Type2ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Type2Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Type2Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Type2(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Type2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Type2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Type2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Type2 }
}
antlr_rust::tid!{Start_Type2ContextExt<'a>}

impl<'input> Start_Type2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Type2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Type2ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Type2ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Type2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn type2(&self) -> Option<Rc<Type2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Type2ContextAttrs<'input> for Start_Type2Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Type2(&mut self,)
	-> Result<Rc<Start_Type2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Type2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_start_Type2);
        let mut _localctx: Rc<Start_Type2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type2*/
			recog.base.set_state(291);
			let tmp = recog.type2()?;
			 cast_mut::<_,Start_Type2Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(292);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Type3 ----------------
pub type Start_Type3ContextAll<'input> = Start_Type3Context<'input>;


pub type Start_Type3Context<'input> = BaseParserRuleContext<'input,Start_Type3ContextExt<'input>>;

#[derive(Clone)]
pub struct Start_Type3ContextExt<'input>{
	pub x: Option<Rc<Type3ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_Type3Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_Type3Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Type3(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Type3(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_Type3ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Type3 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Type3 }
}
antlr_rust::tid!{Start_Type3ContextExt<'a>}

impl<'input> Start_Type3ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_Type3ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_Type3ContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_Type3ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_Type3ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn type3(&self) -> Option<Rc<Type3ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_Type3ContextAttrs<'input> for Start_Type3Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Type3(&mut self,)
	-> Result<Rc<Start_Type3ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_Type3ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_start_Type3);
        let mut _localctx: Rc<Start_Type3ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule type3*/
			recog.base.set_state(294);
			let tmp = recog.type3()?;
			 cast_mut::<_,Start_Type3Context >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(295);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListType ----------------
pub type Start_ListTypeContextAll<'input> = Start_ListTypeContext<'input>;


pub type Start_ListTypeContext<'input> = BaseParserRuleContext<'input,Start_ListTypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListTypeContextExt<'input>{
	pub x: Option<Rc<ListTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListType }
}
antlr_rust::tid!{Start_ListTypeContextExt<'a>}

impl<'input> Start_ListTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListTypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listType(&self) -> Option<Rc<ListTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListTypeContextAttrs<'input> for Start_ListTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListType(&mut self,)
	-> Result<Rc<Start_ListTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_start_ListType);
        let mut _localctx: Rc<Start_ListTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listType*/
			recog.base.set_state(297);
			let tmp = recog.listType()?;
			 cast_mut::<_,Start_ListTypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(298);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_VariantFieldType ----------------
pub type Start_VariantFieldTypeContextAll<'input> = Start_VariantFieldTypeContext<'input>;


pub type Start_VariantFieldTypeContext<'input> = BaseParserRuleContext<'input,Start_VariantFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_VariantFieldTypeContextExt<'input>{
	pub x: Option<Rc<VariantFieldTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_VariantFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_VariantFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_VariantFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_VariantFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_VariantFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_VariantFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_VariantFieldType }
}
antlr_rust::tid!{Start_VariantFieldTypeContextExt<'a>}

impl<'input> Start_VariantFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_VariantFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_VariantFieldTypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_VariantFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_VariantFieldTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn variantFieldType(&self) -> Option<Rc<VariantFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_VariantFieldTypeContextAttrs<'input> for Start_VariantFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_VariantFieldType(&mut self,)
	-> Result<Rc<Start_VariantFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_VariantFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_start_VariantFieldType);
        let mut _localctx: Rc<Start_VariantFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule variantFieldType*/
			recog.base.set_state(300);
			let tmp = recog.variantFieldType()?;
			 cast_mut::<_,Start_VariantFieldTypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(301);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListVariantFieldType ----------------
pub type Start_ListVariantFieldTypeContextAll<'input> = Start_ListVariantFieldTypeContext<'input>;


pub type Start_ListVariantFieldTypeContext<'input> = BaseParserRuleContext<'input,Start_ListVariantFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListVariantFieldTypeContextExt<'input>{
	pub x: Option<Rc<ListVariantFieldTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListVariantFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListVariantFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListVariantFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListVariantFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListVariantFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListVariantFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListVariantFieldType }
}
antlr_rust::tid!{Start_ListVariantFieldTypeContextExt<'a>}

impl<'input> Start_ListVariantFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListVariantFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListVariantFieldTypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListVariantFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListVariantFieldTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listVariantFieldType(&self) -> Option<Rc<ListVariantFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListVariantFieldTypeContextAttrs<'input> for Start_ListVariantFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListVariantFieldType(&mut self,)
	-> Result<Rc<Start_ListVariantFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListVariantFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_start_ListVariantFieldType);
        let mut _localctx: Rc<Start_ListVariantFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listVariantFieldType*/
			recog.base.set_state(303);
			let tmp = recog.listVariantFieldType()?;
			 cast_mut::<_,Start_ListVariantFieldTypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(304);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_RecordFieldType ----------------
pub type Start_RecordFieldTypeContextAll<'input> = Start_RecordFieldTypeContext<'input>;


pub type Start_RecordFieldTypeContext<'input> = BaseParserRuleContext<'input,Start_RecordFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_RecordFieldTypeContextExt<'input>{
	pub x: Option<Rc<RecordFieldTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_RecordFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_RecordFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_RecordFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_RecordFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_RecordFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_RecordFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_RecordFieldType }
}
antlr_rust::tid!{Start_RecordFieldTypeContextExt<'a>}

impl<'input> Start_RecordFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_RecordFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_RecordFieldTypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_RecordFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_RecordFieldTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn recordFieldType(&self) -> Option<Rc<RecordFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_RecordFieldTypeContextAttrs<'input> for Start_RecordFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_RecordFieldType(&mut self,)
	-> Result<Rc<Start_RecordFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_RecordFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_start_RecordFieldType);
        let mut _localctx: Rc<Start_RecordFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule recordFieldType*/
			recog.base.set_state(306);
			let tmp = recog.recordFieldType()?;
			 cast_mut::<_,Start_RecordFieldTypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(307);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_ListRecordFieldType ----------------
pub type Start_ListRecordFieldTypeContextAll<'input> = Start_ListRecordFieldTypeContext<'input>;


pub type Start_ListRecordFieldTypeContext<'input> = BaseParserRuleContext<'input,Start_ListRecordFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct Start_ListRecordFieldTypeContextExt<'input>{
	pub x: Option<Rc<ListRecordFieldTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_ListRecordFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_ListRecordFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_ListRecordFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_ListRecordFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_ListRecordFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_ListRecordFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_ListRecordFieldType }
}
antlr_rust::tid!{Start_ListRecordFieldTypeContextExt<'a>}

impl<'input> Start_ListRecordFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_ListRecordFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_ListRecordFieldTypeContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_ListRecordFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_ListRecordFieldTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn listRecordFieldType(&self) -> Option<Rc<ListRecordFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_ListRecordFieldTypeContextAttrs<'input> for Start_ListRecordFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_ListRecordFieldType(&mut self,)
	-> Result<Rc<Start_ListRecordFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_ListRecordFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_start_ListRecordFieldType);
        let mut _localctx: Rc<Start_ListRecordFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule listRecordFieldType*/
			recog.base.set_state(309);
			let tmp = recog.listRecordFieldType()?;
			 cast_mut::<_,Start_ListRecordFieldTypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(310);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- start_Typing ----------------
pub type Start_TypingContextAll<'input> = Start_TypingContext<'input>;


pub type Start_TypingContext<'input> = BaseParserRuleContext<'input,Start_TypingContextExt<'input>>;

#[derive(Clone)]
pub struct Start_TypingContextExt<'input>{
	pub x: Option<Rc<TypingContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Start_TypingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Start_TypingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_start_Typing(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_start_Typing(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Start_TypingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_start_Typing }
	//fn type_rule_index() -> usize where Self: Sized { RULE_start_Typing }
}
antlr_rust::tid!{Start_TypingContextExt<'a>}

impl<'input> Start_TypingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Start_TypingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Start_TypingContextExt{
				x: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Start_TypingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Start_TypingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn typing(&self) -> Option<Rc<TypingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Start_TypingContextAttrs<'input> for Start_TypingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn start_Typing(&mut self,)
	-> Result<Rc<Start_TypingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Start_TypingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_start_Typing);
        let mut _localctx: Rc<Start_TypingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typing*/
			recog.base.set_state(312);
			let tmp = recog.typing()?;
			 cast_mut::<_,Start_TypingContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(313);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listStellaIdent ----------------
pub type ListStellaIdentContextAll<'input> = ListStellaIdentContext<'input>;


pub type ListStellaIdentContext<'input> = BaseParserRuleContext<'input,ListStellaIdentContextExt<'input>>;

#[derive(Clone)]
pub struct ListStellaIdentContextExt<'input>{
	pub p_2_1: Option<TokenType<'input>>,
	pub p_3_1: Option<TokenType<'input>>,
	pub p_3_3: Option<Rc<ListStellaIdentContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListStellaIdentContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListStellaIdentContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listStellaIdent(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listStellaIdent(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListStellaIdentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listStellaIdent }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listStellaIdent }
}
antlr_rust::tid!{ListStellaIdentContextExt<'a>}

impl<'input> ListStellaIdentContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListStellaIdentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListStellaIdentContextExt{
				p_2_1: None, p_3_1: None, 
				p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListStellaIdentContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListStellaIdentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listStellaIdent(&self) -> Option<Rc<ListStellaIdentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListStellaIdentContextAttrs<'input> for ListStellaIdentContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listStellaIdent(&mut self,)
	-> Result<Rc<ListStellaIdentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListStellaIdentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_listStellaIdent);
        let mut _localctx: Rc<ListStellaIdentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(320);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(316);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,ListStellaIdentContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(317);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,ListStellaIdentContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(318);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listStellaIdent*/
					recog.base.set_state(319);
					let tmp = recog.listStellaIdent()?;
					 cast_mut::<_,ListStellaIdentContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
	pub p_1_1: Option<Rc<LanguageDeclContextAll<'input>>>,
	pub p_1_2: Option<Rc<ListExtensionContextAll<'input>>>,
	pub p_1_3: Option<Rc<ListDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ProgramContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ProgramContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_program(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_program(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProgramContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_program }
	//fn type_rule_index() -> usize where Self: Sized { RULE_program }
}
antlr_rust::tid!{ProgramContextExt<'a>}

impl<'input> ProgramContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProgramContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProgramContextExt{
				p_1_1: None, p_1_2: None, p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

fn languageDecl(&self) -> Option<Rc<LanguageDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listExtension(&self) -> Option<Rc<ListExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listDecl(&self) -> Option<Rc<ListDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ProgramContextAttrs<'input> for ProgramContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn program(&mut self,)
	-> Result<Rc<ProgramContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProgramContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule languageDecl*/
			recog.base.set_state(322);
			let tmp = recog.languageDecl()?;
			 cast_mut::<_,ProgramContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			/*InvokeRule listExtension*/
			recog.base.set_state(323);
			let tmp = recog.listExtension_rec(0)?;
			 cast_mut::<_,ProgramContext >(&mut _localctx).p_1_2 = Some(tmp.clone());
			  

			/*InvokeRule listDecl*/
			recog.base.set_state(324);
			let tmp = recog.listDecl_rec(0)?;
			 cast_mut::<_,ProgramContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- languageDecl ----------------
pub type LanguageDeclContextAll<'input> = LanguageDeclContext<'input>;


pub type LanguageDeclContext<'input> = BaseParserRuleContext<'input,LanguageDeclContextExt<'input>>;

#[derive(Clone)]
pub struct LanguageDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for LanguageDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for LanguageDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_languageDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_languageDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LanguageDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_languageDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_languageDecl }
}
antlr_rust::tid!{LanguageDeclContextExt<'a>}

impl<'input> LanguageDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LanguageDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LanguageDeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LanguageDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<LanguageDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_45
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_45
fn Surrogate_id_SYMB_45(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_45, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_35
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_35
fn Surrogate_id_SYMB_35(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_35, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_1
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_1
fn Surrogate_id_SYMB_1(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_1, 0)
}

}

impl<'input> LanguageDeclContextAttrs<'input> for LanguageDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn languageDecl(&mut self,)
	-> Result<Rc<LanguageDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LanguageDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_languageDecl);
        let mut _localctx: Rc<LanguageDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(326);
			recog.base.match_token(Surrogate_id_SYMB_45,&mut recog.err_handler)?;

			recog.base.set_state(327);
			recog.base.match_token(Surrogate_id_SYMB_35,&mut recog.err_handler)?;

			recog.base.set_state(328);
			recog.base.match_token(Surrogate_id_SYMB_1,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- extension ----------------
pub type ExtensionContextAll<'input> = ExtensionContext<'input>;


pub type ExtensionContext<'input> = BaseParserRuleContext<'input,ExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct ExtensionContextExt<'input>{
	pub p_1_3: Option<Rc<ListExtensionNameContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_extension(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_extension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_extension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_extension }
}
antlr_rust::tid!{ExtensionContextExt<'a>}

impl<'input> ExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExtensionContextExt{
				p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ExtensionContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_37
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_37
fn Surrogate_id_SYMB_37(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_37, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_59
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_59
fn Surrogate_id_SYMB_59(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_59, 0)
}
fn listExtensionName(&self) -> Option<Rc<ListExtensionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExtensionContextAttrs<'input> for ExtensionContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn extension(&mut self,)
	-> Result<Rc<ExtensionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExtensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_extension);
        let mut _localctx: Rc<ExtensionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(330);
			recog.base.match_token(Surrogate_id_SYMB_37,&mut recog.err_handler)?;

			recog.base.set_state(331);
			recog.base.match_token(Surrogate_id_SYMB_59,&mut recog.err_handler)?;

			/*InvokeRule listExtensionName*/
			recog.base.set_state(332);
			let tmp = recog.listExtensionName()?;
			 cast_mut::<_,ExtensionContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listExtensionName ----------------
pub type ListExtensionNameContextAll<'input> = ListExtensionNameContext<'input>;


pub type ListExtensionNameContext<'input> = BaseParserRuleContext<'input,ListExtensionNameContextExt<'input>>;

#[derive(Clone)]
pub struct ListExtensionNameContextExt<'input>{
	pub p_2_1: Option<TokenType<'input>>,
	pub p_3_1: Option<TokenType<'input>>,
	pub p_3_3: Option<Rc<ListExtensionNameContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListExtensionNameContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListExtensionNameContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listExtensionName(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listExtensionName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListExtensionNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listExtensionName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listExtensionName }
}
antlr_rust::tid!{ListExtensionNameContextExt<'a>}

impl<'input> ListExtensionNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListExtensionNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListExtensionNameContextExt{
				p_2_1: None, p_3_1: None, 
				p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListExtensionNameContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListExtensionNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ExtensionName
/// Returns `None` if there is no child corresponding to token ExtensionName
fn ExtensionName(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(ExtensionName, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listExtensionName(&self) -> Option<Rc<ListExtensionNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListExtensionNameContextAttrs<'input> for ListExtensionNameContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listExtensionName(&mut self,)
	-> Result<Rc<ListExtensionNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListExtensionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_listExtensionName);
        let mut _localctx: Rc<ListExtensionNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(339);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(335);
					let tmp = recog.base.match_token(ExtensionName,&mut recog.err_handler)?;
					 cast_mut::<_,ListExtensionNameContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(336);
					let tmp = recog.base.match_token(ExtensionName,&mut recog.err_handler)?;
					 cast_mut::<_,ListExtensionNameContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(337);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listExtensionName*/
					recog.base.set_state(338);
					let tmp = recog.listExtensionName()?;
					 cast_mut::<_,ListExtensionNameContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listExtension ----------------
pub type ListExtensionContextAll<'input> = ListExtensionContext<'input>;


pub type ListExtensionContext<'input> = BaseParserRuleContext<'input,ListExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct ListExtensionContextExt<'input>{
	pub p_2_1: Option<Rc<ListExtensionContextAll<'input>>>,
	pub p_2_2: Option<Rc<ExtensionContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListExtensionContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listExtension(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listExtension(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listExtension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listExtension }
}
antlr_rust::tid!{ListExtensionContextExt<'a>}

impl<'input> ListExtensionContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListExtensionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListExtensionContextExt{
				p_2_1: None, p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListExtensionContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListExtensionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_1
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_1
fn Surrogate_id_SYMB_1(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_1, 0)
}
fn listExtension(&self) -> Option<Rc<ListExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn extension(&self) -> Option<Rc<ExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListExtensionContextAttrs<'input> for ListExtensionContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  listExtension(&mut self,)
	-> Result<Rc<ListExtensionContextAll<'input>>,ANTLRError> {
		self.listExtension_rec(0)
	}

	fn listExtension_rec(&mut self, _p: isize)
	-> Result<Rc<ListExtensionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ListExtensionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 100, RULE_listExtension, _p);
	    let mut _localctx: Rc<ListExtensionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 100;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(348);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ListExtensionContextExt::new(_parentctx.clone(), _parentState);
					(cast_mut::<_,ListExtensionContext>(&mut tmp)).p_2_1 = Some(_prevctx.clone());
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_listExtension);
					_localctx = tmp;
					recog.base.set_state(342);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					/*InvokeRule extension*/
					recog.base.set_state(343);
					let tmp = recog.extension()?;
					 cast_mut::<_,ListExtensionContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					recog.base.set_state(344);
					recog.base.match_token(Surrogate_id_SYMB_1,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(350);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- decl ----------------
pub type DeclContextAll<'input> = DeclContext<'input>;


pub type DeclContext<'input> = BaseParserRuleContext<'input,DeclContextExt<'input>>;

#[derive(Clone)]
pub struct DeclContextExt<'input>{
	pub p_1_1: Option<Rc<ListAnnotationContextAll<'input>>>,
	pub p_1_3: Option<TokenType<'input>>,
	pub p_1_5: Option<Rc<ListParamDeclContextAll<'input>>>,
	pub p_1_7: Option<Rc<ReturnTypeContextAll<'input>>>,
	pub p_1_8: Option<Rc<ThrowTypeContextAll<'input>>>,
	pub p_1_10: Option<Rc<ListDeclContextAll<'input>>>,
	pub p_1_12: Option<Rc<ExprContextAll<'input>>>,
	pub p_2_2: Option<TokenType<'input>>,
	pub p_2_4: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for DeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for DeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_decl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_decl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decl }
}
antlr_rust::tid!{DeclContextExt<'a>}

impl<'input> DeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclContextExt{
				p_1_3: None, p_2_2: None, 
				p_1_1: None, p_1_5: None, p_1_7: None, p_1_8: None, p_1_10: None, p_1_12: None, p_2_4: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<DeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_40
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_40
fn Surrogate_id_SYMB_40(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_40, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_4
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_4
fn Surrogate_id_SYMB_4(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_4, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_51
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_51
fn Surrogate_id_SYMB_51(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_51, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_1
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_1
fn Surrogate_id_SYMB_1(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_1, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_5
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_5
fn Surrogate_id_SYMB_5(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_5, 0)
}
fn listAnnotation(&self) -> Option<Rc<ListAnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn listParamDecl(&self) -> Option<Rc<ListParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnType(&self) -> Option<Rc<ReturnTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn throwType(&self) -> Option<Rc<ThrowTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listDecl(&self) -> Option<Rc<ListDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_56
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_56
fn Surrogate_id_SYMB_56(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_56, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_6
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_6
fn Surrogate_id_SYMB_6(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_6, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclContextAttrs<'input> for DeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decl(&mut self,)
	-> Result<Rc<DeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_decl);
        let mut _localctx: Rc<DeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(370);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule listAnnotation*/
					recog.base.set_state(351);
					let tmp = recog.listAnnotation_rec(0)?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
					  

					recog.base.set_state(352);
					recog.base.match_token(Surrogate_id_SYMB_40,&mut recog.err_handler)?;

					recog.base.set_state(353);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
					  

					recog.base.set_state(354);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule listParamDecl*/
					recog.base.set_state(355);
					let tmp = recog.listParamDecl()?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_1_5 = Some(tmp.clone());
					  

					recog.base.set_state(356);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					/*InvokeRule returnType*/
					recog.base.set_state(357);
					let tmp = recog.returnType()?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_1_7 = Some(tmp.clone());
					  

					/*InvokeRule throwType*/
					recog.base.set_state(358);
					let tmp = recog.throwType()?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_1_8 = Some(tmp.clone());
					  

					recog.base.set_state(359);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listDecl*/
					recog.base.set_state(360);
					let tmp = recog.listDecl_rec(0)?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_1_10 = Some(tmp.clone());
					  

					recog.base.set_state(361);
					recog.base.match_token(Surrogate_id_SYMB_51,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(362);
					let tmp = recog.expr()?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_1_12 = Some(tmp.clone());
					  

					recog.base.set_state(363);
					recog.base.match_token(Surrogate_id_SYMB_1,&mut recog.err_handler)?;

					recog.base.set_state(364);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(366);
					recog.base.match_token(Surrogate_id_SYMB_56,&mut recog.err_handler)?;

					recog.base.set_state(367);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					recog.base.set_state(368);
					recog.base.match_token(Surrogate_id_SYMB_6,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(369);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,DeclContext >(&mut _localctx).p_2_4 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listDecl ----------------
pub type ListDeclContextAll<'input> = ListDeclContext<'input>;


pub type ListDeclContext<'input> = BaseParserRuleContext<'input,ListDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ListDeclContextExt<'input>{
	pub p_2_1: Option<Rc<ListDeclContextAll<'input>>>,
	pub p_2_2: Option<Rc<DeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listDecl }
}
antlr_rust::tid!{ListDeclContextExt<'a>}

impl<'input> ListDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListDeclContextExt{
				p_2_1: None, p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListDeclContextExt<'input>>{

fn listDecl(&self) -> Option<Rc<ListDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn decl(&self) -> Option<Rc<DeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListDeclContextAttrs<'input> for ListDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  listDecl(&mut self,)
	-> Result<Rc<ListDeclContextAll<'input>>,ANTLRError> {
		self.listDecl_rec(0)
	}

	fn listDecl_rec(&mut self, _p: isize)
	-> Result<Rc<ListDeclContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ListDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 104, RULE_listDecl, _p);
	    let mut _localctx: Rc<ListDeclContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 104;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(377);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ListDeclContextExt::new(_parentctx.clone(), _parentState);
					(cast_mut::<_,ListDeclContext>(&mut tmp)).p_2_1 = Some(_prevctx.clone());
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_listDecl);
					_localctx = tmp;
					recog.base.set_state(373);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					/*InvokeRule decl*/
					recog.base.set_state(374);
					let tmp = recog.decl()?;
					 cast_mut::<_,ListDeclContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					}
					} 
				}
				recog.base.set_state(379);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(4,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- localDecl ----------------
pub type LocalDeclContextAll<'input> = LocalDeclContext<'input>;


pub type LocalDeclContext<'input> = BaseParserRuleContext<'input,LocalDeclContextExt<'input>>;

#[derive(Clone)]
pub struct LocalDeclContextExt<'input>{
	pub p_1_1: Option<Rc<DeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for LocalDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for LocalDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_localDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_localDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LocalDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localDecl }
}
antlr_rust::tid!{LocalDeclContextExt<'a>}

impl<'input> LocalDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LocalDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LocalDeclContextExt{
				p_1_1: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait LocalDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<LocalDeclContextExt<'input>>{

fn decl(&self) -> Option<Rc<DeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LocalDeclContextAttrs<'input> for LocalDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn localDecl(&mut self,)
	-> Result<Rc<LocalDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LocalDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_localDecl);
        let mut _localctx: Rc<LocalDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule decl*/
			recog.base.set_state(380);
			let tmp = recog.decl()?;
			 cast_mut::<_,LocalDeclContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listLocalDecl ----------------
pub type ListLocalDeclContextAll<'input> = ListLocalDeclContext<'input>;


pub type ListLocalDeclContext<'input> = BaseParserRuleContext<'input,ListLocalDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ListLocalDeclContextExt<'input>{
	pub p_2_1: Option<Rc<ListLocalDeclContextAll<'input>>>,
	pub p_2_2: Option<Rc<LocalDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListLocalDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListLocalDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listLocalDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listLocalDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListLocalDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listLocalDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listLocalDecl }
}
antlr_rust::tid!{ListLocalDeclContextExt<'a>}

impl<'input> ListLocalDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListLocalDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListLocalDeclContextExt{
				p_2_1: None, p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListLocalDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListLocalDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_1
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_1
fn Surrogate_id_SYMB_1(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_1, 0)
}
fn listLocalDecl(&self) -> Option<Rc<ListLocalDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn localDecl(&self) -> Option<Rc<LocalDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListLocalDeclContextAttrs<'input> for ListLocalDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  listLocalDecl(&mut self,)
	-> Result<Rc<ListLocalDeclContextAll<'input>>,ANTLRError> {
		self.listLocalDecl_rec(0)
	}

	fn listLocalDecl_rec(&mut self, _p: isize)
	-> Result<Rc<ListLocalDeclContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ListLocalDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 108, RULE_listLocalDecl, _p);
	    let mut _localctx: Rc<ListLocalDeclContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 108;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(389);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ListLocalDeclContextExt::new(_parentctx.clone(), _parentState);
					(cast_mut::<_,ListLocalDeclContext>(&mut tmp)).p_2_1 = Some(_prevctx.clone());
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_listLocalDecl);
					_localctx = tmp;
					recog.base.set_state(383);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					/*InvokeRule localDecl*/
					recog.base.set_state(384);
					let tmp = recog.localDecl()?;
					 cast_mut::<_,ListLocalDeclContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					recog.base.set_state(385);
					recog.base.match_token(Surrogate_id_SYMB_1,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(391);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- annotation ----------------
pub type AnnotationContextAll<'input> = AnnotationContext<'input>;


pub type AnnotationContext<'input> = BaseParserRuleContext<'input,AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for AnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for AnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_annotation(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_annotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotation }
}
antlr_rust::tid!{AnnotationContextExt<'a>}

impl<'input> AnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AnnotationContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<AnnotationContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_44
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_44
fn Surrogate_id_SYMB_44(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_44, 0)
}

}

impl<'input> AnnotationContextAttrs<'input> for AnnotationContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn annotation(&mut self,)
	-> Result<Rc<AnnotationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_annotation);
        let mut _localctx: Rc<AnnotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(392);
			recog.base.match_token(Surrogate_id_SYMB_44,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listAnnotation ----------------
pub type ListAnnotationContextAll<'input> = ListAnnotationContext<'input>;


pub type ListAnnotationContext<'input> = BaseParserRuleContext<'input,ListAnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct ListAnnotationContextExt<'input>{
	pub p_2_1: Option<Rc<ListAnnotationContextAll<'input>>>,
	pub p_2_2: Option<Rc<AnnotationContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListAnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListAnnotationContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listAnnotation(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listAnnotation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListAnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listAnnotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listAnnotation }
}
antlr_rust::tid!{ListAnnotationContextExt<'a>}

impl<'input> ListAnnotationContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListAnnotationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListAnnotationContextExt{
				p_2_1: None, p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListAnnotationContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListAnnotationContextExt<'input>>{

fn listAnnotation(&self) -> Option<Rc<ListAnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn annotation(&self) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListAnnotationContextAttrs<'input> for ListAnnotationContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  listAnnotation(&mut self,)
	-> Result<Rc<ListAnnotationContextAll<'input>>,ANTLRError> {
		self.listAnnotation_rec(0)
	}

	fn listAnnotation_rec(&mut self, _p: isize)
	-> Result<Rc<ListAnnotationContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ListAnnotationContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 112, RULE_listAnnotation, _p);
	    let mut _localctx: Rc<ListAnnotationContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 112;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(399);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ListAnnotationContextExt::new(_parentctx.clone(), _parentState);
					(cast_mut::<_,ListAnnotationContext>(&mut tmp)).p_2_1 = Some(_prevctx.clone());
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_listAnnotation);
					_localctx = tmp;
					recog.base.set_state(395);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					/*InvokeRule annotation*/
					recog.base.set_state(396);
					let tmp = recog.annotation()?;
					 cast_mut::<_,ListAnnotationContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					}
					} 
				}
				recog.base.set_state(401);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- paramDecl ----------------
pub type ParamDeclContextAll<'input> = ParamDeclContext<'input>;


pub type ParamDeclContext<'input> = BaseParserRuleContext<'input,ParamDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ParamDeclContextExt<'input>{
	pub p_1_1: Option<TokenType<'input>>,
	pub p_1_3: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ParamDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ParamDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paramDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_paramDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ParamDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paramDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paramDecl }
}
antlr_rust::tid!{ParamDeclContextExt<'a>}

impl<'input> ParamDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamDeclContextExt{
				p_1_1: None, 
				p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ParamDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_7
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_7
fn Surrogate_id_SYMB_7(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_7, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamDeclContextAttrs<'input> for ParamDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paramDecl(&mut self,)
	-> Result<Rc<ParamDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_paramDecl);
        let mut _localctx: Rc<ParamDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(402);
			let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
			 cast_mut::<_,ParamDeclContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			recog.base.set_state(403);
			recog.base.match_token(Surrogate_id_SYMB_7,&mut recog.err_handler)?;

			/*InvokeRule stellatype*/
			recog.base.set_state(404);
			let tmp = recog.stellatype()?;
			 cast_mut::<_,ParamDeclContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listParamDecl ----------------
pub type ListParamDeclContextAll<'input> = ListParamDeclContext<'input>;


pub type ListParamDeclContext<'input> = BaseParserRuleContext<'input,ListParamDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ListParamDeclContextExt<'input>{
	pub p_2_1: Option<Rc<ParamDeclContextAll<'input>>>,
	pub p_3_1: Option<Rc<ParamDeclContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListParamDeclContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListParamDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListParamDeclContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listParamDecl(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listParamDecl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListParamDeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listParamDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listParamDecl }
}
antlr_rust::tid!{ListParamDeclContextExt<'a>}

impl<'input> ListParamDeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListParamDeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListParamDeclContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListParamDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListParamDeclContextExt<'input>>{

fn paramDecl(&self) -> Option<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listParamDecl(&self) -> Option<Rc<ListParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListParamDeclContextAttrs<'input> for ListParamDeclContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listParamDecl(&mut self,)
	-> Result<Rc<ListParamDeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListParamDeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_listParamDecl);
        let mut _localctx: Rc<ListParamDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(412);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule paramDecl*/
					recog.base.set_state(407);
					let tmp = recog.paramDecl()?;
					 cast_mut::<_,ListParamDeclContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule paramDecl*/
					recog.base.set_state(408);
					let tmp = recog.paramDecl()?;
					 cast_mut::<_,ListParamDeclContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(409);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listParamDecl*/
					recog.base.set_state(410);
					let tmp = recog.listParamDecl()?;
					 cast_mut::<_,ListParamDeclContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- returnType ----------------
pub type ReturnTypeContextAll<'input> = ReturnTypeContext<'input>;


pub type ReturnTypeContext<'input> = BaseParserRuleContext<'input,ReturnTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnTypeContextExt<'input>{
	pub p_2_2: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ReturnTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ReturnTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_returnType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_returnType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ReturnTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnType }
}
antlr_rust::tid!{ReturnTypeContextExt<'a>}

impl<'input> ReturnTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnTypeContextExt{
				p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ReturnTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_8
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_8
fn Surrogate_id_SYMB_8(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_8, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnTypeContextAttrs<'input> for ReturnTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnType(&mut self,)
	-> Result<Rc<ReturnTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_returnType);
        let mut _localctx: Rc<ReturnTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(417);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EOF | Surrogate_id_SYMB_4 | Surrogate_id_SYMB_54 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}

			 Surrogate_id_SYMB_8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(415);
					recog.base.match_token(Surrogate_id_SYMB_8,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(416);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,ReturnTypeContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- throwType ----------------
pub type ThrowTypeContextAll<'input> = ThrowTypeContext<'input>;


pub type ThrowTypeContext<'input> = BaseParserRuleContext<'input,ThrowTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ThrowTypeContextExt<'input>{
	pub p_2_2: Option<Rc<ListTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ThrowTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ThrowTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_throwType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_throwType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ThrowTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_throwType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_throwType }
}
antlr_rust::tid!{ThrowTypeContextExt<'a>}

impl<'input> ThrowTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ThrowTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ThrowTypeContextExt{
				p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ThrowTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ThrowTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_54
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_54
fn Surrogate_id_SYMB_54(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_54, 0)
}
fn listType(&self) -> Option<Rc<ListTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ThrowTypeContextAttrs<'input> for ThrowTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn throwType(&mut self,)
	-> Result<Rc<ThrowTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ThrowTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_throwType);
        let mut _localctx: Rc<ThrowTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(422);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EOF | Surrogate_id_SYMB_4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}

			 Surrogate_id_SYMB_54 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(420);
					recog.base.match_token(Surrogate_id_SYMB_54,&mut recog.err_handler)?;

					/*InvokeRule listType*/
					recog.base.set_state(421);
					let tmp = recog.listType()?;
					 cast_mut::<_,ThrowTypeContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
	pub p_1_2: Option<Rc<ExprContextAll<'input>>>,
	pub p_1_4: Option<Rc<ExprContextAll<'input>>>,
	pub p_1_6: Option<Rc<ExprContextAll<'input>>>,
	pub p_2_2: Option<TokenType<'input>>,
	pub p_2_4: Option<Rc<ExprContextAll<'input>>>,
	pub p_2_6: Option<Rc<ExprContextAll<'input>>>,
	pub p_3_1: Option<Rc<Expr1ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				p_2_2: None, 
				p_1_2: None, p_1_4: None, p_1_6: None, p_2_4: None, p_2_6: None, p_3_1: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_42
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_42
fn Surrogate_id_SYMB_42(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_42, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_53
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_53
fn Surrogate_id_SYMB_53(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_53, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_36
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_36
fn Surrogate_id_SYMB_36(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_36, 0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_46
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_46
fn Surrogate_id_SYMB_46(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_46, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_6
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_6
fn Surrogate_id_SYMB_6(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_6, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_43
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_43
fn Surrogate_id_SYMB_43(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_43, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn expr1(&self) -> Option<Rc<Expr1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(439);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_42 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(424);
					recog.base.match_token(Surrogate_id_SYMB_42,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(425);
					let tmp = recog.expr()?;
					 cast_mut::<_,ExprContext >(&mut _localctx).p_1_2 = Some(tmp.clone());
					  

					recog.base.set_state(426);
					recog.base.match_token(Surrogate_id_SYMB_53,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(427);
					let tmp = recog.expr()?;
					 cast_mut::<_,ExprContext >(&mut _localctx).p_1_4 = Some(tmp.clone());
					  

					recog.base.set_state(428);
					recog.base.match_token(Surrogate_id_SYMB_36,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(429);
					let tmp = recog.expr()?;
					 cast_mut::<_,ExprContext >(&mut _localctx).p_1_6 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_46 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(431);
					recog.base.match_token(Surrogate_id_SYMB_46,&mut recog.err_handler)?;

					recog.base.set_state(432);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,ExprContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					recog.base.set_state(433);
					recog.base.match_token(Surrogate_id_SYMB_6,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(434);
					let tmp = recog.expr()?;
					 cast_mut::<_,ExprContext >(&mut _localctx).p_2_4 = Some(tmp.clone());
					  

					recog.base.set_state(435);
					recog.base.match_token(Surrogate_id_SYMB_43,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(436);
					let tmp = recog.expr()?;
					 cast_mut::<_,ExprContext >(&mut _localctx).p_2_6 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_2 | Surrogate_id_SYMB_4 | Surrogate_id_SYMB_10 | Surrogate_id_SYMB_12 |
			 Surrogate_id_SYMB_22 | Surrogate_id_SYMB_23 | Surrogate_id_SYMB_24 |
			 Surrogate_id_SYMB_25 | Surrogate_id_SYMB_26 | Surrogate_id_SYMB_27 |
			 Surrogate_id_SYMB_34 | Surrogate_id_SYMB_38 | Surrogate_id_SYMB_39 |
			 Surrogate_id_SYMB_40 | Surrogate_id_SYMB_41 | Surrogate_id_SYMB_47 |
			 Surrogate_id_SYMB_48 | Surrogate_id_SYMB_50 | Surrogate_id_SYMB_52 |
			 Surrogate_id_SYMB_55 | Surrogate_id_SYMB_57 | StellaIdent | INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expr1*/
					recog.base.set_state(438);
					let tmp = recog.expr1()?;
					 cast_mut::<_,ExprContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listExpr ----------------
pub type ListExprContextAll<'input> = ListExprContext<'input>;


pub type ListExprContext<'input> = BaseParserRuleContext<'input,ListExprContextExt<'input>>;

#[derive(Clone)]
pub struct ListExprContextExt<'input>{
	pub p_2_1: Option<Rc<ExprContextAll<'input>>>,
	pub p_3_1: Option<Rc<ExprContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListExprContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListExprContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listExpr(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listExpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listExpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listExpr }
}
antlr_rust::tid!{ListExprContextExt<'a>}

impl<'input> ListExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListExprContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListExprContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListExprContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listExpr(&self) -> Option<Rc<ListExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListExprContextAttrs<'input> for ListExprContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listExpr(&mut self,)
	-> Result<Rc<ListExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_listExpr);
        let mut _localctx: Rc<ListExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(447);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expr*/
					recog.base.set_state(442);
					let tmp = recog.expr()?;
					 cast_mut::<_,ListExprContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expr*/
					recog.base.set_state(443);
					let tmp = recog.expr()?;
					 cast_mut::<_,ListExprContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(444);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listExpr*/
					recog.base.set_state(445);
					let tmp = recog.listExpr()?;
					 cast_mut::<_,ListExprContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- matchCase ----------------
pub type MatchCaseContextAll<'input> = MatchCaseContext<'input>;


pub type MatchCaseContext<'input> = BaseParserRuleContext<'input,MatchCaseContextExt<'input>>;

#[derive(Clone)]
pub struct MatchCaseContextExt<'input>{
	pub p_1_1: Option<Rc<PatternContextAll<'input>>>,
	pub p_1_3: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for MatchCaseContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for MatchCaseContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_matchCase(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_matchCase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for MatchCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_matchCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_matchCase }
}
antlr_rust::tid!{MatchCaseContextExt<'a>}

impl<'input> MatchCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MatchCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MatchCaseContextExt{
				p_1_1: None, p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MatchCaseContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<MatchCaseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_9
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_9
fn Surrogate_id_SYMB_9(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_9, 0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MatchCaseContextAttrs<'input> for MatchCaseContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn matchCase(&mut self,)
	-> Result<Rc<MatchCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MatchCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_matchCase);
        let mut _localctx: Rc<MatchCaseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule pattern*/
			recog.base.set_state(449);
			let tmp = recog.pattern()?;
			 cast_mut::<_,MatchCaseContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			recog.base.set_state(450);
			recog.base.match_token(Surrogate_id_SYMB_9,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(451);
			let tmp = recog.expr()?;
			 cast_mut::<_,MatchCaseContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listMatchCase ----------------
pub type ListMatchCaseContextAll<'input> = ListMatchCaseContext<'input>;


pub type ListMatchCaseContext<'input> = BaseParserRuleContext<'input,ListMatchCaseContextExt<'input>>;

#[derive(Clone)]
pub struct ListMatchCaseContextExt<'input>{
	pub p_2_1: Option<Rc<MatchCaseContextAll<'input>>>,
	pub p_3_1: Option<Rc<MatchCaseContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListMatchCaseContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListMatchCaseContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListMatchCaseContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listMatchCase(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listMatchCase(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListMatchCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listMatchCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listMatchCase }
}
antlr_rust::tid!{ListMatchCaseContextExt<'a>}

impl<'input> ListMatchCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListMatchCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListMatchCaseContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListMatchCaseContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListMatchCaseContextExt<'input>>{

fn matchCase(&self) -> Option<Rc<MatchCaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_1
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_1
fn Surrogate_id_SYMB_1(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_1, 0)
}
fn listMatchCase(&self) -> Option<Rc<ListMatchCaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListMatchCaseContextAttrs<'input> for ListMatchCaseContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listMatchCase(&mut self,)
	-> Result<Rc<ListMatchCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListMatchCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_listMatchCase);
        let mut _localctx: Rc<ListMatchCaseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(459);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule matchCase*/
					recog.base.set_state(454);
					let tmp = recog.matchCase()?;
					 cast_mut::<_,ListMatchCaseContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule matchCase*/
					recog.base.set_state(455);
					let tmp = recog.matchCase()?;
					 cast_mut::<_,ListMatchCaseContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(456);
					recog.base.match_token(Surrogate_id_SYMB_1,&mut recog.err_handler)?;

					/*InvokeRule listMatchCase*/
					recog.base.set_state(457);
					let tmp = recog.listMatchCase()?;
					 cast_mut::<_,ListMatchCaseContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- optionalTyping ----------------
pub type OptionalTypingContextAll<'input> = OptionalTypingContext<'input>;


pub type OptionalTypingContext<'input> = BaseParserRuleContext<'input,OptionalTypingContextExt<'input>>;

#[derive(Clone)]
pub struct OptionalTypingContextExt<'input>{
	pub p_2_2: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for OptionalTypingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for OptionalTypingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_optionalTyping(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_optionalTyping(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for OptionalTypingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_optionalTyping }
	//fn type_rule_index() -> usize where Self: Sized { RULE_optionalTyping }
}
antlr_rust::tid!{OptionalTypingContextExt<'a>}

impl<'input> OptionalTypingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OptionalTypingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OptionalTypingContextExt{
				p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait OptionalTypingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<OptionalTypingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_7
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_7
fn Surrogate_id_SYMB_7(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_7, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> OptionalTypingContextAttrs<'input> for OptionalTypingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn optionalTyping(&mut self,)
	-> Result<Rc<OptionalTypingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OptionalTypingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_optionalTyping);
        let mut _localctx: Rc<OptionalTypingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(464);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EOF | Surrogate_id_SYMB_0 | Surrogate_id_SYMB_11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}

			 Surrogate_id_SYMB_7 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(462);
					recog.base.match_token(Surrogate_id_SYMB_7,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(463);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,OptionalTypingContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- patternData ----------------
pub type PatternDataContextAll<'input> = PatternDataContext<'input>;


pub type PatternDataContext<'input> = BaseParserRuleContext<'input,PatternDataContextExt<'input>>;

#[derive(Clone)]
pub struct PatternDataContextExt<'input>{
	pub p_2_2: Option<Rc<PatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for PatternDataContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for PatternDataContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_patternData(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_patternData(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternDataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_patternData }
	//fn type_rule_index() -> usize where Self: Sized { RULE_patternData }
}
antlr_rust::tid!{PatternDataContextExt<'a>}

impl<'input> PatternDataContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternDataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternDataContextExt{
				p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternDataContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<PatternDataContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_6
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_6
fn Surrogate_id_SYMB_6(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_6, 0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PatternDataContextAttrs<'input> for PatternDataContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn patternData(&mut self,)
	-> Result<Rc<PatternDataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternDataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_patternData);
        let mut _localctx: Rc<PatternDataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(469);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EOF | Surrogate_id_SYMB_11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}

			 Surrogate_id_SYMB_6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(467);
					recog.base.match_token(Surrogate_id_SYMB_6,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(468);
					let tmp = recog.pattern()?;
					 cast_mut::<_,PatternDataContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- exprData ----------------
pub type ExprDataContextAll<'input> = ExprDataContext<'input>;


pub type ExprDataContext<'input> = BaseParserRuleContext<'input,ExprDataContextExt<'input>>;

#[derive(Clone)]
pub struct ExprDataContextExt<'input>{
	pub p_2_2: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ExprDataContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ExprDataContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exprData(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_exprData(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExprDataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exprData }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exprData }
}
antlr_rust::tid!{ExprDataContextExt<'a>}

impl<'input> ExprDataContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprDataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprDataContextExt{
				p_2_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprDataContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ExprDataContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_6
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_6
fn Surrogate_id_SYMB_6(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_6, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExprDataContextAttrs<'input> for ExprDataContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn exprData(&mut self,)
	-> Result<Rc<ExprDataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprDataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_exprData);
        let mut _localctx: Rc<ExprDataContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(474);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EOF | Surrogate_id_SYMB_11 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}

			 Surrogate_id_SYMB_6 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(472);
					recog.base.match_token(Surrogate_id_SYMB_6,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(473);
					let tmp = recog.expr()?;
					 cast_mut::<_,ExprDataContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pattern ----------------
pub type PatternContextAll<'input> = PatternContext<'input>;


pub type PatternContext<'input> = BaseParserRuleContext<'input,PatternContextExt<'input>>;

#[derive(Clone)]
pub struct PatternContextExt<'input>{
	pub p_1_2: Option<TokenType<'input>>,
	pub p_1_3: Option<Rc<PatternDataContextAll<'input>>>,
	pub p_2_2: Option<Rc<ListPatternContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListLabelledPatternContextAll<'input>>>,
	pub p_4_2: Option<Rc<ListPatternContextAll<'input>>>,
	pub p_5_3: Option<Rc<PatternContextAll<'input>>>,
	pub p_5_5: Option<Rc<PatternContextAll<'input>>>,
	pub p_8_1: Option<TokenType<'input>>,
	pub p_9_3: Option<Rc<PatternContextAll<'input>>>,
	pub p_10_1: Option<TokenType<'input>>,
	pub p_11_2: Option<Rc<PatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for PatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for PatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_pattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_pattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pattern }
}
antlr_rust::tid!{PatternContextExt<'a>}

impl<'input> PatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PatternContextExt{
				p_1_2: None, p_8_1: None, p_10_1: None, 
				p_1_3: None, p_2_2: None, p_3_3: None, p_4_2: None, p_5_3: None, p_5_5: None, p_9_3: None, p_11_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait PatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<PatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_10
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_10
fn Surrogate_id_SYMB_10(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_10, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_11
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_11
fn Surrogate_id_SYMB_11(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_11, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn patternData(&self) -> Option<Rc<PatternDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_4
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_4
fn Surrogate_id_SYMB_4(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_4, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_5
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_5
fn Surrogate_id_SYMB_5(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_5, 0)
}
fn listPattern(&self) -> Option<Rc<ListPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_50
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_50
fn Surrogate_id_SYMB_50(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_50, 0)
}
fn listLabelledPattern(&self) -> Option<Rc<ListLabelledPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_12
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_12
fn Surrogate_id_SYMB_12(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_12, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_13
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_13
fn Surrogate_id_SYMB_13(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_13, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_34
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_34
fn Surrogate_id_SYMB_34(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_34, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
fn pattern_all(&self) ->  Vec<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pattern(&self, i: usize) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_38
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_38
fn Surrogate_id_SYMB_38(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_38, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_55
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_55
fn Surrogate_id_SYMB_55(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_55, 0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_52
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_52
fn Surrogate_id_SYMB_52(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_52, 0)
}

}

impl<'input> PatternContextAttrs<'input> for PatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pattern(&mut self,)
	-> Result<Rc<PatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_pattern);
        let mut _localctx: Rc<PatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(514);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_10 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(476);
					recog.base.match_token(Surrogate_id_SYMB_10,&mut recog.err_handler)?;

					recog.base.set_state(477);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_1_2 = Some(tmp.clone());
					  

					/*InvokeRule patternData*/
					recog.base.set_state(478);
					let tmp = recog.patternData()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
					  

					recog.base.set_state(479);
					recog.base.match_token(Surrogate_id_SYMB_11,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(481);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listPattern*/
					recog.base.set_state(482);
					let tmp = recog.listPattern()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					recog.base.set_state(483);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_50 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(485);
					recog.base.match_token(Surrogate_id_SYMB_50,&mut recog.err_handler)?;

					recog.base.set_state(486);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listLabelledPattern*/
					recog.base.set_state(487);
					let tmp = recog.listLabelledPattern()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					recog.base.set_state(488);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_12 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(490);
					recog.base.match_token(Surrogate_id_SYMB_12,&mut recog.err_handler)?;

					/*InvokeRule listPattern*/
					recog.base.set_state(491);
					let tmp = recog.listPattern()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_4_2 = Some(tmp.clone());
					  

					recog.base.set_state(492);
					recog.base.match_token(Surrogate_id_SYMB_13,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_34 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(494);
					recog.base.match_token(Surrogate_id_SYMB_34,&mut recog.err_handler)?;

					recog.base.set_state(495);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(496);
					let tmp = recog.pattern()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_5_3 = Some(tmp.clone());
					  

					recog.base.set_state(497);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(498);
					let tmp = recog.pattern()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_5_5 = Some(tmp.clone());
					  

					recog.base.set_state(499);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_38 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(501);
					recog.base.match_token(Surrogate_id_SYMB_38,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_55 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(502);
					recog.base.match_token(Surrogate_id_SYMB_55,&mut recog.err_handler)?;

					}
				}

			 INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(503);
					let tmp = recog.base.match_token(INTEGER,&mut recog.err_handler)?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_8_1 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_52 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(504);
					recog.base.match_token(Surrogate_id_SYMB_52,&mut recog.err_handler)?;

					recog.base.set_state(505);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(506);
					let tmp = recog.pattern()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_9_3 = Some(tmp.clone());
					  

					recog.base.set_state(507);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 StellaIdent 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(509);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_10_1 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					recog.base.set_state(510);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule pattern*/
					recog.base.set_state(511);
					let tmp = recog.pattern()?;
					 cast_mut::<_,PatternContext >(&mut _localctx).p_11_2 = Some(tmp.clone());
					  

					recog.base.set_state(512);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listPattern ----------------
pub type ListPatternContextAll<'input> = ListPatternContext<'input>;


pub type ListPatternContext<'input> = BaseParserRuleContext<'input,ListPatternContextExt<'input>>;

#[derive(Clone)]
pub struct ListPatternContextExt<'input>{
	pub p_2_1: Option<Rc<PatternContextAll<'input>>>,
	pub p_3_1: Option<Rc<PatternContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListPatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListPatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listPattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listPattern }
}
antlr_rust::tid!{ListPatternContextExt<'a>}

impl<'input> ListPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListPatternContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListPatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListPatternContextExt<'input>>{

fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listPattern(&self) -> Option<Rc<ListPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListPatternContextAttrs<'input> for ListPatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listPattern(&mut self,)
	-> Result<Rc<ListPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_listPattern);
        let mut _localctx: Rc<ListPatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(522);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(17,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule pattern*/
					recog.base.set_state(517);
					let tmp = recog.pattern()?;
					 cast_mut::<_,ListPatternContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule pattern*/
					recog.base.set_state(518);
					let tmp = recog.pattern()?;
					 cast_mut::<_,ListPatternContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(519);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listPattern*/
					recog.base.set_state(520);
					let tmp = recog.listPattern()?;
					 cast_mut::<_,ListPatternContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- labelledPattern ----------------
pub type LabelledPatternContextAll<'input> = LabelledPatternContext<'input>;


pub type LabelledPatternContext<'input> = BaseParserRuleContext<'input,LabelledPatternContextExt<'input>>;

#[derive(Clone)]
pub struct LabelledPatternContextExt<'input>{
	pub p_1_1: Option<TokenType<'input>>,
	pub p_1_3: Option<Rc<PatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for LabelledPatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for LabelledPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_labelledPattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_labelledPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LabelledPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_labelledPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_labelledPattern }
}
antlr_rust::tid!{LabelledPatternContextExt<'a>}

impl<'input> LabelledPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LabelledPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabelledPatternContextExt{
				p_1_1: None, 
				p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait LabelledPatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<LabelledPatternContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_6
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_6
fn Surrogate_id_SYMB_6(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_6, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn pattern(&self) -> Option<Rc<PatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LabelledPatternContextAttrs<'input> for LabelledPatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn labelledPattern(&mut self,)
	-> Result<Rc<LabelledPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabelledPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 140, RULE_labelledPattern);
        let mut _localctx: Rc<LabelledPatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(524);
			let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
			 cast_mut::<_,LabelledPatternContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			recog.base.set_state(525);
			recog.base.match_token(Surrogate_id_SYMB_6,&mut recog.err_handler)?;

			/*InvokeRule pattern*/
			recog.base.set_state(526);
			let tmp = recog.pattern()?;
			 cast_mut::<_,LabelledPatternContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listLabelledPattern ----------------
pub type ListLabelledPatternContextAll<'input> = ListLabelledPatternContext<'input>;


pub type ListLabelledPatternContext<'input> = BaseParserRuleContext<'input,ListLabelledPatternContextExt<'input>>;

#[derive(Clone)]
pub struct ListLabelledPatternContextExt<'input>{
	pub p_2_1: Option<Rc<LabelledPatternContextAll<'input>>>,
	pub p_3_1: Option<Rc<LabelledPatternContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListLabelledPatternContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListLabelledPatternContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListLabelledPatternContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listLabelledPattern(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listLabelledPattern(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListLabelledPatternContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listLabelledPattern }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listLabelledPattern }
}
antlr_rust::tid!{ListLabelledPatternContextExt<'a>}

impl<'input> ListLabelledPatternContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListLabelledPatternContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListLabelledPatternContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListLabelledPatternContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListLabelledPatternContextExt<'input>>{

fn labelledPattern(&self) -> Option<Rc<LabelledPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listLabelledPattern(&self) -> Option<Rc<ListLabelledPatternContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListLabelledPatternContextAttrs<'input> for ListLabelledPatternContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listLabelledPattern(&mut self,)
	-> Result<Rc<ListLabelledPatternContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListLabelledPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_listLabelledPattern);
        let mut _localctx: Rc<ListLabelledPatternContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(534);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule labelledPattern*/
					recog.base.set_state(529);
					let tmp = recog.labelledPattern()?;
					 cast_mut::<_,ListLabelledPatternContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule labelledPattern*/
					recog.base.set_state(530);
					let tmp = recog.labelledPattern()?;
					 cast_mut::<_,ListLabelledPatternContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(531);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listLabelledPattern*/
					recog.base.set_state(532);
					let tmp = recog.listLabelledPattern()?;
					 cast_mut::<_,ListLabelledPatternContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- binding ----------------
pub type BindingContextAll<'input> = BindingContext<'input>;


pub type BindingContext<'input> = BaseParserRuleContext<'input,BindingContextExt<'input>>;

#[derive(Clone)]
pub struct BindingContextExt<'input>{
	pub p_1_1: Option<TokenType<'input>>,
	pub p_1_3: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for BindingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for BindingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_binding(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_binding(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for BindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binding }
}
antlr_rust::tid!{BindingContextExt<'a>}

impl<'input> BindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BindingContextExt{
				p_1_1: None, 
				p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait BindingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<BindingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_6
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_6
fn Surrogate_id_SYMB_6(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_6, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> BindingContextAttrs<'input> for BindingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn binding(&mut self,)
	-> Result<Rc<BindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_binding);
        let mut _localctx: Rc<BindingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(536);
			let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
			 cast_mut::<_,BindingContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			recog.base.set_state(537);
			recog.base.match_token(Surrogate_id_SYMB_6,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(538);
			let tmp = recog.expr()?;
			 cast_mut::<_,BindingContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listBinding ----------------
pub type ListBindingContextAll<'input> = ListBindingContext<'input>;


pub type ListBindingContext<'input> = BaseParserRuleContext<'input,ListBindingContextExt<'input>>;

#[derive(Clone)]
pub struct ListBindingContextExt<'input>{
	pub p_2_1: Option<Rc<BindingContextAll<'input>>>,
	pub p_3_1: Option<Rc<BindingContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListBindingContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListBindingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListBindingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listBinding(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listBinding(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListBindingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listBinding }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listBinding }
}
antlr_rust::tid!{ListBindingContextExt<'a>}

impl<'input> ListBindingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListBindingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListBindingContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListBindingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListBindingContextExt<'input>>{

fn binding(&self) -> Option<Rc<BindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listBinding(&self) -> Option<Rc<ListBindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListBindingContextAttrs<'input> for ListBindingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listBinding(&mut self,)
	-> Result<Rc<ListBindingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListBindingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_listBinding);
        let mut _localctx: Rc<ListBindingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(546);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule binding*/
					recog.base.set_state(541);
					let tmp = recog.binding()?;
					 cast_mut::<_,ListBindingContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule binding*/
					recog.base.set_state(542);
					let tmp = recog.binding()?;
					 cast_mut::<_,ListBindingContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(543);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listBinding*/
					recog.base.set_state(544);
					let tmp = recog.listBinding()?;
					 cast_mut::<_,ListBindingContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr1 ----------------
pub type Expr1ContextAll<'input> = Expr1Context<'input>;


pub type Expr1Context<'input> = BaseParserRuleContext<'input,Expr1ContextExt<'input>>;

#[derive(Clone)]
pub struct Expr1ContextExt<'input>{
	pub p_1_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_1_3: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_2_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_2_3: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_3_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_3_3: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_4_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_4_3: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_5_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_5_3: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_6_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_6_3: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_7_1: Option<Rc<Expr2ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Expr1Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Expr1Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr1(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_expr1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expr1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr1 }
}
antlr_rust::tid!{Expr1ContextExt<'a>}

impl<'input> Expr1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expr1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expr1ContextExt{
				p_1_1: None, p_1_3: None, p_2_1: None, p_2_3: None, p_3_1: None, p_3_3: None, p_4_1: None, p_4_3: None, p_5_1: None, p_5_3: None, p_6_1: None, p_6_3: None, p_7_1: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Expr1ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Expr1ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_14
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_14
fn Surrogate_id_SYMB_14(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_14, 0)
}
fn expr2_all(&self) ->  Vec<Rc<Expr2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr2(&self, i: usize) -> Option<Rc<Expr2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_15
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_15
fn Surrogate_id_SYMB_15(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_15, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_16
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_16
fn Surrogate_id_SYMB_16(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_16, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_17
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_17
fn Surrogate_id_SYMB_17(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_17, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_18
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_18
fn Surrogate_id_SYMB_18(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_18, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_19
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_19
fn Surrogate_id_SYMB_19(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_19, 0)
}

}

impl<'input> Expr1ContextAttrs<'input> for Expr1Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expr1(&mut self,)
	-> Result<Rc<Expr1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Expr1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_expr1);
        let mut _localctx: Rc<Expr1ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(573);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expr2*/
					recog.base.set_state(548);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_1_1 = Some(tmp.clone());
					  

					recog.base.set_state(549);
					recog.base.match_token(Surrogate_id_SYMB_14,&mut recog.err_handler)?;

					/*InvokeRule expr2*/
					recog.base.set_state(550);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_1_3 = Some(tmp.clone());
					  

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expr2*/
					recog.base.set_state(552);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					recog.base.set_state(553);
					recog.base.match_token(Surrogate_id_SYMB_15,&mut recog.err_handler)?;

					/*InvokeRule expr2*/
					recog.base.set_state(554);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_2_3 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expr2*/
					recog.base.set_state(556);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(557);
					recog.base.match_token(Surrogate_id_SYMB_16,&mut recog.err_handler)?;

					/*InvokeRule expr2*/
					recog.base.set_state(558);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule expr2*/
					recog.base.set_state(560);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_4_1 = Some(tmp.clone());
					  

					recog.base.set_state(561);
					recog.base.match_token(Surrogate_id_SYMB_17,&mut recog.err_handler)?;

					/*InvokeRule expr2*/
					recog.base.set_state(562);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_4_3 = Some(tmp.clone());
					  

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule expr2*/
					recog.base.set_state(564);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_5_1 = Some(tmp.clone());
					  

					recog.base.set_state(565);
					recog.base.match_token(Surrogate_id_SYMB_18,&mut recog.err_handler)?;

					/*InvokeRule expr2*/
					recog.base.set_state(566);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_5_3 = Some(tmp.clone());
					  

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule expr2*/
					recog.base.set_state(568);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_6_1 = Some(tmp.clone());
					  

					recog.base.set_state(569);
					recog.base.match_token(Surrogate_id_SYMB_19,&mut recog.err_handler)?;

					/*InvokeRule expr2*/
					recog.base.set_state(570);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_6_3 = Some(tmp.clone());
					  

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule expr2*/
					recog.base.set_state(572);
					let tmp = recog.expr2_rec(0)?;
					 cast_mut::<_,Expr1Context >(&mut _localctx).p_7_1 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr2 ----------------
pub type Expr2ContextAll<'input> = Expr2Context<'input>;


pub type Expr2Context<'input> = BaseParserRuleContext<'input,Expr2ContextExt<'input>>;

#[derive(Clone)]
pub struct Expr2ContextExt<'input>{
	pub p_1_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_8_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_9_1: Option<Rc<Expr2ContextAll<'input>>>,
	pub p_2_3: Option<Rc<ListParamDeclContextAll<'input>>>,
	pub p_2_7: Option<Rc<ExprContextAll<'input>>>,
	pub p_3_2: Option<Rc<ListExprContextAll<'input>>>,
	pub p_4_3: Option<Rc<ListBindingContextAll<'input>>>,
	pub p_5_2: Option<TokenType<'input>>,
	pub p_5_3: Option<Rc<ExprDataContextAll<'input>>>,
	pub p_6_2: Option<Rc<Expr1ContextAll<'input>>>,
	pub p_6_4: Option<Rc<ListMatchCaseContextAll<'input>>>,
	pub p_7_2: Option<Rc<ListExprContextAll<'input>>>,
	pub p_10_1: Option<Rc<Expr3ContextAll<'input>>>,
	pub p_1_3: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_8_3: Option<Rc<Expr3ContextAll<'input>>>,
	pub p_9_3: Option<Rc<Expr3ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Expr2Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Expr2Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr2(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_expr2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expr2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr2 }
}
antlr_rust::tid!{Expr2ContextExt<'a>}

impl<'input> Expr2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expr2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expr2ContextExt{
				p_5_2: None, 
				p_1_1: None, p_8_1: None, p_9_1: None, p_2_3: None, p_2_7: None, p_3_2: None, p_4_3: None, p_5_3: None, p_6_2: None, p_6_4: None, p_7_2: None, p_10_1: None, p_1_3: None, p_8_3: None, p_9_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Expr2ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Expr2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_40
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_40
fn Surrogate_id_SYMB_40(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_40, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_4
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_4
fn Surrogate_id_SYMB_4(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_4, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_51
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_51
fn Surrogate_id_SYMB_51(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_51, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_1
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_1
fn Surrogate_id_SYMB_1(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_1, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_5
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_5
fn Surrogate_id_SYMB_5(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_5, 0)
}
fn listParamDecl(&self) -> Option<Rc<ListParamDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listExpr(&self) -> Option<Rc<ListExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_50
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_50
fn Surrogate_id_SYMB_50(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_50, 0)
}
fn listBinding(&self) -> Option<Rc<ListBindingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_10
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_10
fn Surrogate_id_SYMB_10(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_10, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_11
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_11
fn Surrogate_id_SYMB_11(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_11, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn exprData(&self) -> Option<Rc<ExprDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_47
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_47
fn Surrogate_id_SYMB_47(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_47, 0)
}
fn expr1(&self) -> Option<Rc<Expr1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listMatchCase(&self) -> Option<Rc<ListMatchCaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_12
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_12
fn Surrogate_id_SYMB_12(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_12, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_13
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_13
fn Surrogate_id_SYMB_13(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_13, 0)
}
fn expr3(&self) -> Option<Rc<Expr3ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_33
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_33
fn Surrogate_id_SYMB_33(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_33, 0)
}
fn expr2(&self) -> Option<Rc<Expr2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_20
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_20
fn Surrogate_id_SYMB_20(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_20, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_49
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_49
fn Surrogate_id_SYMB_49(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_49, 0)
}

}

impl<'input> Expr2ContextAttrs<'input> for Expr2Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr2(&mut self,)
	-> Result<Rc<Expr2ContextAll<'input>>,ANTLRError> {
		self.expr2_rec(0)
	}

	fn expr2_rec(&mut self, _p: isize)
	-> Result<Rc<Expr2ContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Expr2ContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 150, RULE_expr2, _p);
	    let mut _localctx: Rc<Expr2ContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 150;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(611);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_40 
				=> {
					{
					recog.base.set_state(576);
					recog.base.match_token(Surrogate_id_SYMB_40,&mut recog.err_handler)?;

					recog.base.set_state(577);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule listParamDecl*/
					recog.base.set_state(578);
					let tmp = recog.listParamDecl()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_2_3 = Some(tmp.clone());
					  

					recog.base.set_state(579);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					recog.base.set_state(580);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					recog.base.set_state(581);
					recog.base.match_token(Surrogate_id_SYMB_51,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(582);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_2_7 = Some(tmp.clone());
					  

					recog.base.set_state(583);
					recog.base.match_token(Surrogate_id_SYMB_1,&mut recog.err_handler)?;

					recog.base.set_state(584);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_4 
				=> {
					{
					recog.base.set_state(586);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listExpr*/
					recog.base.set_state(587);
					let tmp = recog.listExpr()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_3_2 = Some(tmp.clone());
					  

					recog.base.set_state(588);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_50 
				=> {
					{
					recog.base.set_state(590);
					recog.base.match_token(Surrogate_id_SYMB_50,&mut recog.err_handler)?;

					recog.base.set_state(591);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listBinding*/
					recog.base.set_state(592);
					let tmp = recog.listBinding()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_4_3 = Some(tmp.clone());
					  

					recog.base.set_state(593);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_10 
				=> {
					{
					recog.base.set_state(595);
					recog.base.match_token(Surrogate_id_SYMB_10,&mut recog.err_handler)?;

					recog.base.set_state(596);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_5_2 = Some(tmp.clone());
					  

					/*InvokeRule exprData*/
					recog.base.set_state(597);
					let tmp = recog.exprData()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_5_3 = Some(tmp.clone());
					  

					recog.base.set_state(598);
					recog.base.match_token(Surrogate_id_SYMB_11,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_47 
				=> {
					{
					recog.base.set_state(600);
					recog.base.match_token(Surrogate_id_SYMB_47,&mut recog.err_handler)?;

					/*InvokeRule expr1*/
					recog.base.set_state(601);
					let tmp = recog.expr1()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_6_2 = Some(tmp.clone());
					  

					recog.base.set_state(602);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listMatchCase*/
					recog.base.set_state(603);
					let tmp = recog.listMatchCase()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_6_4 = Some(tmp.clone());
					  

					recog.base.set_state(604);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_12 
				=> {
					{
					recog.base.set_state(606);
					recog.base.match_token(Surrogate_id_SYMB_12,&mut recog.err_handler)?;

					/*InvokeRule listExpr*/
					recog.base.set_state(607);
					let tmp = recog.listExpr()?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_7_2 = Some(tmp.clone());
					  

					recog.base.set_state(608);
					recog.base.match_token(Surrogate_id_SYMB_13,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_2 | Surrogate_id_SYMB_22 | Surrogate_id_SYMB_23 | Surrogate_id_SYMB_24 |
			 Surrogate_id_SYMB_25 | Surrogate_id_SYMB_26 | Surrogate_id_SYMB_27 |
			 Surrogate_id_SYMB_34 | Surrogate_id_SYMB_38 | Surrogate_id_SYMB_39 |
			 Surrogate_id_SYMB_41 | Surrogate_id_SYMB_48 | Surrogate_id_SYMB_52 |
			 Surrogate_id_SYMB_55 | Surrogate_id_SYMB_57 | StellaIdent | INTEGER 
				=> {
					{
					/*InvokeRule expr3*/
					recog.base.set_state(610);
					let tmp = recog.expr3_rec(0)?;
					 cast_mut::<_,Expr2Context >(&mut _localctx).p_10_1 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(624);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(622);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Expr2ContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,Expr2Context>(&mut tmp)).p_1_1 = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr2);
							_localctx = tmp;
							recog.base.set_state(613);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(614);
							recog.base.match_token(Surrogate_id_SYMB_33,&mut recog.err_handler)?;

							/*InvokeRule stellatype*/
							recog.base.set_state(615);
							let tmp = recog.stellatype()?;
							 cast_mut::<_,Expr2Context >(&mut _localctx).p_1_3 = Some(tmp.clone());
							  

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Expr2ContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,Expr2Context>(&mut tmp)).p_8_1 = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr2);
							_localctx = tmp;
							recog.base.set_state(616);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(617);
							recog.base.match_token(Surrogate_id_SYMB_20,&mut recog.err_handler)?;

							/*InvokeRule expr3*/
							recog.base.set_state(618);
							let tmp = recog.expr3_rec(0)?;
							 cast_mut::<_,Expr2Context >(&mut _localctx).p_8_3 = Some(tmp.clone());
							  

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Expr2ContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,Expr2Context>(&mut tmp)).p_9_1 = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr2);
							_localctx = tmp;
							recog.base.set_state(619);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(620);
							recog.base.match_token(Surrogate_id_SYMB_49,&mut recog.err_handler)?;

							/*InvokeRule expr3*/
							recog.base.set_state(621);
							let tmp = recog.expr3_rec(0)?;
							 cast_mut::<_,Expr2Context >(&mut _localctx).p_9_3 = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(626);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- expr3 ----------------
pub type Expr3ContextAll<'input> = Expr3Context<'input>;


pub type Expr3Context<'input> = BaseParserRuleContext<'input,Expr3ContextExt<'input>>;

#[derive(Clone)]
pub struct Expr3ContextExt<'input>{
	pub p_1_1: Option<Rc<Expr3ContextAll<'input>>>,
	pub p_2_1: Option<Rc<Expr3ContextAll<'input>>>,
	pub p_3_1: Option<Rc<Expr4ContextAll<'input>>>,
	pub p_1_3: Option<Rc<Expr4ContextAll<'input>>>,
	pub p_2_3: Option<Rc<Expr4ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Expr3Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Expr3Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr3(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_expr3(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expr3ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr3 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr3 }
}
antlr_rust::tid!{Expr3ContextExt<'a>}

impl<'input> Expr3ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expr3ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expr3ContextExt{
				p_1_1: None, p_2_1: None, p_3_1: None, p_1_3: None, p_2_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Expr3ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Expr3ContextExt<'input>>{

fn expr4(&self) -> Option<Rc<Expr4ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_21
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_21
fn Surrogate_id_SYMB_21(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_21, 0)
}
fn expr3(&self) -> Option<Rc<Expr3ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_32
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_32
fn Surrogate_id_SYMB_32(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_32, 0)
}

}

impl<'input> Expr3ContextAttrs<'input> for Expr3Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr3(&mut self,)
	-> Result<Rc<Expr3ContextAll<'input>>,ANTLRError> {
		self.expr3_rec(0)
	}

	fn expr3_rec(&mut self, _p: isize)
	-> Result<Rc<Expr3ContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Expr3ContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 152, RULE_expr3, _p);
	    let mut _localctx: Rc<Expr3ContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 152;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule expr4*/
			recog.base.set_state(628);
			let tmp = recog.expr4_rec(0)?;
			 cast_mut::<_,Expr3Context >(&mut _localctx).p_3_1 = Some(tmp.clone());
			  

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(638);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(636);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Expr3ContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,Expr3Context>(&mut tmp)).p_1_1 = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr3);
							_localctx = tmp;
							recog.base.set_state(630);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(631);
							recog.base.match_token(Surrogate_id_SYMB_21,&mut recog.err_handler)?;

							/*InvokeRule expr4*/
							recog.base.set_state(632);
							let tmp = recog.expr4_rec(0)?;
							 cast_mut::<_,Expr3Context >(&mut _localctx).p_1_3 = Some(tmp.clone());
							  

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Expr3ContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,Expr3Context>(&mut tmp)).p_2_1 = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr3);
							_localctx = tmp;
							recog.base.set_state(633);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(634);
							recog.base.match_token(Surrogate_id_SYMB_32,&mut recog.err_handler)?;

							/*InvokeRule expr4*/
							recog.base.set_state(635);
							let tmp = recog.expr4_rec(0)?;
							 cast_mut::<_,Expr3Context >(&mut _localctx).p_2_3 = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(640);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- expr4 ----------------
pub type Expr4ContextAll<'input> = Expr4Context<'input>;


pub type Expr4Context<'input> = BaseParserRuleContext<'input,Expr4ContextExt<'input>>;

#[derive(Clone)]
pub struct Expr4ContextExt<'input>{
	pub p_1_1: Option<Rc<Expr4ContextAll<'input>>>,
	pub p_2_1: Option<Rc<Expr5ContextAll<'input>>>,
	pub p_1_3: Option<Rc<ListExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Expr4Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Expr4Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr4(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_expr4(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expr4ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr4 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr4 }
}
antlr_rust::tid!{Expr4ContextExt<'a>}

impl<'input> Expr4ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expr4ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expr4ContextExt{
				p_1_1: None, p_2_1: None, p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Expr4ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Expr4ContextExt<'input>>{

fn expr5(&self) -> Option<Rc<Expr5ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
fn expr4(&self) -> Option<Rc<Expr4ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn listExpr(&self) -> Option<Rc<ListExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Expr4ContextAttrs<'input> for Expr4Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr4(&mut self,)
	-> Result<Rc<Expr4ContextAll<'input>>,ANTLRError> {
		self.expr4_rec(0)
	}

	fn expr4_rec(&mut self, _p: isize)
	-> Result<Rc<Expr4ContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Expr4ContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 154, RULE_expr4, _p);
	    let mut _localctx: Rc<Expr4ContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 154;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule expr5*/
			recog.base.set_state(642);
			let tmp = recog.expr5()?;
			 cast_mut::<_,Expr4Context >(&mut _localctx).p_2_1 = Some(tmp.clone());
			  

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(651);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Expr4ContextExt::new(_parentctx.clone(), _parentState);
					(cast_mut::<_,Expr4Context>(&mut tmp)).p_1_1 = Some(_prevctx.clone());
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr4);
					_localctx = tmp;
					recog.base.set_state(644);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(645);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule listExpr*/
					recog.base.set_state(646);
					let tmp = recog.listExpr()?;
					 cast_mut::<_,Expr4Context >(&mut _localctx).p_1_3 = Some(tmp.clone());
					  

					recog.base.set_state(647);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(653);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- expr5 ----------------
pub type Expr5ContextAll<'input> = Expr5Context<'input>;


pub type Expr5Context<'input> = BaseParserRuleContext<'input,Expr5ContextExt<'input>>;

#[derive(Clone)]
pub struct Expr5ContextExt<'input>{
	pub p_1_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_1_5: Option<Rc<ExprContextAll<'input>>>,
	pub p_2_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_3_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_4_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_5_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_6_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_7_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_8_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_9_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_10_3: Option<Rc<ExprContextAll<'input>>>,
	pub p_10_5: Option<Rc<ExprContextAll<'input>>>,
	pub p_10_7: Option<Rc<ExprContextAll<'input>>>,
	pub p_11_3: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_11_5: Option<Rc<Expr6ContextAll<'input>>>,
	pub p_12_3: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_12_5: Option<Rc<Expr6ContextAll<'input>>>,
	pub p_13_1: Option<Rc<Expr6ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Expr5Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Expr5Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr5(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_expr5(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expr5ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr5 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr5 }
}
antlr_rust::tid!{Expr5ContextExt<'a>}

impl<'input> Expr5ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expr5ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expr5ContextExt{
				p_1_3: None, p_1_5: None, p_2_3: None, p_3_3: None, p_4_3: None, p_5_3: None, p_6_3: None, p_7_3: None, p_8_3: None, p_9_3: None, p_10_3: None, p_10_5: None, p_10_7: None, p_11_3: None, p_11_5: None, p_12_3: None, p_12_5: None, p_13_1: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Expr5ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Expr5ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_34
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_34
fn Surrogate_id_SYMB_34(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_34, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token Surrogate_id_SYMB_0 in current rule
fn Surrogate_id_SYMB_0_all(&self) -> Vec<Rc<TerminalNode<'input,stellaParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token Surrogate_id_SYMB_0, starting from 0.
/// Returns `None` if number of children corresponding to token Surrogate_id_SYMB_0 is less or equal than `i`.
fn Surrogate_id_SYMB_0(&self, i: usize) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, i)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_22
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_22
fn Surrogate_id_SYMB_22(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_22, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_23
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_23
fn Surrogate_id_SYMB_23(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_23, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_24
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_24
fn Surrogate_id_SYMB_24(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_24, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_52
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_52
fn Surrogate_id_SYMB_52(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_52, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_48
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_48
fn Surrogate_id_SYMB_48(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_48, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_25
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_25
fn Surrogate_id_SYMB_25(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_25, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_26
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_26
fn Surrogate_id_SYMB_26(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_26, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_39
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_39
fn Surrogate_id_SYMB_39(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_39, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_27
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_27
fn Surrogate_id_SYMB_27(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_27, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_41
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_41
fn Surrogate_id_SYMB_41(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_41, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_12
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_12
fn Surrogate_id_SYMB_12(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_12, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_13
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_13
fn Surrogate_id_SYMB_13(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_13, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expr6(&self) -> Option<Rc<Expr6ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_57
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_57
fn Surrogate_id_SYMB_57(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_57, 0)
}

}

impl<'input> Expr5ContextAttrs<'input> for Expr5Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expr5(&mut self,)
	-> Result<Rc<Expr5ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Expr5ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_expr5);
        let mut _localctx: Rc<Expr5ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(723);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_34 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(654);
					recog.base.match_token(Surrogate_id_SYMB_34,&mut recog.err_handler)?;

					recog.base.set_state(655);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(656);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_1_3 = Some(tmp.clone());
					  

					recog.base.set_state(657);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(658);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_1_5 = Some(tmp.clone());
					  

					recog.base.set_state(659);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_22 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(661);
					recog.base.match_token(Surrogate_id_SYMB_22,&mut recog.err_handler)?;

					recog.base.set_state(662);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(663);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_2_3 = Some(tmp.clone());
					  

					recog.base.set_state(664);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_23 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(666);
					recog.base.match_token(Surrogate_id_SYMB_23,&mut recog.err_handler)?;

					recog.base.set_state(667);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(668);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					recog.base.set_state(669);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_24 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(671);
					recog.base.match_token(Surrogate_id_SYMB_24,&mut recog.err_handler)?;

					recog.base.set_state(672);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(673);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_4_3 = Some(tmp.clone());
					  

					recog.base.set_state(674);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_52 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(676);
					recog.base.match_token(Surrogate_id_SYMB_52,&mut recog.err_handler)?;

					recog.base.set_state(677);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(678);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_5_3 = Some(tmp.clone());
					  

					recog.base.set_state(679);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_48 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(681);
					recog.base.match_token(Surrogate_id_SYMB_48,&mut recog.err_handler)?;

					recog.base.set_state(682);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(683);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_6_3 = Some(tmp.clone());
					  

					recog.base.set_state(684);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_25 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(686);
					recog.base.match_token(Surrogate_id_SYMB_25,&mut recog.err_handler)?;

					recog.base.set_state(687);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(688);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_7_3 = Some(tmp.clone());
					  

					recog.base.set_state(689);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_26 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					recog.base.set_state(691);
					recog.base.match_token(Surrogate_id_SYMB_26,&mut recog.err_handler)?;

					recog.base.set_state(692);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(693);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_8_3 = Some(tmp.clone());
					  

					recog.base.set_state(694);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_39 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					recog.base.set_state(696);
					recog.base.match_token(Surrogate_id_SYMB_39,&mut recog.err_handler)?;

					recog.base.set_state(697);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(698);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_9_3 = Some(tmp.clone());
					  

					recog.base.set_state(699);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_27 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					recog.base.set_state(701);
					recog.base.match_token(Surrogate_id_SYMB_27,&mut recog.err_handler)?;

					recog.base.set_state(702);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(703);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_10_3 = Some(tmp.clone());
					  

					recog.base.set_state(704);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(705);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_10_5 = Some(tmp.clone());
					  

					recog.base.set_state(706);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(707);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_10_7 = Some(tmp.clone());
					  

					recog.base.set_state(708);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_41 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					recog.base.set_state(710);
					recog.base.match_token(Surrogate_id_SYMB_41,&mut recog.err_handler)?;

					recog.base.set_state(711);
					recog.base.match_token(Surrogate_id_SYMB_12,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(712);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_11_3 = Some(tmp.clone());
					  

					recog.base.set_state(713);
					recog.base.match_token(Surrogate_id_SYMB_13,&mut recog.err_handler)?;

					/*InvokeRule expr6*/
					recog.base.set_state(714);
					let tmp = recog.expr6_rec(0)?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_11_5 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_57 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					recog.base.set_state(716);
					recog.base.match_token(Surrogate_id_SYMB_57,&mut recog.err_handler)?;

					recog.base.set_state(717);
					recog.base.match_token(Surrogate_id_SYMB_12,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(718);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_12_3 = Some(tmp.clone());
					  

					recog.base.set_state(719);
					recog.base.match_token(Surrogate_id_SYMB_13,&mut recog.err_handler)?;

					/*InvokeRule expr6*/
					recog.base.set_state(720);
					let tmp = recog.expr6_rec(0)?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_12_5 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_2 | Surrogate_id_SYMB_38 | Surrogate_id_SYMB_55 | StellaIdent |
			 INTEGER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					/*InvokeRule expr6*/
					recog.base.set_state(722);
					let tmp = recog.expr6_rec(0)?;
					 cast_mut::<_,Expr5Context >(&mut _localctx).p_13_1 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr6 ----------------
pub type Expr6ContextAll<'input> = Expr6Context<'input>;


pub type Expr6Context<'input> = BaseParserRuleContext<'input,Expr6ContextExt<'input>>;

#[derive(Clone)]
pub struct Expr6ContextExt<'input>{
	pub p_1_1: Option<Rc<Expr6ContextAll<'input>>>,
	pub p_2_1: Option<Rc<Expr6ContextAll<'input>>>,
	pub p_5_1: Option<TokenType<'input>>,
	pub p_6_1: Option<TokenType<'input>>,
	pub p_7_2: Option<Rc<ExprContextAll<'input>>>,
	pub p_1_3: Option<TokenType<'input>>,
	pub p_2_3: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Expr6Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Expr6Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr6(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_expr6(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Expr6ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr6 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr6 }
}
antlr_rust::tid!{Expr6ContextExt<'a>}

impl<'input> Expr6ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expr6ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expr6ContextExt{
				p_5_1: None, p_6_1: None, p_1_3: None, p_2_3: None, 
				p_1_1: None, p_2_1: None, p_7_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Expr6ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Expr6ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_55
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_55
fn Surrogate_id_SYMB_55(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_55, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_38
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_38
fn Surrogate_id_SYMB_38(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_38, 0)
}
/// Retrieves first TerminalNode corresponding to token INTEGER
/// Returns `None` if there is no child corresponding to token INTEGER
fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(INTEGER, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_28
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_28
fn Surrogate_id_SYMB_28(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_28, 0)
}
fn expr6(&self) -> Option<Rc<Expr6ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Expr6ContextAttrs<'input> for Expr6Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr6(&mut self,)
	-> Result<Rc<Expr6ContextAll<'input>>,ANTLRError> {
		self.expr6_rec(0)
	}

	fn expr6_rec(&mut self, _p: isize)
	-> Result<Rc<Expr6ContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Expr6ContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 158, RULE_expr6, _p);
	    let mut _localctx: Rc<Expr6ContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 158;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(734);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_55 
				=> {
					{
					recog.base.set_state(726);
					recog.base.match_token(Surrogate_id_SYMB_55,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_38 
				=> {
					{
					recog.base.set_state(727);
					recog.base.match_token(Surrogate_id_SYMB_38,&mut recog.err_handler)?;

					}
				}

			 INTEGER 
				=> {
					{
					recog.base.set_state(728);
					let tmp = recog.base.match_token(INTEGER,&mut recog.err_handler)?;
					 cast_mut::<_,Expr6Context >(&mut _localctx).p_5_1 = Some(tmp.clone());
					  

					}
				}

			 StellaIdent 
				=> {
					{
					recog.base.set_state(729);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,Expr6Context >(&mut _localctx).p_6_1 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_2 
				=> {
					{
					recog.base.set_state(730);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(731);
					let tmp = recog.expr()?;
					 cast_mut::<_,Expr6Context >(&mut _localctx).p_7_2 = Some(tmp.clone());
					  

					recog.base.set_state(732);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(744);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(30,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(742);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Expr6ContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,Expr6Context>(&mut tmp)).p_1_1 = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr6);
							_localctx = tmp;
							recog.base.set_state(736);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(737);
							recog.base.match_token(Surrogate_id_SYMB_28,&mut recog.err_handler)?;

							recog.base.set_state(738);
							let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
							 cast_mut::<_,Expr6Context >(&mut _localctx).p_1_3 = Some(tmp.clone());
							  

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Expr6ContextExt::new(_parentctx.clone(), _parentState);
							(cast_mut::<_,Expr6Context>(&mut tmp)).p_2_1 = Some(_prevctx.clone());
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr6);
							_localctx = tmp;
							recog.base.set_state(739);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(740);
							recog.base.match_token(Surrogate_id_SYMB_28,&mut recog.err_handler)?;

							recog.base.set_state(741);
							let tmp = recog.base.match_token(INTEGER,&mut recog.err_handler)?;
							 cast_mut::<_,Expr6Context >(&mut _localctx).p_2_3 = Some(tmp.clone());
							  

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(746);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(30,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- stellatype ----------------
pub type StellatypeContextAll<'input> = StellatypeContext<'input>;


pub type StellatypeContext<'input> = BaseParserRuleContext<'input,StellatypeContextExt<'input>>;

#[derive(Clone)]
pub struct StellatypeContextExt<'input>{
	pub p_1_3: Option<Rc<ListTypeContextAll<'input>>>,
	pub p_1_6: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_2_2: Option<TokenType<'input>>,
	pub p_2_4: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_3_1: Option<Rc<Type1ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for StellatypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for StellatypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_stellatype(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_stellatype(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StellatypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stellatype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stellatype }
}
antlr_rust::tid!{StellatypeContextExt<'a>}

impl<'input> StellatypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StellatypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StellatypeContextExt{
				p_2_2: None, 
				p_1_3: None, p_1_6: None, p_2_4: None, p_3_1: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait StellatypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<StellatypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_40
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_40
fn Surrogate_id_SYMB_40(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_40, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_8
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_8
fn Surrogate_id_SYMB_8(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_8, 0)
}
fn listType(&self) -> Option<Rc<ListTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_60
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_60
fn Surrogate_id_SYMB_60(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_60, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_28
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_28
fn Surrogate_id_SYMB_28(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_28, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn type1(&self) -> Option<Rc<Type1ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StellatypeContextAttrs<'input> for StellatypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stellatype(&mut self,)
	-> Result<Rc<StellatypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StellatypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_stellatype);
        let mut _localctx: Rc<StellatypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(759);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_40 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(747);
					recog.base.match_token(Surrogate_id_SYMB_40,&mut recog.err_handler)?;

					recog.base.set_state(748);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule listType*/
					recog.base.set_state(749);
					let tmp = recog.listType()?;
					 cast_mut::<_,StellatypeContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
					  

					recog.base.set_state(750);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					recog.base.set_state(751);
					recog.base.match_token(Surrogate_id_SYMB_8,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(752);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,StellatypeContext >(&mut _localctx).p_1_6 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_60 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(754);
					recog.base.match_token(Surrogate_id_SYMB_60,&mut recog.err_handler)?;

					recog.base.set_state(755);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,StellatypeContext >(&mut _localctx).p_2_2 = Some(tmp.clone());
					  

					recog.base.set_state(756);
					recog.base.match_token(Surrogate_id_SYMB_28,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(757);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,StellatypeContext >(&mut _localctx).p_2_4 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_2 | Surrogate_id_SYMB_4 | Surrogate_id_SYMB_12 | Surrogate_id_SYMB_29 |
			 Surrogate_id_SYMB_30 | Surrogate_id_SYMB_31 | Surrogate_id_SYMB_50 |
			 Surrogate_id_SYMB_58 | StellaIdent 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule type1*/
					recog.base.set_state(758);
					let tmp = recog.type1()?;
					 cast_mut::<_,StellatypeContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type1 ----------------
pub type Type1ContextAll<'input> = Type1Context<'input>;


pub type Type1Context<'input> = BaseParserRuleContext<'input,Type1ContextExt<'input>>;

#[derive(Clone)]
pub struct Type1ContextExt<'input>{
	pub p_1_1: Option<Rc<Type2ContextAll<'input>>>,
	pub p_1_3: Option<Rc<Type2ContextAll<'input>>>,
	pub p_2_1: Option<Rc<Type2ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Type1Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Type1Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type1(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_type1(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Type1ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type1 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type1 }
}
antlr_rust::tid!{Type1ContextExt<'a>}

impl<'input> Type1ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type1ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type1ContextExt{
				p_1_1: None, p_1_3: None, p_2_1: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Type1ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Type1ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_20
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_20
fn Surrogate_id_SYMB_20(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_20, 0)
}
fn type2_all(&self) ->  Vec<Rc<Type2ContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn type2(&self, i: usize) -> Option<Rc<Type2ContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Type1ContextAttrs<'input> for Type1Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type1(&mut self,)
	-> Result<Rc<Type1ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type1ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_type1);
        let mut _localctx: Rc<Type1ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(766);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule type2*/
					recog.base.set_state(761);
					let tmp = recog.type2()?;
					 cast_mut::<_,Type1Context >(&mut _localctx).p_1_1 = Some(tmp.clone());
					  

					recog.base.set_state(762);
					recog.base.match_token(Surrogate_id_SYMB_20,&mut recog.err_handler)?;

					/*InvokeRule type2*/
					recog.base.set_state(763);
					let tmp = recog.type2()?;
					 cast_mut::<_,Type1Context >(&mut _localctx).p_1_3 = Some(tmp.clone());
					  

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule type2*/
					recog.base.set_state(765);
					let tmp = recog.type2()?;
					 cast_mut::<_,Type1Context >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type2 ----------------
pub type Type2ContextAll<'input> = Type2Context<'input>;


pub type Type2Context<'input> = BaseParserRuleContext<'input,Type2ContextExt<'input>>;

#[derive(Clone)]
pub struct Type2ContextExt<'input>{
	pub p_1_2: Option<Rc<ListTypeContextAll<'input>>>,
	pub p_2_3: Option<Rc<ListRecordFieldTypeContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListVariantFieldTypeContextAll<'input>>>,
	pub p_4_2: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_5_1: Option<Rc<Type3ContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Type2Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Type2Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type2(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_type2(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Type2ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type2 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type2 }
}
antlr_rust::tid!{Type2ContextExt<'a>}

impl<'input> Type2ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type2ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type2ContextExt{
				p_1_2: None, p_2_3: None, p_3_3: None, p_4_2: None, p_5_1: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Type2ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Type2ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_4
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_4
fn Surrogate_id_SYMB_4(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_4, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_5
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_5
fn Surrogate_id_SYMB_5(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_5, 0)
}
fn listType(&self) -> Option<Rc<ListTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_50
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_50
fn Surrogate_id_SYMB_50(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_50, 0)
}
fn listRecordFieldType(&self) -> Option<Rc<ListRecordFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_58
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_58
fn Surrogate_id_SYMB_58(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_58, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_10
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_10
fn Surrogate_id_SYMB_10(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_10, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_11
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_11
fn Surrogate_id_SYMB_11(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_11, 0)
}
fn listVariantFieldType(&self) -> Option<Rc<ListVariantFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_12
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_12
fn Surrogate_id_SYMB_12(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_12, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_13
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_13
fn Surrogate_id_SYMB_13(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_13, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type3(&self) -> Option<Rc<Type3ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type2ContextAttrs<'input> for Type2Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type2(&mut self,)
	-> Result<Rc<Type2ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type2ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_type2);
        let mut _localctx: Rc<Type2ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(787);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(768);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listType*/
					recog.base.set_state(769);
					let tmp = recog.listType()?;
					 cast_mut::<_,Type2Context >(&mut _localctx).p_1_2 = Some(tmp.clone());
					  

					recog.base.set_state(770);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_50 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(772);
					recog.base.match_token(Surrogate_id_SYMB_50,&mut recog.err_handler)?;

					recog.base.set_state(773);
					recog.base.match_token(Surrogate_id_SYMB_4,&mut recog.err_handler)?;

					/*InvokeRule listRecordFieldType*/
					recog.base.set_state(774);
					let tmp = recog.listRecordFieldType()?;
					 cast_mut::<_,Type2Context >(&mut _localctx).p_2_3 = Some(tmp.clone());
					  

					recog.base.set_state(775);
					recog.base.match_token(Surrogate_id_SYMB_5,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_58 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(777);
					recog.base.match_token(Surrogate_id_SYMB_58,&mut recog.err_handler)?;

					recog.base.set_state(778);
					recog.base.match_token(Surrogate_id_SYMB_10,&mut recog.err_handler)?;

					/*InvokeRule listVariantFieldType*/
					recog.base.set_state(779);
					let tmp = recog.listVariantFieldType()?;
					 cast_mut::<_,Type2Context >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					recog.base.set_state(780);
					recog.base.match_token(Surrogate_id_SYMB_11,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_12 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(782);
					recog.base.match_token(Surrogate_id_SYMB_12,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(783);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,Type2Context >(&mut _localctx).p_4_2 = Some(tmp.clone());
					  

					recog.base.set_state(784);
					recog.base.match_token(Surrogate_id_SYMB_13,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_2 | Surrogate_id_SYMB_29 | Surrogate_id_SYMB_30 | Surrogate_id_SYMB_31 |
			 StellaIdent 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule type3*/
					recog.base.set_state(786);
					let tmp = recog.type3()?;
					 cast_mut::<_,Type2Context >(&mut _localctx).p_5_1 = Some(tmp.clone());
					  

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type3 ----------------
pub type Type3ContextAll<'input> = Type3Context<'input>;


pub type Type3Context<'input> = BaseParserRuleContext<'input,Type3ContextExt<'input>>;

#[derive(Clone)]
pub struct Type3ContextExt<'input>{
	pub p_4_1: Option<TokenType<'input>>,
	pub p_5_2: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for Type3Context<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for Type3Context<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type3(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_type3(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Type3ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type3 }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type3 }
}
antlr_rust::tid!{Type3ContextExt<'a>}

impl<'input> Type3ContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type3ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type3ContextExt{
				p_4_1: None, 
				p_5_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait Type3ContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<Type3ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_29
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_29
fn Surrogate_id_SYMB_29(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_29, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_30
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_30
fn Surrogate_id_SYMB_30(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_30, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_31
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_31
fn Surrogate_id_SYMB_31(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_31, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_2
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_2
fn Surrogate_id_SYMB_2(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_2, 0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_3
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_3
fn Surrogate_id_SYMB_3(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_3, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Type3ContextAttrs<'input> for Type3Context<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type3(&mut self,)
	-> Result<Rc<Type3ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type3ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_type3);
        let mut _localctx: Rc<Type3ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(797);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 Surrogate_id_SYMB_29 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(789);
					recog.base.match_token(Surrogate_id_SYMB_29,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_30 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(790);
					recog.base.match_token(Surrogate_id_SYMB_30,&mut recog.err_handler)?;

					}
				}

			 Surrogate_id_SYMB_31 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(791);
					recog.base.match_token(Surrogate_id_SYMB_31,&mut recog.err_handler)?;

					}
				}

			 StellaIdent 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(792);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					 cast_mut::<_,Type3Context >(&mut _localctx).p_4_1 = Some(tmp.clone());
					  

					}
				}

			 Surrogate_id_SYMB_2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(793);
					recog.base.match_token(Surrogate_id_SYMB_2,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(794);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,Type3Context >(&mut _localctx).p_5_2 = Some(tmp.clone());
					  

					recog.base.set_state(795);
					recog.base.match_token(Surrogate_id_SYMB_3,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listType ----------------
pub type ListTypeContextAll<'input> = ListTypeContext<'input>;


pub type ListTypeContext<'input> = BaseParserRuleContext<'input,ListTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ListTypeContextExt<'input>{
	pub p_2_1: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_3_1: Option<Rc<StellatypeContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listType }
}
antlr_rust::tid!{ListTypeContextExt<'a>}

impl<'input> ListTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListTypeContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListTypeContextExt<'input>>{

fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listType(&self) -> Option<Rc<ListTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListTypeContextAttrs<'input> for ListTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listType(&mut self,)
	-> Result<Rc<ListTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_listType);
        let mut _localctx: Rc<ListTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(805);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(35,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule stellatype*/
					recog.base.set_state(800);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,ListTypeContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule stellatype*/
					recog.base.set_state(801);
					let tmp = recog.stellatype()?;
					 cast_mut::<_,ListTypeContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(802);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listType*/
					recog.base.set_state(803);
					let tmp = recog.listType()?;
					 cast_mut::<_,ListTypeContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variantFieldType ----------------
pub type VariantFieldTypeContextAll<'input> = VariantFieldTypeContext<'input>;


pub type VariantFieldTypeContext<'input> = BaseParserRuleContext<'input,VariantFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct VariantFieldTypeContextExt<'input>{
	pub p_1_1: Option<TokenType<'input>>,
	pub p_1_2: Option<Rc<OptionalTypingContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for VariantFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for VariantFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variantFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_variantFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for VariantFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variantFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variantFieldType }
}
antlr_rust::tid!{VariantFieldTypeContextExt<'a>}

impl<'input> VariantFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariantFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariantFieldTypeContextExt{
				p_1_1: None, 
				p_1_2: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait VariantFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<VariantFieldTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn optionalTyping(&self) -> Option<Rc<OptionalTypingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariantFieldTypeContextAttrs<'input> for VariantFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variantFieldType(&mut self,)
	-> Result<Rc<VariantFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariantFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_variantFieldType);
        let mut _localctx: Rc<VariantFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(807);
			let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
			 cast_mut::<_,VariantFieldTypeContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			/*InvokeRule optionalTyping*/
			recog.base.set_state(808);
			let tmp = recog.optionalTyping()?;
			 cast_mut::<_,VariantFieldTypeContext >(&mut _localctx).p_1_2 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listVariantFieldType ----------------
pub type ListVariantFieldTypeContextAll<'input> = ListVariantFieldTypeContext<'input>;


pub type ListVariantFieldTypeContext<'input> = BaseParserRuleContext<'input,ListVariantFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ListVariantFieldTypeContextExt<'input>{
	pub p_2_1: Option<Rc<VariantFieldTypeContextAll<'input>>>,
	pub p_3_1: Option<Rc<VariantFieldTypeContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListVariantFieldTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListVariantFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListVariantFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listVariantFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listVariantFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListVariantFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listVariantFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listVariantFieldType }
}
antlr_rust::tid!{ListVariantFieldTypeContextExt<'a>}

impl<'input> ListVariantFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListVariantFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListVariantFieldTypeContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListVariantFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListVariantFieldTypeContextExt<'input>>{

fn variantFieldType(&self) -> Option<Rc<VariantFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listVariantFieldType(&self) -> Option<Rc<ListVariantFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListVariantFieldTypeContextAttrs<'input> for ListVariantFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listVariantFieldType(&mut self,)
	-> Result<Rc<ListVariantFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListVariantFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_listVariantFieldType);
        let mut _localctx: Rc<ListVariantFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(816);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule variantFieldType*/
					recog.base.set_state(811);
					let tmp = recog.variantFieldType()?;
					 cast_mut::<_,ListVariantFieldTypeContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule variantFieldType*/
					recog.base.set_state(812);
					let tmp = recog.variantFieldType()?;
					 cast_mut::<_,ListVariantFieldTypeContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(813);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listVariantFieldType*/
					recog.base.set_state(814);
					let tmp = recog.listVariantFieldType()?;
					 cast_mut::<_,ListVariantFieldTypeContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- recordFieldType ----------------
pub type RecordFieldTypeContextAll<'input> = RecordFieldTypeContext<'input>;


pub type RecordFieldTypeContext<'input> = BaseParserRuleContext<'input,RecordFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct RecordFieldTypeContextExt<'input>{
	pub p_1_1: Option<TokenType<'input>>,
	pub p_1_3: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for RecordFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for RecordFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_recordFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_recordFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RecordFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_recordFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_recordFieldType }
}
antlr_rust::tid!{RecordFieldTypeContextExt<'a>}

impl<'input> RecordFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RecordFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RecordFieldTypeContextExt{
				p_1_1: None, 
				p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait RecordFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<RecordFieldTypeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_7
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_7
fn Surrogate_id_SYMB_7(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_7, 0)
}
/// Retrieves first TerminalNode corresponding to token StellaIdent
/// Returns `None` if there is no child corresponding to token StellaIdent
fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(StellaIdent, 0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RecordFieldTypeContextAttrs<'input> for RecordFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn recordFieldType(&mut self,)
	-> Result<Rc<RecordFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RecordFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_recordFieldType);
        let mut _localctx: Rc<RecordFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(818);
			let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
			 cast_mut::<_,RecordFieldTypeContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			recog.base.set_state(819);
			recog.base.match_token(Surrogate_id_SYMB_7,&mut recog.err_handler)?;

			/*InvokeRule stellatype*/
			recog.base.set_state(820);
			let tmp = recog.stellatype()?;
			 cast_mut::<_,RecordFieldTypeContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- listRecordFieldType ----------------
pub type ListRecordFieldTypeContextAll<'input> = ListRecordFieldTypeContext<'input>;


pub type ListRecordFieldTypeContext<'input> = BaseParserRuleContext<'input,ListRecordFieldTypeContextExt<'input>>;

#[derive(Clone)]
pub struct ListRecordFieldTypeContextExt<'input>{
	pub p_2_1: Option<Rc<RecordFieldTypeContextAll<'input>>>,
	pub p_3_1: Option<Rc<RecordFieldTypeContextAll<'input>>>,
	pub p_3_3: Option<Rc<ListRecordFieldTypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ListRecordFieldTypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ListRecordFieldTypeContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_listRecordFieldType(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_listRecordFieldType(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ListRecordFieldTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_listRecordFieldType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_listRecordFieldType }
}
antlr_rust::tid!{ListRecordFieldTypeContextExt<'a>}

impl<'input> ListRecordFieldTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ListRecordFieldTypeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ListRecordFieldTypeContextExt{
				p_2_1: None, p_3_1: None, p_3_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ListRecordFieldTypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ListRecordFieldTypeContextExt<'input>>{

fn recordFieldType(&self) -> Option<Rc<RecordFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_0
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_0
fn Surrogate_id_SYMB_0(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_0, 0)
}
fn listRecordFieldType(&self) -> Option<Rc<ListRecordFieldTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ListRecordFieldTypeContextAttrs<'input> for ListRecordFieldTypeContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn listRecordFieldType(&mut self,)
	-> Result<Rc<ListRecordFieldTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ListRecordFieldTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_listRecordFieldType);
        let mut _localctx: Rc<ListRecordFieldTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(828);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule recordFieldType*/
					recog.base.set_state(823);
					let tmp = recog.recordFieldType()?;
					 cast_mut::<_,ListRecordFieldTypeContext >(&mut _localctx).p_2_1 = Some(tmp.clone());
					  

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule recordFieldType*/
					recog.base.set_state(824);
					let tmp = recog.recordFieldType()?;
					 cast_mut::<_,ListRecordFieldTypeContext >(&mut _localctx).p_3_1 = Some(tmp.clone());
					  

					recog.base.set_state(825);
					recog.base.match_token(Surrogate_id_SYMB_0,&mut recog.err_handler)?;

					/*InvokeRule listRecordFieldType*/
					recog.base.set_state(826);
					let tmp = recog.listRecordFieldType()?;
					 cast_mut::<_,ListRecordFieldTypeContext >(&mut _localctx).p_3_3 = Some(tmp.clone());
					  

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typing ----------------
pub type TypingContextAll<'input> = TypingContext<'input>;


pub type TypingContext<'input> = BaseParserRuleContext<'input,TypingContextExt<'input>>;

#[derive(Clone)]
pub struct TypingContextExt<'input>{
	pub p_1_1: Option<Rc<ExprContextAll<'input>>>,
	pub p_1_3: Option<Rc<StellatypeContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for TypingContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for TypingContext<'input>{
		fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_typing(self);
		}fn exit(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
			listener.exit_typing(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for TypingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typing }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typing }
}
antlr_rust::tid!{TypingContextExt<'a>}

impl<'input> TypingContextExt<'input>{
	fn new(parent: Option<Rc<dyn stellaParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypingContextExt{
				p_1_1: None, p_1_3: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait TypingContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<TypingContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Surrogate_id_SYMB_7
/// Returns `None` if there is no child corresponding to token Surrogate_id_SYMB_7
fn Surrogate_id_SYMB_7(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(Surrogate_id_SYMB_7, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stellatype(&self) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypingContextAttrs<'input> for TypingContext<'input>{}

impl<'input, I, H> stellaParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typing(&mut self,)
	-> Result<Rc<TypingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_typing);
        let mut _localctx: Rc<TypingContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr*/
			recog.base.set_state(830);
			let tmp = recog.expr()?;
			 cast_mut::<_,TypingContext >(&mut _localctx).p_1_1 = Some(tmp.clone());
			  

			recog.base.set_state(831);
			recog.base.match_token(Surrogate_id_SYMB_7,&mut recog.err_handler)?;

			/*InvokeRule stellatype*/
			recog.base.set_state(832);
			let tmp = recog.stellatype()?;
			 cast_mut::<_,TypingContext >(&mut _localctx).p_1_3 = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x46\u{345}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x03\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x04\x03\
	\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\x03\x07\x03\
	\x07\x03\x07\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0d\x03\
	\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\
	\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x12\x03\x12\x03\x12\x03\x13\x03\
	\x13\x03\x13\x03\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x16\x03\
	\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x18\x03\x18\x03\x18\x03\x19\x03\
	\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x03\
	\x1c\x03\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1e\x03\x1e\x03\x1e\x03\x1f\x03\
	\x1f\x03\x1f\x03\x20\x03\x20\x03\x20\x03\x21\x03\x21\x03\x21\x03\x22\x03\
	\x22\x03\x22\x03\x23\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x03\x25\x03\
	\x25\x03\x25\x03\x26\x03\x26\x03\x26\x03\x27\x03\x27\x03\x27\x03\x28\x03\
	\x28\x03\x28\x03\x29\x03\x29\x03\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2b\x03\
	\x2b\x03\x2b\x03\x2c\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\x2d\x03\x2e\x03\
	\x2e\x03\x2e\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x03\x2f\x05\x2f\u{143}\x0a\
	\x2f\x03\x30\x03\x30\x03\x30\x03\x30\x03\x31\x03\x31\x03\x31\x03\x31\x03\
	\x32\x03\x32\x03\x32\x03\x32\x03\x33\x03\x33\x03\x33\x03\x33\x03\x33\x05\
	\x33\u{156}\x0a\x33\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x07\x34\u{15d}\
	\x0a\x34\x0c\x34\x0e\x34\u{160}\x0b\x34\x03\x35\x03\x35\x03\x35\x03\x35\
	\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\
	\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x05\x35\u{175}\x0a\x35\
	\x03\x36\x03\x36\x03\x36\x07\x36\u{17a}\x0a\x36\x0c\x36\x0e\x36\u{17d}\x0b\
	\x36\x03\x37\x03\x37\x03\x38\x03\x38\x03\x38\x03\x38\x03\x38\x07\x38\u{186}\
	\x0a\x38\x0c\x38\x0e\x38\u{189}\x0b\x38\x03\x39\x03\x39\x03\x3a\x03\x3a\
	\x03\x3a\x07\x3a\u{190}\x0a\x3a\x0c\x3a\x0e\x3a\u{193}\x0b\x3a\x03\x3b\x03\
	\x3b\x03\x3b\x03\x3b\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x03\x3c\x05\
	\x3c\u{19f}\x0a\x3c\x03\x3d\x03\x3d\x03\x3d\x05\x3d\u{1a4}\x0a\x3d\x03\x3e\
	\x03\x3e\x03\x3e\x05\x3e\u{1a9}\x0a\x3e\x03\x3f\x03\x3f\x03\x3f\x03\x3f\
	\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\
	\x03\x3f\x03\x3f\x05\x3f\u{1ba}\x0a\x3f\x03\x40\x03\x40\x03\x40\x03\x40\
	\x03\x40\x03\x40\x05\x40\u{1c2}\x0a\x40\x03\x41\x03\x41\x03\x41\x03\x41\
	\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x05\x42\u{1ce}\x0a\x42\
	\x03\x43\x03\x43\x03\x43\x05\x43\u{1d3}\x0a\x43\x03\x44\x03\x44\x03\x44\
	\x05\x44\u{1d8}\x0a\x44\x03\x45\x03\x45\x03\x45\x05\x45\u{1dd}\x0a\x45\x03\
	\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\
	\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\
	\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\
	\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\x46\x03\
	\x46\x03\x46\x05\x46\u{205}\x0a\x46\x03\x47\x03\x47\x03\x47\x03\x47\x03\
	\x47\x03\x47\x05\x47\u{20d}\x0a\x47\x03\x48\x03\x48\x03\x48\x03\x48\x03\
	\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x05\x49\u{219}\x0a\x49\x03\
	\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\x4b\x03\
	\x4b\x05\x4b\u{225}\x0a\x4b\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\
	\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\
	\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x03\
	\x4c\x03\x4c\x05\x4c\u{240}\x0a\x4c\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\
	\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\
	\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\
	\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\
	\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x05\x4d\u{266}\x0a\x4d\x03\x4d\x03\
	\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x03\x4d\x07\x4d\u{271}\
	\x0a\x4d\x0c\x4d\x0e\x4d\u{274}\x0b\x4d\x03\x4e\x03\x4e\x03\x4e\x03\x4e\
	\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x07\x4e\u{27f}\x0a\x4e\x0c\x4e\
	\x0e\x4e\u{282}\x0b\x4e\x03\x4f\x03\x4f\x03\x4f\x03\x4f\x03\x4f\x03\x4f\
	\x03\x4f\x03\x4f\x07\x4f\u{28c}\x0a\x4f\x0c\x4f\x0e\x4f\u{28f}\x0b\x4f\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\
	\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x05\x50\u{2d6}\x0a\x50\x03\
	\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x05\
	\x51\u{2e1}\x0a\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x03\x51\x07\
	\x51\u{2e9}\x0a\x51\x0c\x51\x0e\x51\u{2ec}\x0b\x51\x03\x52\x03\x52\x03\x52\
	\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\x03\x52\
	\x05\x52\u{2fa}\x0a\x52\x03\x53\x03\x53\x03\x53\x03\x53\x03\x53\x05\x53\
	\u{301}\x0a\x53\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\
	\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\x03\x54\
	\x03\x54\x03\x54\x03\x54\x05\x54\u{316}\x0a\x54\x03\x55\x03\x55\x03\x55\
	\x03\x55\x03\x55\x03\x55\x03\x55\x03\x55\x05\x55\u{320}\x0a\x55\x03\x56\
	\x03\x56\x03\x56\x03\x56\x03\x56\x03\x56\x05\x56\u{328}\x0a\x56\x03\x57\
	\x03\x57\x03\x57\x03\x58\x03\x58\x03\x58\x03\x58\x03\x58\x03\x58\x05\x58\
	\u{333}\x0a\x58\x03\x59\x03\x59\x03\x59\x03\x59\x03\x5a\x03\x5a\x03\x5a\
	\x03\x5a\x03\x5a\x03\x5a\x05\x5a\u{33f}\x0a\x5a\x03\x5b\x03\x5b\x03\x5b\
	\x03\x5b\x03\x5b\x02\x0a\x66\x6a\x6e\x72\u{98}\u{9a}\u{9c}\u{a0}\x5c\x02\
	\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\
	\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\
	\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\
	\x70\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\
	\u{8e}\u{90}\u{92}\u{94}\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\
	\u{a6}\u{a8}\u{aa}\u{ac}\u{ae}\u{b0}\u{b2}\u{b4}\x02\x02\x02\u{345}\x02\
	\u{b6}\x03\x02\x02\x02\x04\u{b9}\x03\x02\x02\x02\x06\u{bc}\x03\x02\x02\x02\
	\x08\u{bf}\x03\x02\x02\x02\x0a\u{c2}\x03\x02\x02\x02\x0c\u{c5}\x03\x02\x02\
	\x02\x0e\u{c8}\x03\x02\x02\x02\x10\u{cb}\x03\x02\x02\x02\x12\u{ce}\x03\x02\
	\x02\x02\x14\u{d1}\x03\x02\x02\x02\x16\u{d4}\x03\x02\x02\x02\x18\u{d7}\x03\
	\x02\x02\x02\x1a\u{da}\x03\x02\x02\x02\x1c\u{dd}\x03\x02\x02\x02\x1e\u{e0}\
	\x03\x02\x02\x02\x20\u{e3}\x03\x02\x02\x02\x22\u{e6}\x03\x02\x02\x02\x24\
	\u{e9}\x03\x02\x02\x02\x26\u{ec}\x03\x02\x02\x02\x28\u{ef}\x03\x02\x02\x02\
	\x2a\u{f2}\x03\x02\x02\x02\x2c\u{f5}\x03\x02\x02\x02\x2e\u{f8}\x03\x02\x02\
	\x02\x30\u{fb}\x03\x02\x02\x02\x32\u{fe}\x03\x02\x02\x02\x34\u{101}\x03\
	\x02\x02\x02\x36\u{104}\x03\x02\x02\x02\x38\u{107}\x03\x02\x02\x02\x3a\u{10a}\
	\x03\x02\x02\x02\x3c\u{10d}\x03\x02\x02\x02\x3e\u{110}\x03\x02\x02\x02\x40\
	\u{113}\x03\x02\x02\x02\x42\u{116}\x03\x02\x02\x02\x44\u{119}\x03\x02\x02\
	\x02\x46\u{11c}\x03\x02\x02\x02\x48\u{11f}\x03\x02\x02\x02\x4a\u{122}\x03\
	\x02\x02\x02\x4c\u{125}\x03\x02\x02\x02\x4e\u{128}\x03\x02\x02\x02\x50\u{12b}\
	\x03\x02\x02\x02\x52\u{12e}\x03\x02\x02\x02\x54\u{131}\x03\x02\x02\x02\x56\
	\u{134}\x03\x02\x02\x02\x58\u{137}\x03\x02\x02\x02\x5a\u{13a}\x03\x02\x02\
	\x02\x5c\u{142}\x03\x02\x02\x02\x5e\u{144}\x03\x02\x02\x02\x60\u{148}\x03\
	\x02\x02\x02\x62\u{14c}\x03\x02\x02\x02\x64\u{155}\x03\x02\x02\x02\x66\u{157}\
	\x03\x02\x02\x02\x68\u{174}\x03\x02\x02\x02\x6a\u{176}\x03\x02\x02\x02\x6c\
	\u{17e}\x03\x02\x02\x02\x6e\u{180}\x03\x02\x02\x02\x70\u{18a}\x03\x02\x02\
	\x02\x72\u{18c}\x03\x02\x02\x02\x74\u{194}\x03\x02\x02\x02\x76\u{19e}\x03\
	\x02\x02\x02\x78\u{1a3}\x03\x02\x02\x02\x7a\u{1a8}\x03\x02\x02\x02\x7c\u{1b9}\
	\x03\x02\x02\x02\x7e\u{1c1}\x03\x02\x02\x02\u{80}\u{1c3}\x03\x02\x02\x02\
	\u{82}\u{1cd}\x03\x02\x02\x02\u{84}\u{1d2}\x03\x02\x02\x02\u{86}\u{1d7}\
	\x03\x02\x02\x02\u{88}\u{1dc}\x03\x02\x02\x02\u{8a}\u{204}\x03\x02\x02\x02\
	\u{8c}\u{20c}\x03\x02\x02\x02\u{8e}\u{20e}\x03\x02\x02\x02\u{90}\u{218}\
	\x03\x02\x02\x02\u{92}\u{21a}\x03\x02\x02\x02\u{94}\u{224}\x03\x02\x02\x02\
	\u{96}\u{23f}\x03\x02\x02\x02\u{98}\u{265}\x03\x02\x02\x02\u{9a}\u{275}\
	\x03\x02\x02\x02\u{9c}\u{283}\x03\x02\x02\x02\u{9e}\u{2d5}\x03\x02\x02\x02\
	\u{a0}\u{2e0}\x03\x02\x02\x02\u{a2}\u{2f9}\x03\x02\x02\x02\u{a4}\u{300}\
	\x03\x02\x02\x02\u{a6}\u{315}\x03\x02\x02\x02\u{a8}\u{31f}\x03\x02\x02\x02\
	\u{aa}\u{327}\x03\x02\x02\x02\u{ac}\u{329}\x03\x02\x02\x02\u{ae}\u{332}\
	\x03\x02\x02\x02\u{b0}\u{334}\x03\x02\x02\x02\u{b2}\u{33e}\x03\x02\x02\x02\
	\u{b4}\u{340}\x03\x02\x02\x02\u{b6}\u{b7}\x05\x5c\x2f\x02\u{b7}\u{b8}\x07\
	\x02\x02\x03\u{b8}\x03\x03\x02\x02\x02\u{b9}\u{ba}\x05\x5e\x30\x02\u{ba}\
	\u{bb}\x07\x02\x02\x03\u{bb}\x05\x03\x02\x02\x02\u{bc}\u{bd}\x05\x60\x31\
	\x02\u{bd}\u{be}\x07\x02\x02\x03\u{be}\x07\x03\x02\x02\x02\u{bf}\u{c0}\x05\
	\x62\x32\x02\u{c0}\u{c1}\x07\x02\x02\x03\u{c1}\x09\x03\x02\x02\x02\u{c2}\
	\u{c3}\x05\x64\x33\x02\u{c3}\u{c4}\x07\x02\x02\x03\u{c4}\x0b\x03\x02\x02\
	\x02\u{c5}\u{c6}\x05\x66\x34\x02\u{c6}\u{c7}\x07\x02\x02\x03\u{c7}\x0d\x03\
	\x02\x02\x02\u{c8}\u{c9}\x05\x68\x35\x02\u{c9}\u{ca}\x07\x02\x02\x03\u{ca}\
	\x0f\x03\x02\x02\x02\u{cb}\u{cc}\x05\x6a\x36\x02\u{cc}\u{cd}\x07\x02\x02\
	\x03\u{cd}\x11\x03\x02\x02\x02\u{ce}\u{cf}\x05\x6c\x37\x02\u{cf}\u{d0}\x07\
	\x02\x02\x03\u{d0}\x13\x03\x02\x02\x02\u{d1}\u{d2}\x05\x6e\x38\x02\u{d2}\
	\u{d3}\x07\x02\x02\x03\u{d3}\x15\x03\x02\x02\x02\u{d4}\u{d5}\x05\x70\x39\
	\x02\u{d5}\u{d6}\x07\x02\x02\x03\u{d6}\x17\x03\x02\x02\x02\u{d7}\u{d8}\x05\
	\x72\x3a\x02\u{d8}\u{d9}\x07\x02\x02\x03\u{d9}\x19\x03\x02\x02\x02\u{da}\
	\u{db}\x05\x74\x3b\x02\u{db}\u{dc}\x07\x02\x02\x03\u{dc}\x1b\x03\x02\x02\
	\x02\u{dd}\u{de}\x05\x76\x3c\x02\u{de}\u{df}\x07\x02\x02\x03\u{df}\x1d\x03\
	\x02\x02\x02\u{e0}\u{e1}\x05\x78\x3d\x02\u{e1}\u{e2}\x07\x02\x02\x03\u{e2}\
	\x1f\x03\x02\x02\x02\u{e3}\u{e4}\x05\x7a\x3e\x02\u{e4}\u{e5}\x07\x02\x02\
	\x03\u{e5}\x21\x03\x02\x02\x02\u{e6}\u{e7}\x05\x7c\x3f\x02\u{e7}\u{e8}\x07\
	\x02\x02\x03\u{e8}\x23\x03\x02\x02\x02\u{e9}\u{ea}\x05\x7e\x40\x02\u{ea}\
	\u{eb}\x07\x02\x02\x03\u{eb}\x25\x03\x02\x02\x02\u{ec}\u{ed}\x05\u{80}\x41\
	\x02\u{ed}\u{ee}\x07\x02\x02\x03\u{ee}\x27\x03\x02\x02\x02\u{ef}\u{f0}\x05\
	\u{82}\x42\x02\u{f0}\u{f1}\x07\x02\x02\x03\u{f1}\x29\x03\x02\x02\x02\u{f2}\
	\u{f3}\x05\u{84}\x43\x02\u{f3}\u{f4}\x07\x02\x02\x03\u{f4}\x2b\x03\x02\x02\
	\x02\u{f5}\u{f6}\x05\u{86}\x44\x02\u{f6}\u{f7}\x07\x02\x02\x03\u{f7}\x2d\
	\x03\x02\x02\x02\u{f8}\u{f9}\x05\u{88}\x45\x02\u{f9}\u{fa}\x07\x02\x02\x03\
	\u{fa}\x2f\x03\x02\x02\x02\u{fb}\u{fc}\x05\u{8a}\x46\x02\u{fc}\u{fd}\x07\
	\x02\x02\x03\u{fd}\x31\x03\x02\x02\x02\u{fe}\u{ff}\x05\u{8c}\x47\x02\u{ff}\
	\u{100}\x07\x02\x02\x03\u{100}\x33\x03\x02\x02\x02\u{101}\u{102}\x05\u{8e}\
	\x48\x02\u{102}\u{103}\x07\x02\x02\x03\u{103}\x35\x03\x02\x02\x02\u{104}\
	\u{105}\x05\u{90}\x49\x02\u{105}\u{106}\x07\x02\x02\x03\u{106}\x37\x03\x02\
	\x02\x02\u{107}\u{108}\x05\u{92}\x4a\x02\u{108}\u{109}\x07\x02\x02\x03\u{109}\
	\x39\x03\x02\x02\x02\u{10a}\u{10b}\x05\u{94}\x4b\x02\u{10b}\u{10c}\x07\x02\
	\x02\x03\u{10c}\x3b\x03\x02\x02\x02\u{10d}\u{10e}\x05\u{96}\x4c\x02\u{10e}\
	\u{10f}\x07\x02\x02\x03\u{10f}\x3d\x03\x02\x02\x02\u{110}\u{111}\x05\u{98}\
	\x4d\x02\u{111}\u{112}\x07\x02\x02\x03\u{112}\x3f\x03\x02\x02\x02\u{113}\
	\u{114}\x05\u{9a}\x4e\x02\u{114}\u{115}\x07\x02\x02\x03\u{115}\x41\x03\x02\
	\x02\x02\u{116}\u{117}\x05\u{9c}\x4f\x02\u{117}\u{118}\x07\x02\x02\x03\u{118}\
	\x43\x03\x02\x02\x02\u{119}\u{11a}\x05\u{9e}\x50\x02\u{11a}\u{11b}\x07\x02\
	\x02\x03\u{11b}\x45\x03\x02\x02\x02\u{11c}\u{11d}\x05\u{a0}\x51\x02\u{11d}\
	\u{11e}\x07\x02\x02\x03\u{11e}\x47\x03\x02\x02\x02\u{11f}\u{120}\x05\u{a2}\
	\x52\x02\u{120}\u{121}\x07\x02\x02\x03\u{121}\x49\x03\x02\x02\x02\u{122}\
	\u{123}\x05\u{a4}\x53\x02\u{123}\u{124}\x07\x02\x02\x03\u{124}\x4b\x03\x02\
	\x02\x02\u{125}\u{126}\x05\u{a6}\x54\x02\u{126}\u{127}\x07\x02\x02\x03\u{127}\
	\x4d\x03\x02\x02\x02\u{128}\u{129}\x05\u{a8}\x55\x02\u{129}\u{12a}\x07\x02\
	\x02\x03\u{12a}\x4f\x03\x02\x02\x02\u{12b}\u{12c}\x05\u{aa}\x56\x02\u{12c}\
	\u{12d}\x07\x02\x02\x03\u{12d}\x51\x03\x02\x02\x02\u{12e}\u{12f}\x05\u{ac}\
	\x57\x02\u{12f}\u{130}\x07\x02\x02\x03\u{130}\x53\x03\x02\x02\x02\u{131}\
	\u{132}\x05\u{ae}\x58\x02\u{132}\u{133}\x07\x02\x02\x03\u{133}\x55\x03\x02\
	\x02\x02\u{134}\u{135}\x05\u{b0}\x59\x02\u{135}\u{136}\x07\x02\x02\x03\u{136}\
	\x57\x03\x02\x02\x02\u{137}\u{138}\x05\u{b2}\x5a\x02\u{138}\u{139}\x07\x02\
	\x02\x03\u{139}\x59\x03\x02\x02\x02\u{13a}\u{13b}\x05\u{b4}\x5b\x02\u{13b}\
	\u{13c}\x07\x02\x02\x03\u{13c}\x5b\x03\x02\x02\x02\u{13d}\u{143}\x03\x02\
	\x02\x02\u{13e}\u{143}\x07\x42\x02\x02\u{13f}\u{140}\x07\x42\x02\x02\u{140}\
	\u{141}\x07\x03\x02\x02\u{141}\u{143}\x05\x5c\x2f\x02\u{142}\u{13d}\x03\
	\x02\x02\x02\u{142}\u{13e}\x03\x02\x02\x02\u{142}\u{13f}\x03\x02\x02\x02\
	\u{143}\x5d\x03\x02\x02\x02\u{144}\u{145}\x05\x60\x31\x02\u{145}\u{146}\
	\x05\x66\x34\x02\u{146}\u{147}\x05\x6a\x36\x02\u{147}\x5f\x03\x02\x02\x02\
	\u{148}\u{149}\x07\x30\x02\x02\u{149}\u{14a}\x07\x26\x02\x02\u{14a}\u{14b}\
	\x07\x04\x02\x02\u{14b}\x61\x03\x02\x02\x02\u{14c}\u{14d}\x07\x28\x02\x02\
	\u{14d}\u{14e}\x07\x3e\x02\x02\u{14e}\u{14f}\x05\x64\x33\x02\u{14f}\x63\
	\x03\x02\x02\x02\u{150}\u{156}\x03\x02\x02\x02\u{151}\u{156}\x07\x43\x02\
	\x02\u{152}\u{153}\x07\x43\x02\x02\u{153}\u{154}\x07\x03\x02\x02\u{154}\
	\u{156}\x05\x64\x33\x02\u{155}\u{150}\x03\x02\x02\x02\u{155}\u{151}\x03\
	\x02\x02\x02\u{155}\u{152}\x03\x02\x02\x02\u{156}\x65\x03\x02\x02\x02\u{157}\
	\u{15e}\x08\x34\x01\x02\u{158}\u{159}\x0c\x03\x02\x02\u{159}\u{15a}\x05\
	\x62\x32\x02\u{15a}\u{15b}\x07\x04\x02\x02\u{15b}\u{15d}\x03\x02\x02\x02\
	\u{15c}\u{158}\x03\x02\x02\x02\u{15d}\u{160}\x03\x02\x02\x02\u{15e}\u{15c}\
	\x03\x02\x02\x02\u{15e}\u{15f}\x03\x02\x02\x02\u{15f}\x67\x03\x02\x02\x02\
	\u{160}\u{15e}\x03\x02\x02\x02\u{161}\u{162}\x05\x72\x3a\x02\u{162}\u{163}\
	\x07\x2b\x02\x02\u{163}\u{164}\x07\x42\x02\x02\u{164}\u{165}\x07\x05\x02\
	\x02\u{165}\u{166}\x05\x76\x3c\x02\u{166}\u{167}\x07\x06\x02\x02\u{167}\
	\u{168}\x05\x78\x3d\x02\u{168}\u{169}\x05\x7a\x3e\x02\u{169}\u{16a}\x07\
	\x07\x02\x02\u{16a}\u{16b}\x05\x6a\x36\x02\u{16b}\u{16c}\x07\x36\x02\x02\
	\u{16c}\u{16d}\x05\x7c\x3f\x02\u{16d}\u{16e}\x07\x04\x02\x02\u{16e}\u{16f}\
	\x07\x08\x02\x02\u{16f}\u{175}\x03\x02\x02\x02\u{170}\u{171}\x07\x3b\x02\
	\x02\u{171}\u{172}\x07\x42\x02\x02\u{172}\u{173}\x07\x09\x02\x02\u{173}\
	\u{175}\x05\u{a2}\x52\x02\u{174}\u{161}\x03\x02\x02\x02\u{174}\u{170}\x03\
	\x02\x02\x02\u{175}\x69\x03\x02\x02\x02\u{176}\u{17b}\x08\x36\x01\x02\u{177}\
	\u{178}\x0c\x03\x02\x02\u{178}\u{17a}\x05\x68\x35\x02\u{179}\u{177}\x03\
	\x02\x02\x02\u{17a}\u{17d}\x03\x02\x02\x02\u{17b}\u{179}\x03\x02\x02\x02\
	\u{17b}\u{17c}\x03\x02\x02\x02\u{17c}\x6b\x03\x02\x02\x02\u{17d}\u{17b}\
	\x03\x02\x02\x02\u{17e}\u{17f}\x05\x68\x35\x02\u{17f}\x6d\x03\x02\x02\x02\
	\u{180}\u{187}\x08\x38\x01\x02\u{181}\u{182}\x0c\x03\x02\x02\u{182}\u{183}\
	\x05\x6c\x37\x02\u{183}\u{184}\x07\x04\x02\x02\u{184}\u{186}\x03\x02\x02\
	\x02\u{185}\u{181}\x03\x02\x02\x02\u{186}\u{189}\x03\x02\x02\x02\u{187}\
	\u{185}\x03\x02\x02\x02\u{187}\u{188}\x03\x02\x02\x02\u{188}\x6f\x03\x02\
	\x02\x02\u{189}\u{187}\x03\x02\x02\x02\u{18a}\u{18b}\x07\x2f\x02\x02\u{18b}\
	\x71\x03\x02\x02\x02\u{18c}\u{191}\x08\x3a\x01\x02\u{18d}\u{18e}\x0c\x03\
	\x02\x02\u{18e}\u{190}\x05\x70\x39\x02\u{18f}\u{18d}\x03\x02\x02\x02\u{190}\
	\u{193}\x03\x02\x02\x02\u{191}\u{18f}\x03\x02\x02\x02\u{191}\u{192}\x03\
	\x02\x02\x02\u{192}\x73\x03\x02\x02\x02\u{193}\u{191}\x03\x02\x02\x02\u{194}\
	\u{195}\x07\x42\x02\x02\u{195}\u{196}\x07\x0a\x02\x02\u{196}\u{197}\x05\
	\u{a2}\x52\x02\u{197}\x75\x03\x02\x02\x02\u{198}\u{19f}\x03\x02\x02\x02\
	\u{199}\u{19f}\x05\x74\x3b\x02\u{19a}\u{19b}\x05\x74\x3b\x02\u{19b}\u{19c}\
	\x07\x03\x02\x02\u{19c}\u{19d}\x05\x76\x3c\x02\u{19d}\u{19f}\x03\x02\x02\
	\x02\u{19e}\u{198}\x03\x02\x02\x02\u{19e}\u{199}\x03\x02\x02\x02\u{19e}\
	\u{19a}\x03\x02\x02\x02\u{19f}\x77\x03\x02\x02\x02\u{1a0}\u{1a4}\x03\x02\
	\x02\x02\u{1a1}\u{1a2}\x07\x0b\x02\x02\u{1a2}\u{1a4}\x05\u{a2}\x52\x02\u{1a3}\
	\u{1a0}\x03\x02\x02\x02\u{1a3}\u{1a1}\x03\x02\x02\x02\u{1a4}\x79\x03\x02\
	\x02\x02\u{1a5}\u{1a9}\x03\x02\x02\x02\u{1a6}\u{1a7}\x07\x39\x02\x02\u{1a7}\
	\u{1a9}\x05\u{aa}\x56\x02\u{1a8}\u{1a5}\x03\x02\x02\x02\u{1a8}\u{1a6}\x03\
	\x02\x02\x02\u{1a9}\x7b\x03\x02\x02\x02\u{1aa}\u{1ab}\x07\x2d\x02\x02\u{1ab}\
	\u{1ac}\x05\x7c\x3f\x02\u{1ac}\u{1ad}\x07\x38\x02\x02\u{1ad}\u{1ae}\x05\
	\x7c\x3f\x02\u{1ae}\u{1af}\x07\x27\x02\x02\u{1af}\u{1b0}\x05\x7c\x3f\x02\
	\u{1b0}\u{1ba}\x03\x02\x02\x02\u{1b1}\u{1b2}\x07\x31\x02\x02\u{1b2}\u{1b3}\
	\x07\x42\x02\x02\u{1b3}\u{1b4}\x07\x09\x02\x02\u{1b4}\u{1b5}\x05\x7c\x3f\
	\x02\u{1b5}\u{1b6}\x07\x2e\x02\x02\u{1b6}\u{1b7}\x05\x7c\x3f\x02\u{1b7}\
	\u{1ba}\x03\x02\x02\x02\u{1b8}\u{1ba}\x05\u{96}\x4c\x02\u{1b9}\u{1aa}\x03\
	\x02\x02\x02\u{1b9}\u{1b1}\x03\x02\x02\x02\u{1b9}\u{1b8}\x03\x02\x02\x02\
	\u{1ba}\x7d\x03\x02\x02\x02\u{1bb}\u{1c2}\x03\x02\x02\x02\u{1bc}\u{1c2}\
	\x05\x7c\x3f\x02\u{1bd}\u{1be}\x05\x7c\x3f\x02\u{1be}\u{1bf}\x07\x03\x02\
	\x02\u{1bf}\u{1c0}\x05\x7e\x40\x02\u{1c0}\u{1c2}\x03\x02\x02\x02\u{1c1}\
	\u{1bb}\x03\x02\x02\x02\u{1c1}\u{1bc}\x03\x02\x02\x02\u{1c1}\u{1bd}\x03\
	\x02\x02\x02\u{1c2}\x7f\x03\x02\x02\x02\u{1c3}\u{1c4}\x05\u{8a}\x46\x02\
	\u{1c4}\u{1c5}\x07\x0c\x02\x02\u{1c5}\u{1c6}\x05\x7c\x3f\x02\u{1c6}\u{81}\
	\x03\x02\x02\x02\u{1c7}\u{1ce}\x03\x02\x02\x02\u{1c8}\u{1ce}\x05\u{80}\x41\
	\x02\u{1c9}\u{1ca}\x05\u{80}\x41\x02\u{1ca}\u{1cb}\x07\x04\x02\x02\u{1cb}\
	\u{1cc}\x05\u{82}\x42\x02\u{1cc}\u{1ce}\x03\x02\x02\x02\u{1cd}\u{1c7}\x03\
	\x02\x02\x02\u{1cd}\u{1c8}\x03\x02\x02\x02\u{1cd}\u{1c9}\x03\x02\x02\x02\
	\u{1ce}\u{83}\x03\x02\x02\x02\u{1cf}\u{1d3}\x03\x02\x02\x02\u{1d0}\u{1d1}\
	\x07\x0a\x02\x02\u{1d1}\u{1d3}\x05\u{a2}\x52\x02\u{1d2}\u{1cf}\x03\x02\x02\
	\x02\u{1d2}\u{1d0}\x03\x02\x02\x02\u{1d3}\u{85}\x03\x02\x02\x02\u{1d4}\u{1d8}\
	\x03\x02\x02\x02\u{1d5}\u{1d6}\x07\x09\x02\x02\u{1d6}\u{1d8}\x05\u{8a}\x46\
	\x02\u{1d7}\u{1d4}\x03\x02\x02\x02\u{1d7}\u{1d5}\x03\x02\x02\x02\u{1d8}\
	\u{87}\x03\x02\x02\x02\u{1d9}\u{1dd}\x03\x02\x02\x02\u{1da}\u{1db}\x07\x09\
	\x02\x02\u{1db}\u{1dd}\x05\x7c\x3f\x02\u{1dc}\u{1d9}\x03\x02\x02\x02\u{1dc}\
	\u{1da}\x03\x02\x02\x02\u{1dd}\u{89}\x03\x02\x02\x02\u{1de}\u{1df}\x07\x0d\
	\x02\x02\u{1df}\u{1e0}\x07\x42\x02\x02\u{1e0}\u{1e1}\x05\u{86}\x44\x02\u{1e1}\
	\u{1e2}\x07\x0e\x02\x02\u{1e2}\u{205}\x03\x02\x02\x02\u{1e3}\u{1e4}\x07\
	\x07\x02\x02\u{1e4}\u{1e5}\x05\u{8c}\x47\x02\u{1e5}\u{1e6}\x07\x08\x02\x02\
	\u{1e6}\u{205}\x03\x02\x02\x02\u{1e7}\u{1e8}\x07\x35\x02\x02\u{1e8}\u{1e9}\
	\x07\x07\x02\x02\u{1e9}\u{1ea}\x05\u{90}\x49\x02\u{1ea}\u{1eb}\x07\x08\x02\
	\x02\u{1eb}\u{205}\x03\x02\x02\x02\u{1ec}\u{1ed}\x07\x0f\x02\x02\u{1ed}\
	\u{1ee}\x05\u{8c}\x47\x02\u{1ee}\u{1ef}\x07\x10\x02\x02\u{1ef}\u{205}\x03\
	\x02\x02\x02\u{1f0}\u{1f1}\x07\x25\x02\x02\u{1f1}\u{1f2}\x07\x05\x02\x02\
	\u{1f2}\u{1f3}\x05\u{8a}\x46\x02\u{1f3}\u{1f4}\x07\x03\x02\x02\u{1f4}\u{1f5}\
	\x05\u{8a}\x46\x02\u{1f5}\u{1f6}\x07\x06\x02\x02\u{1f6}\u{205}\x03\x02\x02\
	\x02\u{1f7}\u{205}\x07\x29\x02\x02\u{1f8}\u{205}\x07\x3a\x02\x02\u{1f9}\
	\u{205}\x07\x44\x02\x02\u{1fa}\u{1fb}\x07\x37\x02\x02\u{1fb}\u{1fc}\x07\
	\x05\x02\x02\u{1fc}\u{1fd}\x05\u{8a}\x46\x02\u{1fd}\u{1fe}\x07\x06\x02\x02\
	\u{1fe}\u{205}\x03\x02\x02\x02\u{1ff}\u{205}\x07\x42\x02\x02\u{200}\u{201}\
	\x07\x05\x02\x02\u{201}\u{202}\x05\u{8a}\x46\x02\u{202}\u{203}\x07\x06\x02\
	\x02\u{203}\u{205}\x03\x02\x02\x02\u{204}\u{1de}\x03\x02\x02\x02\u{204}\
	\u{1e3}\x03\x02\x02\x02\u{204}\u{1e7}\x03\x02\x02\x02\u{204}\u{1ec}\x03\
	\x02\x02\x02\u{204}\u{1f0}\x03\x02\x02\x02\u{204}\u{1f7}\x03\x02\x02\x02\
	\u{204}\u{1f8}\x03\x02\x02\x02\u{204}\u{1f9}\x03\x02\x02\x02\u{204}\u{1fa}\
	\x03\x02\x02\x02\u{204}\u{1ff}\x03\x02\x02\x02\u{204}\u{200}\x03\x02\x02\
	\x02\u{205}\u{8b}\x03\x02\x02\x02\u{206}\u{20d}\x03\x02\x02\x02\u{207}\u{20d}\
	\x05\u{8a}\x46\x02\u{208}\u{209}\x05\u{8a}\x46\x02\u{209}\u{20a}\x07\x03\
	\x02\x02\u{20a}\u{20b}\x05\u{8c}\x47\x02\u{20b}\u{20d}\x03\x02\x02\x02\u{20c}\
	\u{206}\x03\x02\x02\x02\u{20c}\u{207}\x03\x02\x02\x02\u{20c}\u{208}\x03\
	\x02\x02\x02\u{20d}\u{8d}\x03\x02\x02\x02\u{20e}\u{20f}\x07\x42\x02\x02\
	\u{20f}\u{210}\x07\x09\x02\x02\u{210}\u{211}\x05\u{8a}\x46\x02\u{211}\u{8f}\
	\x03\x02\x02\x02\u{212}\u{219}\x03\x02\x02\x02\u{213}\u{219}\x05\u{8e}\x48\
	\x02\u{214}\u{215}\x05\u{8e}\x48\x02\u{215}\u{216}\x07\x03\x02\x02\u{216}\
	\u{217}\x05\u{90}\x49\x02\u{217}\u{219}\x03\x02\x02\x02\u{218}\u{212}\x03\
	\x02\x02\x02\u{218}\u{213}\x03\x02\x02\x02\u{218}\u{214}\x03\x02\x02\x02\
	\u{219}\u{91}\x03\x02\x02\x02\u{21a}\u{21b}\x07\x42\x02\x02\u{21b}\u{21c}\
	\x07\x09\x02\x02\u{21c}\u{21d}\x05\x7c\x3f\x02\u{21d}\u{93}\x03\x02\x02\
	\x02\u{21e}\u{225}\x03\x02\x02\x02\u{21f}\u{225}\x05\u{92}\x4a\x02\u{220}\
	\u{221}\x05\u{92}\x4a\x02\u{221}\u{222}\x07\x03\x02\x02\u{222}\u{223}\x05\
	\u{94}\x4b\x02\u{223}\u{225}\x03\x02\x02\x02\u{224}\u{21e}\x03\x02\x02\x02\
	\u{224}\u{21f}\x03\x02\x02\x02\u{224}\u{220}\x03\x02\x02\x02\u{225}\u{95}\
	\x03\x02\x02\x02\u{226}\u{227}\x05\u{98}\x4d\x02\u{227}\u{228}\x07\x11\x02\
	\x02\u{228}\u{229}\x05\u{98}\x4d\x02\u{229}\u{240}\x03\x02\x02\x02\u{22a}\
	\u{22b}\x05\u{98}\x4d\x02\u{22b}\u{22c}\x07\x12\x02\x02\u{22c}\u{22d}\x05\
	\u{98}\x4d\x02\u{22d}\u{240}\x03\x02\x02\x02\u{22e}\u{22f}\x05\u{98}\x4d\
	\x02\u{22f}\u{230}\x07\x13\x02\x02\u{230}\u{231}\x05\u{98}\x4d\x02\u{231}\
	\u{240}\x03\x02\x02\x02\u{232}\u{233}\x05\u{98}\x4d\x02\u{233}\u{234}\x07\
	\x14\x02\x02\u{234}\u{235}\x05\u{98}\x4d\x02\u{235}\u{240}\x03\x02\x02\x02\
	\u{236}\u{237}\x05\u{98}\x4d\x02\u{237}\u{238}\x07\x15\x02\x02\u{238}\u{239}\
	\x05\u{98}\x4d\x02\u{239}\u{240}\x03\x02\x02\x02\u{23a}\u{23b}\x05\u{98}\
	\x4d\x02\u{23b}\u{23c}\x07\x16\x02\x02\u{23c}\u{23d}\x05\u{98}\x4d\x02\u{23d}\
	\u{240}\x03\x02\x02\x02\u{23e}\u{240}\x05\u{98}\x4d\x02\u{23f}\u{226}\x03\
	\x02\x02\x02\u{23f}\u{22a}\x03\x02\x02\x02\u{23f}\u{22e}\x03\x02\x02\x02\
	\u{23f}\u{232}\x03\x02\x02\x02\u{23f}\u{236}\x03\x02\x02\x02\u{23f}\u{23a}\
	\x03\x02\x02\x02\u{23f}\u{23e}\x03\x02\x02\x02\u{240}\u{97}\x03\x02\x02\
	\x02\u{241}\u{242}\x08\x4d\x01\x02\u{242}\u{243}\x07\x2b\x02\x02\u{243}\
	\u{244}\x07\x05\x02\x02\u{244}\u{245}\x05\x76\x3c\x02\u{245}\u{246}\x07\
	\x06\x02\x02\u{246}\u{247}\x07\x07\x02\x02\u{247}\u{248}\x07\x36\x02\x02\
	\u{248}\u{249}\x05\x7c\x3f\x02\u{249}\u{24a}\x07\x04\x02\x02\u{24a}\u{24b}\
	\x07\x08\x02\x02\u{24b}\u{266}\x03\x02\x02\x02\u{24c}\u{24d}\x07\x07\x02\
	\x02\u{24d}\u{24e}\x05\x7e\x40\x02\u{24e}\u{24f}\x07\x08\x02\x02\u{24f}\
	\u{266}\x03\x02\x02\x02\u{250}\u{251}\x07\x35\x02\x02\u{251}\u{252}\x07\
	\x07\x02\x02\u{252}\u{253}\x05\u{94}\x4b\x02\u{253}\u{254}\x07\x08\x02\x02\
	\u{254}\u{266}\x03\x02\x02\x02\u{255}\u{256}\x07\x0d\x02\x02\u{256}\u{257}\
	\x07\x42\x02\x02\u{257}\u{258}\x05\u{88}\x45\x02\u{258}\u{259}\x07\x0e\x02\
	\x02\u{259}\u{266}\x03\x02\x02\x02\u{25a}\u{25b}\x07\x32\x02\x02\u{25b}\
	\u{25c}\x05\u{96}\x4c\x02\u{25c}\u{25d}\x07\x07\x02\x02\u{25d}\u{25e}\x05\
	\u{82}\x42\x02\u{25e}\u{25f}\x07\x08\x02\x02\u{25f}\u{266}\x03\x02\x02\x02\
	\u{260}\u{261}\x07\x0f\x02\x02\u{261}\u{262}\x05\x7e\x40\x02\u{262}\u{263}\
	\x07\x10\x02\x02\u{263}\u{266}\x03\x02\x02\x02\u{264}\u{266}\x05\u{9a}\x4e\
	\x02\u{265}\u{241}\x03\x02\x02\x02\u{265}\u{24c}\x03\x02\x02\x02\u{265}\
	\u{250}\x03\x02\x02\x02\u{265}\u{255}\x03\x02\x02\x02\u{265}\u{25a}\x03\
	\x02\x02\x02\u{265}\u{260}\x03\x02\x02\x02\u{265}\u{264}\x03\x02\x02\x02\
	\u{266}\u{272}\x03\x02\x02\x02\u{267}\u{268}\x0c\x0c\x02\x02\u{268}\u{269}\
	\x07\x24\x02\x02\u{269}\u{271}\x05\u{a2}\x52\x02\u{26a}\u{26b}\x0c\x05\x02\
	\x02\u{26b}\u{26c}\x07\x17\x02\x02\u{26c}\u{271}\x05\u{9a}\x4e\x02\u{26d}\
	\u{26e}\x0c\x04\x02\x02\u{26e}\u{26f}\x07\x34\x02\x02\u{26f}\u{271}\x05\
	\u{9a}\x4e\x02\u{270}\u{267}\x03\x02\x02\x02\u{270}\u{26a}\x03\x02\x02\x02\
	\u{270}\u{26d}\x03\x02\x02\x02\u{271}\u{274}\x03\x02\x02\x02\u{272}\u{270}\
	\x03\x02\x02\x02\u{272}\u{273}\x03\x02\x02\x02\u{273}\u{99}\x03\x02\x02\
	\x02\u{274}\u{272}\x03\x02\x02\x02\u{275}\u{276}\x08\x4e\x01\x02\u{276}\
	\u{277}\x05\u{9c}\x4f\x02\u{277}\u{280}\x03\x02\x02\x02\u{278}\u{279}\x0c\
	\x05\x02\x02\u{279}\u{27a}\x07\x18\x02\x02\u{27a}\u{27f}\x05\u{9c}\x4f\x02\
	\u{27b}\u{27c}\x0c\x04\x02\x02\u{27c}\u{27d}\x07\x23\x02\x02\u{27d}\u{27f}\
	\x05\u{9c}\x4f\x02\u{27e}\u{278}\x03\x02\x02\x02\u{27e}\u{27b}\x03\x02\x02\
	\x02\u{27f}\u{282}\x03\x02\x02\x02\u{280}\u{27e}\x03\x02\x02\x02\u{280}\
	\u{281}\x03\x02\x02\x02\u{281}\u{9b}\x03\x02\x02\x02\u{282}\u{280}\x03\x02\
	\x02\x02\u{283}\u{284}\x08\x4f\x01\x02\u{284}\u{285}\x05\u{9e}\x50\x02\u{285}\
	\u{28d}\x03\x02\x02\x02\u{286}\u{287}\x0c\x04\x02\x02\u{287}\u{288}\x07\
	\x05\x02\x02\u{288}\u{289}\x05\x7e\x40\x02\u{289}\u{28a}\x07\x06\x02\x02\
	\u{28a}\u{28c}\x03\x02\x02\x02\u{28b}\u{286}\x03\x02\x02\x02\u{28c}\u{28f}\
	\x03\x02\x02\x02\u{28d}\u{28b}\x03\x02\x02\x02\u{28d}\u{28e}\x03\x02\x02\
	\x02\u{28e}\u{9d}\x03\x02\x02\x02\u{28f}\u{28d}\x03\x02\x02\x02\u{290}\u{291}\
	\x07\x25\x02\x02\u{291}\u{292}\x07\x05\x02\x02\u{292}\u{293}\x05\x7c\x3f\
	\x02\u{293}\u{294}\x07\x03\x02\x02\u{294}\u{295}\x05\x7c\x3f\x02\u{295}\
	\u{296}\x07\x06\x02\x02\u{296}\u{2d6}\x03\x02\x02\x02\u{297}\u{298}\x07\
	\x19\x02\x02\u{298}\u{299}\x07\x05\x02\x02\u{299}\u{29a}\x05\x7c\x3f\x02\
	\u{29a}\u{29b}\x07\x06\x02\x02\u{29b}\u{2d6}\x03\x02\x02\x02\u{29c}\u{29d}\
	\x07\x1a\x02\x02\u{29d}\u{29e}\x07\x05\x02\x02\u{29e}\u{29f}\x05\x7c\x3f\
	\x02\u{29f}\u{2a0}\x07\x06\x02\x02\u{2a0}\u{2d6}\x03\x02\x02\x02\u{2a1}\
	\u{2a2}\x07\x1b\x02\x02\u{2a2}\u{2a3}\x07\x05\x02\x02\u{2a3}\u{2a4}\x05\
	\x7c\x3f\x02\u{2a4}\u{2a5}\x07\x06\x02\x02\u{2a5}\u{2d6}\x03\x02\x02\x02\
	\u{2a6}\u{2a7}\x07\x37\x02\x02\u{2a7}\u{2a8}\x07\x05\x02\x02\u{2a8}\u{2a9}\
	\x05\x7c\x3f\x02\u{2a9}\u{2aa}\x07\x06\x02\x02\u{2aa}\u{2d6}\x03\x02\x02\
	\x02\u{2ab}\u{2ac}\x07\x33\x02\x02\u{2ac}\u{2ad}\x07\x05\x02\x02\u{2ad}\
	\u{2ae}\x05\x7c\x3f\x02\u{2ae}\u{2af}\x07\x06\x02\x02\u{2af}\u{2d6}\x03\
	\x02\x02\x02\u{2b0}\u{2b1}\x07\x1c\x02\x02\u{2b1}\u{2b2}\x07\x05\x02\x02\
	\u{2b2}\u{2b3}\x05\x7c\x3f\x02\u{2b3}\u{2b4}\x07\x06\x02\x02\u{2b4}\u{2d6}\
	\x03\x02\x02\x02\u{2b5}\u{2b6}\x07\x1d\x02\x02\u{2b6}\u{2b7}\x07\x05\x02\
	\x02\u{2b7}\u{2b8}\x05\x7c\x3f\x02\u{2b8}\u{2b9}\x07\x06\x02\x02\u{2b9}\
	\u{2d6}\x03\x02\x02\x02\u{2ba}\u{2bb}\x07\x2a\x02\x02\u{2bb}\u{2bc}\x07\
	\x05\x02\x02\u{2bc}\u{2bd}\x05\x7c\x3f\x02\u{2bd}\u{2be}\x07\x06\x02\x02\
	\u{2be}\u{2d6}\x03\x02\x02\x02\u{2bf}\u{2c0}\x07\x1e\x02\x02\u{2c0}\u{2c1}\
	\x07\x05\x02\x02\u{2c1}\u{2c2}\x05\x7c\x3f\x02\u{2c2}\u{2c3}\x07\x03\x02\
	\x02\u{2c3}\u{2c4}\x05\x7c\x3f\x02\u{2c4}\u{2c5}\x07\x03\x02\x02\u{2c5}\
	\u{2c6}\x05\x7c\x3f\x02\u{2c6}\u{2c7}\x07\x06\x02\x02\u{2c7}\u{2d6}\x03\
	\x02\x02\x02\u{2c8}\u{2c9}\x07\x2c\x02\x02\u{2c9}\u{2ca}\x07\x0f\x02\x02\
	\u{2ca}\u{2cb}\x05\u{a2}\x52\x02\u{2cb}\u{2cc}\x07\x10\x02\x02\u{2cc}\u{2cd}\
	\x05\u{a0}\x51\x02\u{2cd}\u{2d6}\x03\x02\x02\x02\u{2ce}\u{2cf}\x07\x3c\x02\
	\x02\u{2cf}\u{2d0}\x07\x0f\x02\x02\u{2d0}\u{2d1}\x05\u{a2}\x52\x02\u{2d1}\
	\u{2d2}\x07\x10\x02\x02\u{2d2}\u{2d3}\x05\u{a0}\x51\x02\u{2d3}\u{2d6}\x03\
	\x02\x02\x02\u{2d4}\u{2d6}\x05\u{a0}\x51\x02\u{2d5}\u{290}\x03\x02\x02\x02\
	\u{2d5}\u{297}\x03\x02\x02\x02\u{2d5}\u{29c}\x03\x02\x02\x02\u{2d5}\u{2a1}\
	\x03\x02\x02\x02\u{2d5}\u{2a6}\x03\x02\x02\x02\u{2d5}\u{2ab}\x03\x02\x02\
	\x02\u{2d5}\u{2b0}\x03\x02\x02\x02\u{2d5}\u{2b5}\x03\x02\x02\x02\u{2d5}\
	\u{2ba}\x03\x02\x02\x02\u{2d5}\u{2bf}\x03\x02\x02\x02\u{2d5}\u{2c8}\x03\
	\x02\x02\x02\u{2d5}\u{2ce}\x03\x02\x02\x02\u{2d5}\u{2d4}\x03\x02\x02\x02\
	\u{2d6}\u{9f}\x03\x02\x02\x02\u{2d7}\u{2d8}\x08\x51\x01\x02\u{2d8}\u{2e1}\
	\x07\x3a\x02\x02\u{2d9}\u{2e1}\x07\x29\x02\x02\u{2da}\u{2e1}\x07\x44\x02\
	\x02\u{2db}\u{2e1}\x07\x42\x02\x02\u{2dc}\u{2dd}\x07\x05\x02\x02\u{2dd}\
	\u{2de}\x05\x7c\x3f\x02\u{2de}\u{2df}\x07\x06\x02\x02\u{2df}\u{2e1}\x03\
	\x02\x02\x02\u{2e0}\u{2d7}\x03\x02\x02\x02\u{2e0}\u{2d9}\x03\x02\x02\x02\
	\u{2e0}\u{2da}\x03\x02\x02\x02\u{2e0}\u{2db}\x03\x02\x02\x02\u{2e0}\u{2dc}\
	\x03\x02\x02\x02\u{2e1}\u{2ea}\x03\x02\x02\x02\u{2e2}\u{2e3}\x0c\x09\x02\
	\x02\u{2e3}\u{2e4}\x07\x1f\x02\x02\u{2e4}\u{2e9}\x07\x42\x02\x02\u{2e5}\
	\u{2e6}\x0c\x08\x02\x02\u{2e6}\u{2e7}\x07\x1f\x02\x02\u{2e7}\u{2e9}\x07\
	\x44\x02\x02\u{2e8}\u{2e2}\x03\x02\x02\x02\u{2e8}\u{2e5}\x03\x02\x02\x02\
	\u{2e9}\u{2ec}\x03\x02\x02\x02\u{2ea}\u{2e8}\x03\x02\x02\x02\u{2ea}\u{2eb}\
	\x03\x02\x02\x02\u{2eb}\u{a1}\x03\x02\x02\x02\u{2ec}\u{2ea}\x03\x02\x02\
	\x02\u{2ed}\u{2ee}\x07\x2b\x02\x02\u{2ee}\u{2ef}\x07\x05\x02\x02\u{2ef}\
	\u{2f0}\x05\u{aa}\x56\x02\u{2f0}\u{2f1}\x07\x06\x02\x02\u{2f1}\u{2f2}\x07\
	\x0b\x02\x02\u{2f2}\u{2f3}\x05\u{a2}\x52\x02\u{2f3}\u{2fa}\x03\x02\x02\x02\
	\u{2f4}\u{2f5}\x07\x3f\x02\x02\u{2f5}\u{2f6}\x07\x42\x02\x02\u{2f6}\u{2f7}\
	\x07\x1f\x02\x02\u{2f7}\u{2fa}\x05\u{a2}\x52\x02\u{2f8}\u{2fa}\x05\u{a4}\
	\x53\x02\u{2f9}\u{2ed}\x03\x02\x02\x02\u{2f9}\u{2f4}\x03\x02\x02\x02\u{2f9}\
	\u{2f8}\x03\x02\x02\x02\u{2fa}\u{a3}\x03\x02\x02\x02\u{2fb}\u{2fc}\x05\u{a6}\
	\x54\x02\u{2fc}\u{2fd}\x07\x17\x02\x02\u{2fd}\u{2fe}\x05\u{a6}\x54\x02\u{2fe}\
	\u{301}\x03\x02\x02\x02\u{2ff}\u{301}\x05\u{a6}\x54\x02\u{300}\u{2fb}\x03\
	\x02\x02\x02\u{300}\u{2ff}\x03\x02\x02\x02\u{301}\u{a5}\x03\x02\x02\x02\
	\u{302}\u{303}\x07\x07\x02\x02\u{303}\u{304}\x05\u{aa}\x56\x02\u{304}\u{305}\
	\x07\x08\x02\x02\u{305}\u{316}\x03\x02\x02\x02\u{306}\u{307}\x07\x35\x02\
	\x02\u{307}\u{308}\x07\x07\x02\x02\u{308}\u{309}\x05\u{b2}\x5a\x02\u{309}\
	\u{30a}\x07\x08\x02\x02\u{30a}\u{316}\x03\x02\x02\x02\u{30b}\u{30c}\x07\
	\x3d\x02\x02\u{30c}\u{30d}\x07\x0d\x02\x02\u{30d}\u{30e}\x05\u{ae}\x58\x02\
	\u{30e}\u{30f}\x07\x0e\x02\x02\u{30f}\u{316}\x03\x02\x02\x02\u{310}\u{311}\
	\x07\x0f\x02\x02\u{311}\u{312}\x05\u{a2}\x52\x02\u{312}\u{313}\x07\x10\x02\
	\x02\u{313}\u{316}\x03\x02\x02\x02\u{314}\u{316}\x05\u{a8}\x55\x02\u{315}\
	\u{302}\x03\x02\x02\x02\u{315}\u{306}\x03\x02\x02\x02\u{315}\u{30b}\x03\
	\x02\x02\x02\u{315}\u{310}\x03\x02\x02\x02\u{315}\u{314}\x03\x02\x02\x02\
	\u{316}\u{a7}\x03\x02\x02\x02\u{317}\u{320}\x07\x20\x02\x02\u{318}\u{320}\
	\x07\x21\x02\x02\u{319}\u{320}\x07\x22\x02\x02\u{31a}\u{320}\x07\x42\x02\
	\x02\u{31b}\u{31c}\x07\x05\x02\x02\u{31c}\u{31d}\x05\u{a2}\x52\x02\u{31d}\
	\u{31e}\x07\x06\x02\x02\u{31e}\u{320}\x03\x02\x02\x02\u{31f}\u{317}\x03\
	\x02\x02\x02\u{31f}\u{318}\x03\x02\x02\x02\u{31f}\u{319}\x03\x02\x02\x02\
	\u{31f}\u{31a}\x03\x02\x02\x02\u{31f}\u{31b}\x03\x02\x02\x02\u{320}\u{a9}\
	\x03\x02\x02\x02\u{321}\u{328}\x03\x02\x02\x02\u{322}\u{328}\x05\u{a2}\x52\
	\x02\u{323}\u{324}\x05\u{a2}\x52\x02\u{324}\u{325}\x07\x03\x02\x02\u{325}\
	\u{326}\x05\u{aa}\x56\x02\u{326}\u{328}\x03\x02\x02\x02\u{327}\u{321}\x03\
	\x02\x02\x02\u{327}\u{322}\x03\x02\x02\x02\u{327}\u{323}\x03\x02\x02\x02\
	\u{328}\u{ab}\x03\x02\x02\x02\u{329}\u{32a}\x07\x42\x02\x02\u{32a}\u{32b}\
	\x05\u{84}\x43\x02\u{32b}\u{ad}\x03\x02\x02\x02\u{32c}\u{333}\x03\x02\x02\
	\x02\u{32d}\u{333}\x05\u{ac}\x57\x02\u{32e}\u{32f}\x05\u{ac}\x57\x02\u{32f}\
	\u{330}\x07\x03\x02\x02\u{330}\u{331}\x05\u{ae}\x58\x02\u{331}\u{333}\x03\
	\x02\x02\x02\u{332}\u{32c}\x03\x02\x02\x02\u{332}\u{32d}\x03\x02\x02\x02\
	\u{332}\u{32e}\x03\x02\x02\x02\u{333}\u{af}\x03\x02\x02\x02\u{334}\u{335}\
	\x07\x42\x02\x02\u{335}\u{336}\x07\x0a\x02\x02\u{336}\u{337}\x05\u{a2}\x52\
	\x02\u{337}\u{b1}\x03\x02\x02\x02\u{338}\u{33f}\x03\x02\x02\x02\u{339}\u{33f}\
	\x05\u{b0}\x59\x02\u{33a}\u{33b}\x05\u{b0}\x59\x02\u{33b}\u{33c}\x07\x03\
	\x02\x02\u{33c}\u{33d}\x05\u{b2}\x5a\x02\u{33d}\u{33f}\x03\x02\x02\x02\u{33e}\
	\u{338}\x03\x02\x02\x02\u{33e}\u{339}\x03\x02\x02\x02\u{33e}\u{33a}\x03\
	\x02\x02\x02\u{33f}\u{b3}\x03\x02\x02\x02\u{340}\u{341}\x05\x7c\x3f\x02\
	\u{341}\u{342}\x07\x0a\x02\x02\u{342}\u{343}\x05\u{a2}\x52\x02\u{343}\u{b5}\
	\x03\x02\x02\x02\x28\u{142}\u{155}\u{15e}\u{174}\u{17b}\u{187}\u{191}\u{19e}\
	\u{1a3}\u{1a8}\u{1b9}\u{1c1}\u{1cd}\u{1d2}\u{1d7}\u{1dc}\u{204}\u{20c}\u{218}\
	\u{224}\u{23f}\u{265}\u{270}\u{272}\u{27e}\u{280}\u{28d}\u{2d5}\u{2e0}\u{2e8}\
	\u{2ea}\u{2f9}\u{300}\u{315}\u{31f}\u{327}\u{332}\u{33e}";

