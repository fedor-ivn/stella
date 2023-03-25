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

		pub const COMMA:isize=1; 
		pub const SEMICOLON:isize=2; 
		pub const LPAREN:isize=3; 
		pub const RPAREN:isize=4; 
		pub const LBRACE:isize=5; 
		pub const RBRACE:isize=6; 
		pub const EQUALS:isize=7; 
		pub const COLON:isize=8; 
		pub const ARROW:isize=9; 
		pub const DOUBLEARROW:isize=10; 
		pub const LANGLEBAR:isize=11; 
		pub const RANGLEBAR:isize=12; 
		pub const LBRACKET:isize=13; 
		pub const RBRACKET:isize=14; 
		pub const LE:isize=15; 
		pub const LEQ:isize=16; 
		pub const GE:isize=17; 
		pub const GEQ:isize=18; 
		pub const EQ:isize=19; 
		pub const NEQ:isize=20; 
		pub const PLUS:isize=21; 
		pub const TIMES:isize=22; 
		pub const LIST_HEAD:isize=23; 
		pub const LIST_ISEMPTY:isize=24; 
		pub const LIST_TAIL:isize=25; 
		pub const NAT_PRED:isize=26; 
		pub const NAT_ISZERO:isize=27; 
		pub const NAT_REC:isize=28; 
		pub const DOT:isize=29; 
		pub const TYPE_BOOL:isize=30; 
		pub const TYPE_NAT:isize=31; 
		pub const TYPE_UNIT:isize=32; 
		pub const AND:isize=33; 
		pub const AS:isize=34; 
		pub const CONS:isize=35; 
		pub const CORE:isize=36; 
		pub const ELSE:isize=37; 
		pub const EXTEND:isize=38; 
		pub const FALSE:isize=39; 
		pub const FIX:isize=40; 
		pub const FN:isize=41; 
		pub const FOLD:isize=42; 
		pub const IF:isize=43; 
		pub const IN:isize=44; 
		pub const INLINE:isize=45; 
		pub const LANGUAGE:isize=46; 
		pub const LET:isize=47; 
		pub const MATCH:isize=48; 
		pub const NOT:isize=49; 
		pub const OR:isize=50; 
		pub const RECORD:isize=51; 
		pub const RETURN:isize=52; 
		pub const SUCC:isize=53; 
		pub const THEN:isize=54; 
		pub const THROWS:isize=55; 
		pub const TRUE:isize=56; 
		pub const TYPE:isize=57; 
		pub const UNFOLD:isize=58; 
		pub const VARIANT:isize=59; 
		pub const WITH:isize=60; 
		pub const MU:isize=61; 
		pub const COMMENT_antlr_builtin:isize=62; 
		pub const StellaIdent:isize=63; 
		pub const ExtensionName:isize=64; 
		pub const INTEGER:isize=65; 
		pub const WS:isize=66; 
		pub const ErrorToken:isize=67;
	pub const RULE_start_Program:usize = 0; 
	pub const RULE_start_Expr:usize = 1; 
	pub const RULE_start_Type:usize = 2; 
	pub const RULE_program:usize = 3; 
	pub const RULE_languageDecl:usize = 4; 
	pub const RULE_extension:usize = 5; 
	pub const RULE_decl:usize = 6; 
	pub const RULE_annotation:usize = 7; 
	pub const RULE_paramDecl:usize = 8; 
	pub const RULE_expr:usize = 9; 
	pub const RULE_stellatype:usize = 10;
	pub const ruleNames: [&'static str; 11] =  [
		"start_Program", "start_Expr", "start_Type", "program", "languageDecl", 
		"extension", "decl", "annotation", "paramDecl", "expr", "stellatype"
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
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;68]  = [
		None, Some("COMMA"), Some("SEMICOLON"), Some("LPAREN"), Some("RPAREN"), 
		Some("LBRACE"), Some("RBRACE"), Some("EQUALS"), Some("COLON"), Some("ARROW"), 
		Some("DOUBLEARROW"), Some("LANGLEBAR"), Some("RANGLEBAR"), Some("LBRACKET"), 
		Some("RBRACKET"), Some("LE"), Some("LEQ"), Some("GE"), Some("GEQ"), Some("EQ"), 
		Some("NEQ"), Some("PLUS"), Some("TIMES"), Some("LIST_HEAD"), Some("LIST_ISEMPTY"), 
		Some("LIST_TAIL"), Some("NAT_PRED"), Some("NAT_ISZERO"), Some("NAT_REC"), 
		Some("DOT"), Some("TYPE_BOOL"), Some("TYPE_NAT"), Some("TYPE_UNIT"), Some("AND"), 
		Some("AS"), Some("CONS"), Some("CORE"), Some("ELSE"), Some("EXTEND"), 
		Some("FALSE"), Some("FIX"), Some("FN"), Some("FOLD"), Some("IF"), Some("IN"), 
		Some("INLINE"), Some("LANGUAGE"), Some("LET"), Some("MATCH"), Some("NOT"), 
		Some("OR"), Some("RECORD"), Some("RETURN"), Some("SUCC"), Some("THEN"), 
		Some("THROWS"), Some("TRUE"), Some("TYPE"), Some("UNFOLD"), Some("VARIANT"), 
		Some("WITH"), Some("MU"), Some("COMMENT_antlr_builtin"), Some("StellaIdent"), 
		Some("ExtensionName"), Some("INTEGER"), Some("WS"), Some("ErrorToken")
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
        recog.base.enter_rule(_localctx.clone(), 0, RULE_start_Program);
        let mut _localctx: Rc<Start_ProgramContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule program*/
			recog.base.set_state(22);
			let tmp = recog.program()?;
			 cast_mut::<_,Start_ProgramContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(23);
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
        recog.base.enter_rule(_localctx.clone(), 2, RULE_start_Expr);
        let mut _localctx: Rc<Start_ExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr*/
			recog.base.set_state(25);
			let tmp = recog.expr()?;
			 cast_mut::<_,Start_ExprContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(26);
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
        recog.base.enter_rule(_localctx.clone(), 4, RULE_start_Type);
        let mut _localctx: Rc<Start_TypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule stellatype*/
			recog.base.set_state(28);
			let tmp = recog.stellatype()?;
			 cast_mut::<_,Start_TypeContext >(&mut _localctx).x = Some(tmp.clone());
			  

			recog.base.set_state(29);
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
//------------------- program ----------------
pub type ProgramContextAll<'input> = ProgramContext<'input>;


pub type ProgramContext<'input> = BaseParserRuleContext<'input,ProgramContextExt<'input>>;

#[derive(Clone)]
pub struct ProgramContextExt<'input>{
	pub extension: Option<Rc<ExtensionContextAll<'input>>>,
	pub extensions:Vec<Rc<ExtensionContextAll<'input>>>,
	pub decl: Option<Rc<DeclContextAll<'input>>>,
	pub decls:Vec<Rc<DeclContextAll<'input>>>,
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
				extension: None, decl: None, 
				extensions: Vec::new(), decls: Vec::new(), 
				ph:PhantomData
			}),
		)
	}
}

pub trait ProgramContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ProgramContextExt<'input>>{

fn languageDecl(&self) -> Option<Rc<LanguageDeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn extension_all(&self) ->  Vec<Rc<ExtensionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn extension(&self, i: usize) -> Option<Rc<ExtensionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn decl_all(&self) ->  Vec<Rc<DeclContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn decl(&self, i: usize) -> Option<Rc<DeclContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
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
        recog.base.enter_rule(_localctx.clone(), 6, RULE_program);
        let mut _localctx: Rc<ProgramContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule languageDecl*/
			recog.base.set_state(31);
			recog.languageDecl()?;

			recog.base.set_state(35);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==EXTEND {
				{
				{
				/*InvokeRule extension*/
				recog.base.set_state(32);
				let tmp = recog.extension()?;
				 cast_mut::<_,ProgramContext >(&mut _localctx).extension = Some(tmp.clone());
				  

				let temp =  cast_mut::<_,ProgramContext >(&mut _localctx).extension.clone().unwrap()
				 ;
				 cast_mut::<_,ProgramContext >(&mut _localctx).extensions.push(temp);
				  
				}
				}
				recog.base.set_state(37);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(41);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 41)) & !0x3f) == 0 && ((1usize << (_la - 41)) & ((1usize << (FN - 41)) | (1usize << (INLINE - 41)) | (1usize << (TYPE - 41)))) != 0) {
				{
				{
				/*InvokeRule decl*/
				recog.base.set_state(38);
				let tmp = recog.decl()?;
				 cast_mut::<_,ProgramContext >(&mut _localctx).decl = Some(tmp.clone());
				  

				let temp =  cast_mut::<_,ProgramContext >(&mut _localctx).decl.clone().unwrap()
				 ;
				 cast_mut::<_,ProgramContext >(&mut _localctx).decls.push(temp);
				  
				}
				}
				recog.base.set_state(43);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
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
#[derive(Debug)]
pub enum LanguageDeclContextAll<'input>{
	LanguageCoreContext(LanguageCoreContext<'input>),
Error(LanguageDeclContext<'input>)
}
antlr_rust::tid!{LanguageDeclContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LanguageDeclContextAll<'input>{}

impl<'input> stellaParserContext<'input> for LanguageDeclContextAll<'input>{}

impl<'input> Deref for LanguageDeclContextAll<'input>{
	type Target = dyn LanguageDeclContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LanguageDeclContextAll::*;
		match self{
			LanguageCoreContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for LanguageDeclContextAll<'input>{
    fn enter(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LanguageDeclContext<'input> = BaseParserRuleContext<'input,LanguageDeclContextExt<'input>>;

#[derive(Clone)]
pub struct LanguageDeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for LanguageDeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for LanguageDeclContext<'input>{
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
		LanguageDeclContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LanguageDeclContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LanguageDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<LanguageDeclContextExt<'input>>{


}

impl<'input> LanguageDeclContextAttrs<'input> for LanguageDeclContext<'input>{}

pub type LanguageCoreContext<'input> = BaseParserRuleContext<'input,LanguageCoreContextExt<'input>>;

pub trait LanguageCoreContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LANGUAGE
	/// Returns `None` if there is no child corresponding to token LANGUAGE
	fn LANGUAGE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(LANGUAGE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token CORE
	/// Returns `None` if there is no child corresponding to token CORE
	fn CORE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(CORE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> LanguageCoreContextAttrs<'input> for LanguageCoreContext<'input>{}

pub struct LanguageCoreContextExt<'input>{
	base:LanguageDeclContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LanguageCoreContextExt<'a>}

impl<'input> stellaParserContext<'input> for LanguageCoreContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for LanguageCoreContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LanguageCore(self);
	}
}

impl<'input> CustomRuleContext<'input> for LanguageCoreContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_languageDecl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_languageDecl }
}

impl<'input> Borrow<LanguageDeclContextExt<'input>> for LanguageCoreContext<'input>{
	fn borrow(&self) -> &LanguageDeclContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LanguageDeclContextExt<'input>> for LanguageCoreContext<'input>{
	fn borrow_mut(&mut self) -> &mut LanguageDeclContextExt<'input> { &mut self.base }
}

impl<'input> LanguageDeclContextAttrs<'input> for LanguageCoreContext<'input> {}

impl<'input> LanguageCoreContextExt<'input>{
	fn new(ctx: &dyn LanguageDeclContextAttrs<'input>) -> Rc<LanguageDeclContextAll<'input>>  {
		Rc::new(
			LanguageDeclContextAll::LanguageCoreContext(
				BaseParserRuleContext::copy_from(ctx,LanguageCoreContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

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
        recog.base.enter_rule(_localctx.clone(), 8, RULE_languageDecl);
        let mut _localctx: Rc<LanguageDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = LanguageCoreContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(44);
			recog.base.match_token(LANGUAGE,&mut recog.err_handler)?;

			recog.base.set_state(45);
			recog.base.match_token(CORE,&mut recog.err_handler)?;

			recog.base.set_state(46);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

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
#[derive(Debug)]
pub enum ExtensionContextAll<'input>{
	AnExtensionContext(AnExtensionContext<'input>),
Error(ExtensionContext<'input>)
}
antlr_rust::tid!{ExtensionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExtensionContextAll<'input>{}

impl<'input> stellaParserContext<'input> for ExtensionContextAll<'input>{}

impl<'input> Deref for ExtensionContextAll<'input>{
	type Target = dyn ExtensionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExtensionContextAll::*;
		match self{
			AnExtensionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ExtensionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExtensionContext<'input> = BaseParserRuleContext<'input,ExtensionContextExt<'input>>;

#[derive(Clone)]
pub struct ExtensionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ExtensionContext<'input>{
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
		ExtensionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExtensionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExtensionContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ExtensionContextExt<'input>>{


}

impl<'input> ExtensionContextAttrs<'input> for ExtensionContext<'input>{}

pub type AnExtensionContext<'input> = BaseParserRuleContext<'input,AnExtensionContextExt<'input>>;

pub trait AnExtensionContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EXTEND
	/// Returns `None` if there is no child corresponding to token EXTEND
	fn EXTEND(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(EXTEND, 0)
	}
	/// Retrieves first TerminalNode corresponding to token WITH
	/// Returns `None` if there is no child corresponding to token WITH
	fn WITH(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(WITH, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token ExtensionName in current rule
	fn ExtensionName_all(&self) -> Vec<Rc<TerminalNode<'input,stellaParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token ExtensionName, starting from 0.
	/// Returns `None` if number of children corresponding to token ExtensionName is less or equal than `i`.
	fn ExtensionName(&self, i: usize) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(ExtensionName, i)
	}
	/// Retrieves first TerminalNode corresponding to token COMMA
	/// Returns `None` if there is no child corresponding to token COMMA
	fn COMMA(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(COMMA, 0)
	}
}

impl<'input> AnExtensionContextAttrs<'input> for AnExtensionContext<'input>{}

pub struct AnExtensionContextExt<'input>{
	base:ExtensionContextExt<'input>,
	pub ExtensionName: Option<TokenType<'input>>,
	pub extensionNames:Vec<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AnExtensionContextExt<'a>}

impl<'input> stellaParserContext<'input> for AnExtensionContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for AnExtensionContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AnExtension(self);
	}
}

impl<'input> CustomRuleContext<'input> for AnExtensionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_extension }
	//fn type_rule_index() -> usize where Self: Sized { RULE_extension }
}

impl<'input> Borrow<ExtensionContextExt<'input>> for AnExtensionContext<'input>{
	fn borrow(&self) -> &ExtensionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExtensionContextExt<'input>> for AnExtensionContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExtensionContextExt<'input> { &mut self.base }
}

impl<'input> ExtensionContextAttrs<'input> for AnExtensionContext<'input> {}

impl<'input> AnExtensionContextExt<'input>{
	fn new(ctx: &dyn ExtensionContextAttrs<'input>) -> Rc<ExtensionContextAll<'input>>  {
		Rc::new(
			ExtensionContextAll::AnExtensionContext(
				BaseParserRuleContext::copy_from(ctx,AnExtensionContextExt{
					ExtensionName:None, 
        			extensionNames:Vec::new(), 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

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
        recog.base.enter_rule(_localctx.clone(), 10, RULE_extension);
        let mut _localctx: Rc<ExtensionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = AnExtensionContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(48);
			recog.base.match_token(EXTEND,&mut recog.err_handler)?;

			recog.base.set_state(49);
			recog.base.match_token(WITH,&mut recog.err_handler)?;

			recog.base.set_state(50);
			let tmp = recog.base.match_token(ExtensionName,&mut recog.err_handler)?;
			if let ExtensionContextAll::AnExtensionContext(ctx) = cast_mut::<_,ExtensionContextAll >(&mut _localctx){
			ctx.ExtensionName = Some(tmp.clone()); } else {unreachable!("cant cast");}  

			let temp = if let ExtensionContextAll::AnExtensionContext(ctx) = cast_mut::<_,ExtensionContextAll >(&mut _localctx){
			ctx.ExtensionName.clone().unwrap() } else {unreachable!("cant cast");} ;
			if let ExtensionContextAll::AnExtensionContext(ctx) = cast_mut::<_,ExtensionContextAll >(&mut _localctx){
			ctx.extensionNames.push(temp); } else {unreachable!("cant cast");}  
			{
			recog.base.set_state(51);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			recog.base.set_state(52);
			let tmp = recog.base.match_token(ExtensionName,&mut recog.err_handler)?;
			if let ExtensionContextAll::AnExtensionContext(ctx) = cast_mut::<_,ExtensionContextAll >(&mut _localctx){
			ctx.ExtensionName = Some(tmp.clone()); } else {unreachable!("cant cast");}  

			let temp = if let ExtensionContextAll::AnExtensionContext(ctx) = cast_mut::<_,ExtensionContextAll >(&mut _localctx){
			ctx.ExtensionName.clone().unwrap() } else {unreachable!("cant cast");} ;
			if let ExtensionContextAll::AnExtensionContext(ctx) = cast_mut::<_,ExtensionContextAll >(&mut _localctx){
			ctx.extensionNames.push(temp); } else {unreachable!("cant cast");}  
			}
			recog.base.set_state(54);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

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
//------------------- decl ----------------
#[derive(Debug)]
pub enum DeclContextAll<'input>{
	DeclTypeAliasContext(DeclTypeAliasContext<'input>),
	DeclFunContext(DeclFunContext<'input>),
Error(DeclContext<'input>)
}
antlr_rust::tid!{DeclContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for DeclContextAll<'input>{}

impl<'input> stellaParserContext<'input> for DeclContextAll<'input>{}

impl<'input> Deref for DeclContextAll<'input>{
	type Target = dyn DeclContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use DeclContextAll::*;
		match self{
			DeclTypeAliasContext(inner) => inner,
			DeclFunContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for DeclContextAll<'input>{
    fn enter(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type DeclContext<'input> = BaseParserRuleContext<'input,DeclContextExt<'input>>;

#[derive(Clone)]
pub struct DeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for DeclContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for DeclContext<'input>{
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
		DeclContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait DeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<DeclContextExt<'input>>{


}

impl<'input> DeclContextAttrs<'input> for DeclContext<'input>{}

pub type DeclTypeAliasContext<'input> = BaseParserRuleContext<'input,DeclTypeAliasContextExt<'input>>;

pub trait DeclTypeAliasContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TYPE
	/// Returns `None` if there is no child corresponding to token TYPE
	fn TYPE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(TYPE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token EQUALS
	/// Returns `None` if there is no child corresponding to token EQUALS
	fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(EQUALS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
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

impl<'input> DeclTypeAliasContextAttrs<'input> for DeclTypeAliasContext<'input>{}

pub struct DeclTypeAliasContextExt<'input>{
	base:DeclContextExt<'input>,
	pub name: Option<TokenType<'input>>,
	pub atype: Option<Rc<StellatypeContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DeclTypeAliasContextExt<'a>}

impl<'input> stellaParserContext<'input> for DeclTypeAliasContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for DeclTypeAliasContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DeclTypeAlias(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclTypeAliasContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decl }
}

impl<'input> Borrow<DeclContextExt<'input>> for DeclTypeAliasContext<'input>{
	fn borrow(&self) -> &DeclContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DeclContextExt<'input>> for DeclTypeAliasContext<'input>{
	fn borrow_mut(&mut self) -> &mut DeclContextExt<'input> { &mut self.base }
}

impl<'input> DeclContextAttrs<'input> for DeclTypeAliasContext<'input> {}

impl<'input> DeclTypeAliasContextExt<'input>{
	fn new(ctx: &dyn DeclContextAttrs<'input>) -> Rc<DeclContextAll<'input>>  {
		Rc::new(
			DeclContextAll::DeclTypeAliasContext(
				BaseParserRuleContext::copy_from(ctx,DeclTypeAliasContextExt{
					name:None, 
        			atype:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DeclFunContext<'input> = BaseParserRuleContext<'input,DeclFunContextExt<'input>>;

pub trait DeclFunContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FN
	/// Returns `None` if there is no child corresponding to token FN
	fn FN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(FN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACE
	/// Returns `None` if there is no child corresponding to token LBRACE
	fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(LBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RETURN
	/// Returns `None` if there is no child corresponding to token RETURN
	fn RETURN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(RETURN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACE
	/// Returns `None` if there is no child corresponding to token RBRACE
	fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(RBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token StellaIdent
	/// Returns `None` if there is no child corresponding to token StellaIdent
	fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(StellaIdent, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ARROW
	/// Returns `None` if there is no child corresponding to token ARROW
	fn ARROW(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(ARROW, 0)
	}
	/// Retrieves first TerminalNode corresponding to token THROWS
	/// Returns `None` if there is no child corresponding to token THROWS
	fn THROWS(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(THROWS, 0)
	}
	fn annotation_all(&self) ->  Vec<Rc<AnnotationContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn annotation(&self, i: usize) -> Option<Rc<AnnotationContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn paramDecl_all(&self) ->  Vec<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn paramDecl(&self, i: usize) -> Option<Rc<ParamDeclContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn stellatype_all(&self) ->  Vec<Rc<StellatypeContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stellatype(&self, i: usize) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn decl_all(&self) ->  Vec<Rc<DeclContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn decl(&self, i: usize) -> Option<Rc<DeclContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,stellaParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> DeclFunContextAttrs<'input> for DeclFunContext<'input>{}

pub struct DeclFunContextExt<'input>{
	base:DeclContextExt<'input>,
	pub annotation: Option<Rc<AnnotationContextAll<'input>>>,
	pub annotations:Vec<Rc<AnnotationContextAll<'input>>>,
	pub name: Option<TokenType<'input>>,
	pub paramDecl: Option<Rc<ParamDeclContextAll<'input>>>,
	pub paramDecls:Vec<Rc<ParamDeclContextAll<'input>>>,
	pub returnType: Option<Rc<StellatypeContextAll<'input>>>,
	pub throwType: Option<Rc<StellatypeContextAll<'input>>>,
	pub decl: Option<Rc<DeclContextAll<'input>>>,
	pub localDecls:Vec<Rc<DeclContextAll<'input>>>,
	pub returnExpr: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DeclFunContextExt<'a>}

impl<'input> stellaParserContext<'input> for DeclFunContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for DeclFunContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DeclFun(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclFunContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decl }
}

impl<'input> Borrow<DeclContextExt<'input>> for DeclFunContext<'input>{
	fn borrow(&self) -> &DeclContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DeclContextExt<'input>> for DeclFunContext<'input>{
	fn borrow_mut(&mut self) -> &mut DeclContextExt<'input> { &mut self.base }
}

impl<'input> DeclContextAttrs<'input> for DeclFunContext<'input> {}

impl<'input> DeclFunContextExt<'input>{
	fn new(ctx: &dyn DeclContextAttrs<'input>) -> Rc<DeclContextAll<'input>>  {
		Rc::new(
			DeclContextAll::DeclFunContext(
				BaseParserRuleContext::copy_from(ctx,DeclFunContextExt{
					name:None, 
        			annotation:None, paramDecl:None, returnType:None, throwType:None, decl:None, returnExpr:None, 
        			annotations:Vec::new(), paramDecls:Vec::new(), localDecls:Vec::new(), 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

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
        recog.base.enter_rule(_localctx.clone(), 12, RULE_decl);
        let mut _localctx: Rc<DeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(102);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 FN | INLINE 
				=> {
					let tmp = DeclFunContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(59);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==INLINE {
						{
						{
						/*InvokeRule annotation*/
						recog.base.set_state(56);
						let tmp = recog.annotation()?;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.annotation = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						let temp = if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.annotation.clone().unwrap() } else {unreachable!("cant cast");} ;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.annotations.push(temp); } else {unreachable!("cant cast");}  
						}
						}
						recog.base.set_state(61);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(62);
					recog.base.match_token(FN,&mut recog.err_handler)?;

					recog.base.set_state(63);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
					ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(64);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(73);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==StellaIdent {
						{
						/*InvokeRule paramDecl*/
						recog.base.set_state(65);
						let tmp = recog.paramDecl()?;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.paramDecl = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						let temp = if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.paramDecl.clone().unwrap() } else {unreachable!("cant cast");} ;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.paramDecls.push(temp); } else {unreachable!("cant cast");}  
						recog.base.set_state(70);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==COMMA {
							{
							{
							recog.base.set_state(66);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule paramDecl*/
							recog.base.set_state(67);
							let tmp = recog.paramDecl()?;
							if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
							ctx.paramDecl = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							let temp = if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
							ctx.paramDecl.clone().unwrap() } else {unreachable!("cant cast");} ;
							if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
							ctx.paramDecls.push(temp); } else {unreachable!("cant cast");}  
							}
							}
							recog.base.set_state(72);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(75);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					recog.base.set_state(78);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ARROW {
						{
						recog.base.set_state(76);
						recog.base.match_token(ARROW,&mut recog.err_handler)?;

						/*InvokeRule stellatype*/
						recog.base.set_state(77);
						let tmp = recog.stellatype()?;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.returnType = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(82);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==THROWS {
						{
						recog.base.set_state(80);
						recog.base.match_token(THROWS,&mut recog.err_handler)?;

						/*InvokeRule stellatype*/
						recog.base.set_state(81);
						let tmp = recog.stellatype()?;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.throwType = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						}
					}

					recog.base.set_state(84);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					recog.base.set_state(88);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while ((((_la - 41)) & !0x3f) == 0 && ((1usize << (_la - 41)) & ((1usize << (FN - 41)) | (1usize << (INLINE - 41)) | (1usize << (TYPE - 41)))) != 0) {
						{
						{
						/*InvokeRule decl*/
						recog.base.set_state(85);
						let tmp = recog.decl()?;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.decl = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						let temp = if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.decl.clone().unwrap() } else {unreachable!("cant cast");} ;
						if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
						ctx.localDecls.push(temp); } else {unreachable!("cant cast");}  
						}
						}
						recog.base.set_state(90);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(91);
					recog.base.match_token(RETURN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(92);
					let tmp = recog.expr()?;
					if let DeclContextAll::DeclFunContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
					ctx.returnExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(93);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					recog.base.set_state(94);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}

			 TYPE 
				=> {
					let tmp = DeclTypeAliasContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(96);
					recog.base.match_token(TYPE,&mut recog.err_handler)?;

					recog.base.set_state(97);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					if let DeclContextAll::DeclTypeAliasContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
					ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(98);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(99);
					let tmp = recog.stellatype()?;
					if let DeclContextAll::DeclTypeAliasContext(ctx) = cast_mut::<_,DeclContextAll >(&mut _localctx){
					ctx.atype = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(100);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

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
//------------------- annotation ----------------
#[derive(Debug)]
pub enum AnnotationContextAll<'input>{
	InlineAnnotationContext(InlineAnnotationContext<'input>),
Error(AnnotationContext<'input>)
}
antlr_rust::tid!{AnnotationContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AnnotationContextAll<'input>{}

impl<'input> stellaParserContext<'input> for AnnotationContextAll<'input>{}

impl<'input> Deref for AnnotationContextAll<'input>{
	type Target = dyn AnnotationContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AnnotationContextAll::*;
		match self{
			InlineAnnotationContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for AnnotationContextAll<'input>{
    fn enter(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type AnnotationContext<'input> = BaseParserRuleContext<'input,AnnotationContextExt<'input>>;

#[derive(Clone)]
pub struct AnnotationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for AnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for AnnotationContext<'input>{
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
		AnnotationContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AnnotationContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AnnotationContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<AnnotationContextExt<'input>>{


}

impl<'input> AnnotationContextAttrs<'input> for AnnotationContext<'input>{}

pub type InlineAnnotationContext<'input> = BaseParserRuleContext<'input,InlineAnnotationContextExt<'input>>;

pub trait InlineAnnotationContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INLINE
	/// Returns `None` if there is no child corresponding to token INLINE
	fn INLINE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(INLINE, 0)
	}
}

impl<'input> InlineAnnotationContextAttrs<'input> for InlineAnnotationContext<'input>{}

pub struct InlineAnnotationContextExt<'input>{
	base:AnnotationContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{InlineAnnotationContextExt<'a>}

impl<'input> stellaParserContext<'input> for InlineAnnotationContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for InlineAnnotationContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_InlineAnnotation(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineAnnotationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_annotation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_annotation }
}

impl<'input> Borrow<AnnotationContextExt<'input>> for InlineAnnotationContext<'input>{
	fn borrow(&self) -> &AnnotationContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AnnotationContextExt<'input>> for InlineAnnotationContext<'input>{
	fn borrow_mut(&mut self) -> &mut AnnotationContextExt<'input> { &mut self.base }
}

impl<'input> AnnotationContextAttrs<'input> for InlineAnnotationContext<'input> {}

impl<'input> InlineAnnotationContextExt<'input>{
	fn new(ctx: &dyn AnnotationContextAttrs<'input>) -> Rc<AnnotationContextAll<'input>>  {
		Rc::new(
			AnnotationContextAll::InlineAnnotationContext(
				BaseParserRuleContext::copy_from(ctx,InlineAnnotationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

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
        recog.base.enter_rule(_localctx.clone(), 14, RULE_annotation);
        let mut _localctx: Rc<AnnotationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = InlineAnnotationContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(104);
			recog.base.match_token(INLINE,&mut recog.err_handler)?;

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
//------------------- paramDecl ----------------
pub type ParamDeclContextAll<'input> = ParamDeclContext<'input>;


pub type ParamDeclContext<'input> = BaseParserRuleContext<'input,ParamDeclContextExt<'input>>;

#[derive(Clone)]
pub struct ParamDeclContextExt<'input>{
	pub name: Option<TokenType<'input>>,
	pub paramType: Option<Rc<StellatypeContextAll<'input>>>,
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
				name: None, 
				paramType: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamDeclContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ParamDeclContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
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
        recog.base.enter_rule(_localctx.clone(), 16, RULE_paramDecl);
        let mut _localctx: Rc<ParamDeclContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(106);
			let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
			 cast_mut::<_,ParamDeclContext >(&mut _localctx).name = Some(tmp.clone());
			  

			recog.base.set_state(107);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			/*InvokeRule stellatype*/
			recog.base.set_state(108);
			let tmp = recog.stellatype()?;
			 cast_mut::<_,ParamDeclContext >(&mut _localctx).paramType = Some(tmp.clone());
			  

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
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	ConstTrueContext(ConstTrueContext<'input>),
	SuccContext(SuccContext<'input>),
	VarContext(VarContext<'input>),
	NatRecContext(NatRecContext<'input>),
	ConstFalseContext(ConstFalseContext<'input>),
	IfContext(IfContext<'input>),
	ConstIntContext(ConstIntContext<'input>),
Error(ExprContext<'input>)
}
antlr_rust::tid!{ExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> stellaParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			ConstTrueContext(inner) => inner,
			SuccContext(inner) => inner,
			VarContext(inner) => inner,
			NatRecContext(inner) => inner,
			ConstFalseContext(inner) => inner,
			IfContext(inner) => inner,
			ConstIntContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ExprContext<'input>{
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
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type ConstTrueContext<'input> = BaseParserRuleContext<'input,ConstTrueContextExt<'input>>;

pub trait ConstTrueContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TRUE
	/// Returns `None` if there is no child corresponding to token TRUE
	fn TRUE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(TRUE, 0)
	}
}

impl<'input> ConstTrueContextAttrs<'input> for ConstTrueContext<'input>{}

pub struct ConstTrueContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ConstTrueContextExt<'a>}

impl<'input> stellaParserContext<'input> for ConstTrueContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ConstTrueContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ConstTrue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstTrueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ConstTrueContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ConstTrueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ConstTrueContext<'input> {}

impl<'input> ConstTrueContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ConstTrueContext(
				BaseParserRuleContext::copy_from(ctx,ConstTrueContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SuccContext<'input> = BaseParserRuleContext<'input,SuccContextExt<'input>>;

pub trait SuccContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token SUCC
	/// Returns `None` if there is no child corresponding to token SUCC
	fn SUCC(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(SUCC, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SuccContextAttrs<'input> for SuccContext<'input>{}

pub struct SuccContextExt<'input>{
	base:ExprContextExt<'input>,
	pub n: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SuccContextExt<'a>}

impl<'input> stellaParserContext<'input> for SuccContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for SuccContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Succ(self);
	}
}

impl<'input> CustomRuleContext<'input> for SuccContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for SuccContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for SuccContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for SuccContext<'input> {}

impl<'input> SuccContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::SuccContext(
				BaseParserRuleContext::copy_from(ctx,SuccContextExt{
        			n:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VarContext<'input> = BaseParserRuleContext<'input,VarContextExt<'input>>;

pub trait VarContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token StellaIdent
	/// Returns `None` if there is no child corresponding to token StellaIdent
	fn StellaIdent(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(StellaIdent, 0)
	}
}

impl<'input> VarContextAttrs<'input> for VarContext<'input>{}

pub struct VarContextExt<'input>{
	base:ExprContextExt<'input>,
	pub name: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VarContextExt<'a>}

impl<'input> stellaParserContext<'input> for VarContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for VarContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Var(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for VarContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for VarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for VarContext<'input> {}

impl<'input> VarContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::VarContext(
				BaseParserRuleContext::copy_from(ctx,VarContextExt{
					name:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NatRecContext<'input> = BaseParserRuleContext<'input,NatRecContextExt<'input>>;

pub trait NatRecContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NAT_REC
	/// Returns `None` if there is no child corresponding to token NAT_REC
	fn NAT_REC(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(NAT_REC, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,stellaParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> NatRecContextAttrs<'input> for NatRecContext<'input>{}

pub struct NatRecContextExt<'input>{
	base:ExprContextExt<'input>,
	pub n: Option<Rc<ExprContextAll<'input>>>,
	pub initial: Option<Rc<ExprContextAll<'input>>>,
	pub step: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NatRecContextExt<'a>}

impl<'input> stellaParserContext<'input> for NatRecContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for NatRecContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_NatRec(self);
	}
}

impl<'input> CustomRuleContext<'input> for NatRecContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for NatRecContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for NatRecContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for NatRecContext<'input> {}

impl<'input> NatRecContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::NatRecContext(
				BaseParserRuleContext::copy_from(ctx,NatRecContextExt{
        			n:None, initial:None, step:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ConstFalseContext<'input> = BaseParserRuleContext<'input,ConstFalseContextExt<'input>>;

pub trait ConstFalseContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FALSE
	/// Returns `None` if there is no child corresponding to token FALSE
	fn FALSE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(FALSE, 0)
	}
}

impl<'input> ConstFalseContextAttrs<'input> for ConstFalseContext<'input>{}

pub struct ConstFalseContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ConstFalseContextExt<'a>}

impl<'input> stellaParserContext<'input> for ConstFalseContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ConstFalseContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ConstFalse(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstFalseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ConstFalseContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ConstFalseContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ConstFalseContext<'input> {}

impl<'input> ConstFalseContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ConstFalseContext(
				BaseParserRuleContext::copy_from(ctx,ConstFalseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IfContext<'input> = BaseParserRuleContext<'input,IfContextExt<'input>>;

pub trait IfContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IF
	/// Returns `None` if there is no child corresponding to token IF
	fn IF(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(IF, 0)
	}
	/// Retrieves first TerminalNode corresponding to token THEN
	/// Returns `None` if there is no child corresponding to token THEN
	fn THEN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(THEN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ELSE
	/// Returns `None` if there is no child corresponding to token ELSE
	fn ELSE(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(ELSE, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> IfContextAttrs<'input> for IfContext<'input>{}

pub struct IfContextExt<'input>{
	base:ExprContextExt<'input>,
	pub condition: Option<Rc<ExprContextAll<'input>>>,
	pub thenExpr: Option<Rc<ExprContextAll<'input>>>,
	pub elseExpr: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IfContextExt<'a>}

impl<'input> stellaParserContext<'input> for IfContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for IfContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_If(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for IfContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for IfContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for IfContext<'input> {}

impl<'input> IfContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::IfContext(
				BaseParserRuleContext::copy_from(ctx,IfContextExt{
        			condition:None, thenExpr:None, elseExpr:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ConstIntContext<'input> = BaseParserRuleContext<'input,ConstIntContextExt<'input>>;

pub trait ConstIntContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INTEGER
	/// Returns `None` if there is no child corresponding to token INTEGER
	fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(INTEGER, 0)
	}
}

impl<'input> ConstIntContextAttrs<'input> for ConstIntContext<'input>{}

pub struct ConstIntContextExt<'input>{
	base:ExprContextExt<'input>,
	pub n: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ConstIntContextExt<'a>}

impl<'input> stellaParserContext<'input> for ConstIntContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for ConstIntContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ConstInt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstIntContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ConstIntContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ConstIntContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ConstIntContext<'input> {}

impl<'input> ConstIntContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ConstIntContext(
				BaseParserRuleContext::copy_from(ctx,ConstIntContextExt{
					n:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

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
        recog.base.enter_rule(_localctx.clone(), 18, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(135);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IF 
				=> {
					let tmp = IfContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(110);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(111);
					let tmp = recog.expr()?;
					if let ExprContextAll::IfContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.condition = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(112);
					recog.base.match_token(THEN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(113);
					let tmp = recog.expr()?;
					if let ExprContextAll::IfContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.thenExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(114);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(115);
					let tmp = recog.expr()?;
					if let ExprContextAll::IfContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.elseExpr = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

			 TRUE 
				=> {
					let tmp = ConstTrueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(117);
					recog.base.match_token(TRUE,&mut recog.err_handler)?;

					}
				}

			 FALSE 
				=> {
					let tmp = ConstFalseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(118);
					recog.base.match_token(FALSE,&mut recog.err_handler)?;

					}
				}

			 INTEGER 
				=> {
					let tmp = ConstIntContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(119);
					let tmp = recog.base.match_token(INTEGER,&mut recog.err_handler)?;
					if let ExprContextAll::ConstIntContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.n = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					}
				}

			 SUCC 
				=> {
					let tmp = SuccContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(120);
					recog.base.match_token(SUCC,&mut recog.err_handler)?;

					recog.base.set_state(121);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(122);
					let tmp = recog.expr()?;
					if let ExprContextAll::SuccContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.n = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(123);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 NAT_REC 
				=> {
					let tmp = NatRecContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(125);
					recog.base.match_token(NAT_REC,&mut recog.err_handler)?;

					recog.base.set_state(126);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(127);
					let tmp = recog.expr()?;
					if let ExprContextAll::NatRecContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.n = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(128);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(129);
					let tmp = recog.expr()?;
					if let ExprContextAll::NatRecContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.initial = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(130);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(131);
					let tmp = recog.expr()?;
					if let ExprContextAll::NatRecContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.step = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(132);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}

			 StellaIdent 
				=> {
					let tmp = VarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(134);
					let tmp = recog.base.match_token(StellaIdent,&mut recog.err_handler)?;
					if let ExprContextAll::VarContext(ctx) = cast_mut::<_,ExprContextAll >(&mut _localctx){
					ctx.name = Some(tmp.clone()); } else {unreachable!("cant cast");}  

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
//------------------- stellatype ----------------
#[derive(Debug)]
pub enum StellatypeContextAll<'input>{
	TypeBoolContext(TypeBoolContext<'input>),
	TypeNatContext(TypeNatContext<'input>),
	TypeFunContext(TypeFunContext<'input>),
Error(StellatypeContext<'input>)
}
antlr_rust::tid!{StellatypeContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for StellatypeContextAll<'input>{}

impl<'input> stellaParserContext<'input> for StellatypeContextAll<'input>{}

impl<'input> Deref for StellatypeContextAll<'input>{
	type Target = dyn StellatypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StellatypeContextAll::*;
		match self{
			TypeBoolContext(inner) => inner,
			TypeNatContext(inner) => inner,
			TypeFunContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for StellatypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn stellaParserListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type StellatypeContext<'input> = BaseParserRuleContext<'input,StellatypeContextExt<'input>>;

#[derive(Clone)]
pub struct StellatypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> stellaParserContext<'input> for StellatypeContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for StellatypeContext<'input>{
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
		StellatypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StellatypeContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StellatypeContextAttrs<'input>: stellaParserContext<'input> + BorrowMut<StellatypeContextExt<'input>>{


}

impl<'input> StellatypeContextAttrs<'input> for StellatypeContext<'input>{}

pub type TypeBoolContext<'input> = BaseParserRuleContext<'input,TypeBoolContextExt<'input>>;

pub trait TypeBoolContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TYPE_BOOL
	/// Returns `None` if there is no child corresponding to token TYPE_BOOL
	fn TYPE_BOOL(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(TYPE_BOOL, 0)
	}
}

impl<'input> TypeBoolContextAttrs<'input> for TypeBoolContext<'input>{}

pub struct TypeBoolContextExt<'input>{
	base:StellatypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeBoolContextExt<'a>}

impl<'input> stellaParserContext<'input> for TypeBoolContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for TypeBoolContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeBool(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeBoolContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stellatype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stellatype }
}

impl<'input> Borrow<StellatypeContextExt<'input>> for TypeBoolContext<'input>{
	fn borrow(&self) -> &StellatypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StellatypeContextExt<'input>> for TypeBoolContext<'input>{
	fn borrow_mut(&mut self) -> &mut StellatypeContextExt<'input> { &mut self.base }
}

impl<'input> StellatypeContextAttrs<'input> for TypeBoolContext<'input> {}

impl<'input> TypeBoolContextExt<'input>{
	fn new(ctx: &dyn StellatypeContextAttrs<'input>) -> Rc<StellatypeContextAll<'input>>  {
		Rc::new(
			StellatypeContextAll::TypeBoolContext(
				BaseParserRuleContext::copy_from(ctx,TypeBoolContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeNatContext<'input> = BaseParserRuleContext<'input,TypeNatContextExt<'input>>;

pub trait TypeNatContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TYPE_NAT
	/// Returns `None` if there is no child corresponding to token TYPE_NAT
	fn TYPE_NAT(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(TYPE_NAT, 0)
	}
}

impl<'input> TypeNatContextAttrs<'input> for TypeNatContext<'input>{}

pub struct TypeNatContextExt<'input>{
	base:StellatypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeNatContextExt<'a>}

impl<'input> stellaParserContext<'input> for TypeNatContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for TypeNatContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeNat(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeNatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stellatype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stellatype }
}

impl<'input> Borrow<StellatypeContextExt<'input>> for TypeNatContext<'input>{
	fn borrow(&self) -> &StellatypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StellatypeContextExt<'input>> for TypeNatContext<'input>{
	fn borrow_mut(&mut self) -> &mut StellatypeContextExt<'input> { &mut self.base }
}

impl<'input> StellatypeContextAttrs<'input> for TypeNatContext<'input> {}

impl<'input> TypeNatContextExt<'input>{
	fn new(ctx: &dyn StellatypeContextAttrs<'input>) -> Rc<StellatypeContextAll<'input>>  {
		Rc::new(
			StellatypeContextAll::TypeNatContext(
				BaseParserRuleContext::copy_from(ctx,TypeNatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TypeFunContext<'input> = BaseParserRuleContext<'input,TypeFunContextExt<'input>>;

pub trait TypeFunContextAttrs<'input>: stellaParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FN
	/// Returns `None` if there is no child corresponding to token FN
	fn FN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(FN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ARROW
	/// Returns `None` if there is no child corresponding to token ARROW
	fn ARROW(&self) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(ARROW, 0)
	}
	fn stellatype_all(&self) ->  Vec<Rc<StellatypeContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stellatype(&self, i: usize) -> Option<Rc<StellatypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,stellaParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,stellaParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> TypeFunContextAttrs<'input> for TypeFunContext<'input>{}

pub struct TypeFunContextExt<'input>{
	base:StellatypeContextExt<'input>,
	pub stellatype: Option<Rc<StellatypeContextAll<'input>>>,
	pub paramTypes:Vec<Rc<StellatypeContextAll<'input>>>,
	pub returnType: Option<Rc<StellatypeContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TypeFunContextExt<'a>}

impl<'input> stellaParserContext<'input> for TypeFunContext<'input>{}

impl<'input,'a> Listenable<dyn stellaParserListener<'input> + 'a> for TypeFunContext<'input>{
	fn enter(&self,listener: &mut (dyn stellaParserListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TypeFun(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeFunContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = stellaParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stellatype }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stellatype }
}

impl<'input> Borrow<StellatypeContextExt<'input>> for TypeFunContext<'input>{
	fn borrow(&self) -> &StellatypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StellatypeContextExt<'input>> for TypeFunContext<'input>{
	fn borrow_mut(&mut self) -> &mut StellatypeContextExt<'input> { &mut self.base }
}

impl<'input> StellatypeContextAttrs<'input> for TypeFunContext<'input> {}

impl<'input> TypeFunContextExt<'input>{
	fn new(ctx: &dyn StellatypeContextAttrs<'input>) -> Rc<StellatypeContextAll<'input>>  {
		Rc::new(
			StellatypeContextAll::TypeFunContext(
				BaseParserRuleContext::copy_from(ctx,TypeFunContextExt{
        			stellatype:None, returnType:None, 
        			paramTypes:Vec::new(), 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

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
        recog.base.enter_rule(_localctx.clone(), 20, RULE_stellatype);
        let mut _localctx: Rc<StellatypeContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 TYPE_BOOL 
				=> {
					let tmp = TypeBoolContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(137);
					recog.base.match_token(TYPE_BOOL,&mut recog.err_handler)?;

					}
				}

			 TYPE_NAT 
				=> {
					let tmp = TypeNatContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(138);
					recog.base.match_token(TYPE_NAT,&mut recog.err_handler)?;

					}
				}

			 FN 
				=> {
					let tmp = TypeFunContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(139);
					recog.base.match_token(FN,&mut recog.err_handler)?;

					recog.base.set_state(140);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					recog.base.set_state(149);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 30)) & !0x3f) == 0 && ((1usize << (_la - 30)) & ((1usize << (TYPE_BOOL - 30)) | (1usize << (TYPE_NAT - 30)) | (1usize << (FN - 30)))) != 0) {
						{
						/*InvokeRule stellatype*/
						recog.base.set_state(141);
						let tmp = recog.stellatype()?;
						if let StellatypeContextAll::TypeFunContext(ctx) = cast_mut::<_,StellatypeContextAll >(&mut _localctx){
						ctx.stellatype = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						let temp = if let StellatypeContextAll::TypeFunContext(ctx) = cast_mut::<_,StellatypeContextAll >(&mut _localctx){
						ctx.stellatype.clone().unwrap() } else {unreachable!("cant cast");} ;
						if let StellatypeContextAll::TypeFunContext(ctx) = cast_mut::<_,StellatypeContextAll >(&mut _localctx){
						ctx.paramTypes.push(temp); } else {unreachable!("cant cast");}  
						recog.base.set_state(146);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==COMMA {
							{
							{
							recog.base.set_state(142);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule stellatype*/
							recog.base.set_state(143);
							let tmp = recog.stellatype()?;
							if let StellatypeContextAll::TypeFunContext(ctx) = cast_mut::<_,StellatypeContextAll >(&mut _localctx){
							ctx.stellatype = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							let temp = if let StellatypeContextAll::TypeFunContext(ctx) = cast_mut::<_,StellatypeContextAll >(&mut _localctx){
							ctx.stellatype.clone().unwrap() } else {unreachable!("cant cast");} ;
							if let StellatypeContextAll::TypeFunContext(ctx) = cast_mut::<_,StellatypeContextAll >(&mut _localctx){
							ctx.paramTypes.push(temp); } else {unreachable!("cant cast");}  
							}
							}
							recog.base.set_state(148);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(151);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					recog.base.set_state(152);
					recog.base.match_token(ARROW,&mut recog.err_handler)?;

					/*InvokeRule stellatype*/
					recog.base.set_state(153);
					let tmp = recog.stellatype()?;
					if let StellatypeContextAll::TypeFunContext(ctx) = cast_mut::<_,StellatypeContextAll >(&mut _localctx){
					ctx.returnType = Some(tmp.clone()); } else {unreachable!("cant cast");}  

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
	\x45\u{9f}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x03\x02\x03\x02\x03\x02\x03\x03\
	\x03\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x05\x03\x05\x07\x05\x24\x0a\
	\x05\x0c\x05\x0e\x05\x27\x0b\x05\x03\x05\x07\x05\x2a\x0a\x05\x0c\x05\x0e\
	\x05\x2d\x0b\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x08\x07\x08\x3c\x0a\x08\x0c\
	\x08\x0e\x08\x3f\x0b\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\
	\x07\x08\x47\x0a\x08\x0c\x08\x0e\x08\x4a\x0b\x08\x05\x08\x4c\x0a\x08\x03\
	\x08\x03\x08\x03\x08\x05\x08\x51\x0a\x08\x03\x08\x03\x08\x05\x08\x55\x0a\
	\x08\x03\x08\x03\x08\x07\x08\x59\x0a\x08\x0c\x08\x0e\x08\x5c\x0b\x08\x03\
	\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\
	\x08\x03\x08\x05\x08\x69\x0a\x08\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\
	\u{8a}\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\
	\x0c\u{93}\x0a\x0c\x0c\x0c\x0e\x0c\u{96}\x0b\x0c\x05\x0c\u{98}\x0a\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x05\x0c\u{9d}\x0a\x0c\x03\x0c\x02\x02\x0d\x02\x04\x06\
	\x08\x0a\x0c\x0e\x10\x12\x14\x16\x02\x02\x02\u{a6}\x02\x18\x03\x02\x02\x02\
	\x04\x1b\x03\x02\x02\x02\x06\x1e\x03\x02\x02\x02\x08\x21\x03\x02\x02\x02\
	\x0a\x2e\x03\x02\x02\x02\x0c\x32\x03\x02\x02\x02\x0e\x68\x03\x02\x02\x02\
	\x10\x6a\x03\x02\x02\x02\x12\x6c\x03\x02\x02\x02\x14\u{89}\x03\x02\x02\x02\
	\x16\u{9c}\x03\x02\x02\x02\x18\x19\x05\x08\x05\x02\x19\x1a\x07\x02\x02\x03\
	\x1a\x03\x03\x02\x02\x02\x1b\x1c\x05\x14\x0b\x02\x1c\x1d\x07\x02\x02\x03\
	\x1d\x05\x03\x02\x02\x02\x1e\x1f\x05\x16\x0c\x02\x1f\x20\x07\x02\x02\x03\
	\x20\x07\x03\x02\x02\x02\x21\x25\x05\x0a\x06\x02\x22\x24\x05\x0c\x07\x02\
	\x23\x22\x03\x02\x02\x02\x24\x27\x03\x02\x02\x02\x25\x23\x03\x02\x02\x02\
	\x25\x26\x03\x02\x02\x02\x26\x2b\x03\x02\x02\x02\x27\x25\x03\x02\x02\x02\
	\x28\x2a\x05\x0e\x08\x02\x29\x28\x03\x02\x02\x02\x2a\x2d\x03\x02\x02\x02\
	\x2b\x29\x03\x02\x02\x02\x2b\x2c\x03\x02\x02\x02\x2c\x09\x03\x02\x02\x02\
	\x2d\x2b\x03\x02\x02\x02\x2e\x2f\x07\x30\x02\x02\x2f\x30\x07\x26\x02\x02\
	\x30\x31\x07\x04\x02\x02\x31\x0b\x03\x02\x02\x02\x32\x33\x07\x28\x02\x02\
	\x33\x34\x07\x3e\x02\x02\x34\x35\x07\x42\x02\x02\x35\x36\x07\x03\x02\x02\
	\x36\x37\x07\x42\x02\x02\x37\x38\x03\x02\x02\x02\x38\x39\x07\x04\x02\x02\
	\x39\x0d\x03\x02\x02\x02\x3a\x3c\x05\x10\x09\x02\x3b\x3a\x03\x02\x02\x02\
	\x3c\x3f\x03\x02\x02\x02\x3d\x3b\x03\x02\x02\x02\x3d\x3e\x03\x02\x02\x02\
	\x3e\x40\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\x40\x41\x07\x2b\x02\x02\
	\x41\x42\x07\x41\x02\x02\x42\x4b\x07\x05\x02\x02\x43\x48\x05\x12\x0a\x02\
	\x44\x45\x07\x03\x02\x02\x45\x47\x05\x12\x0a\x02\x46\x44\x03\x02\x02\x02\
	\x47\x4a\x03\x02\x02\x02\x48\x46\x03\x02\x02\x02\x48\x49\x03\x02\x02\x02\
	\x49\x4c\x03\x02\x02\x02\x4a\x48\x03\x02\x02\x02\x4b\x43\x03\x02\x02\x02\
	\x4b\x4c\x03\x02\x02\x02\x4c\x4d\x03\x02\x02\x02\x4d\x50\x07\x06\x02\x02\
	\x4e\x4f\x07\x0b\x02\x02\x4f\x51\x05\x16\x0c\x02\x50\x4e\x03\x02\x02\x02\
	\x50\x51\x03\x02\x02\x02\x51\x54\x03\x02\x02\x02\x52\x53\x07\x39\x02\x02\
	\x53\x55\x05\x16\x0c\x02\x54\x52\x03\x02\x02\x02\x54\x55\x03\x02\x02\x02\
	\x55\x56\x03\x02\x02\x02\x56\x5a\x07\x07\x02\x02\x57\x59\x05\x0e\x08\x02\
	\x58\x57\x03\x02\x02\x02\x59\x5c\x03\x02\x02\x02\x5a\x58\x03\x02\x02\x02\
	\x5a\x5b\x03\x02\x02\x02\x5b\x5d\x03\x02\x02\x02\x5c\x5a\x03\x02\x02\x02\
	\x5d\x5e\x07\x36\x02\x02\x5e\x5f\x05\x14\x0b\x02\x5f\x60\x07\x04\x02\x02\
	\x60\x61\x07\x08\x02\x02\x61\x69\x03\x02\x02\x02\x62\x63\x07\x3b\x02\x02\
	\x63\x64\x07\x41\x02\x02\x64\x65\x07\x09\x02\x02\x65\x66\x05\x16\x0c\x02\
	\x66\x67\x07\x04\x02\x02\x67\x69\x03\x02\x02\x02\x68\x3d\x03\x02\x02\x02\
	\x68\x62\x03\x02\x02\x02\x69\x0f\x03\x02\x02\x02\x6a\x6b\x07\x2f\x02\x02\
	\x6b\x11\x03\x02\x02\x02\x6c\x6d\x07\x41\x02\x02\x6d\x6e\x07\x0a\x02\x02\
	\x6e\x6f\x05\x16\x0c\x02\x6f\x13\x03\x02\x02\x02\x70\x71\x07\x2d\x02\x02\
	\x71\x72\x05\x14\x0b\x02\x72\x73\x07\x38\x02\x02\x73\x74\x05\x14\x0b\x02\
	\x74\x75\x07\x27\x02\x02\x75\x76\x05\x14\x0b\x02\x76\u{8a}\x03\x02\x02\x02\
	\x77\u{8a}\x07\x3a\x02\x02\x78\u{8a}\x07\x29\x02\x02\x79\u{8a}\x07\x43\x02\
	\x02\x7a\x7b\x07\x37\x02\x02\x7b\x7c\x07\x05\x02\x02\x7c\x7d\x05\x14\x0b\
	\x02\x7d\x7e\x07\x06\x02\x02\x7e\u{8a}\x03\x02\x02\x02\x7f\u{80}\x07\x1e\
	\x02\x02\u{80}\u{81}\x07\x05\x02\x02\u{81}\u{82}\x05\x14\x0b\x02\u{82}\u{83}\
	\x07\x03\x02\x02\u{83}\u{84}\x05\x14\x0b\x02\u{84}\u{85}\x07\x03\x02\x02\
	\u{85}\u{86}\x05\x14\x0b\x02\u{86}\u{87}\x07\x06\x02\x02\u{87}\u{8a}\x03\
	\x02\x02\x02\u{88}\u{8a}\x07\x41\x02\x02\u{89}\x70\x03\x02\x02\x02\u{89}\
	\x77\x03\x02\x02\x02\u{89}\x78\x03\x02\x02\x02\u{89}\x79\x03\x02\x02\x02\
	\u{89}\x7a\x03\x02\x02\x02\u{89}\x7f\x03\x02\x02\x02\u{89}\u{88}\x03\x02\
	\x02\x02\u{8a}\x15\x03\x02\x02\x02\u{8b}\u{9d}\x07\x20\x02\x02\u{8c}\u{9d}\
	\x07\x21\x02\x02\u{8d}\u{8e}\x07\x2b\x02\x02\u{8e}\u{97}\x07\x05\x02\x02\
	\u{8f}\u{94}\x05\x16\x0c\x02\u{90}\u{91}\x07\x03\x02\x02\u{91}\u{93}\x05\
	\x16\x0c\x02\u{92}\u{90}\x03\x02\x02\x02\u{93}\u{96}\x03\x02\x02\x02\u{94}\
	\u{92}\x03\x02\x02\x02\u{94}\u{95}\x03\x02\x02\x02\u{95}\u{98}\x03\x02\x02\
	\x02\u{96}\u{94}\x03\x02\x02\x02\u{97}\u{8f}\x03\x02\x02\x02\u{97}\u{98}\
	\x03\x02\x02\x02\u{98}\u{99}\x03\x02\x02\x02\u{99}\u{9a}\x07\x06\x02\x02\
	\u{9a}\u{9b}\x07\x0b\x02\x02\u{9b}\u{9d}\x05\x16\x0c\x02\u{9c}\u{8b}\x03\
	\x02\x02\x02\u{9c}\u{8c}\x03\x02\x02\x02\u{9c}\u{8d}\x03\x02\x02\x02\u{9d}\
	\x17\x03\x02\x02\x02\x0f\x25\x2b\x3d\x48\x4b\x50\x54\x5a\x68\u{89}\u{94}\
	\u{97}\u{9c}";

