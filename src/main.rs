mod ast;
mod build;
mod stellalexer;
mod stellaparser;
mod stellaparserlistener;
mod typecheck;

fn main() {
    println!("test started");

    // let input_data = "language core; fn main(x : Nat) -> Nat { return succ(x); }";
    let input_data: std::io::Result<String> = std::io::stdin().lines().collect();

    let input_stream = antlr_rust::InputStream::new(input_data.as_ref().unwrap().as_str());
    let lexer = stellalexer::stellaLexer::new(input_stream);
    let token_stream = antlr_rust::common_token_stream::CommonTokenStream::new(lexer);
    let mut parser = stellaparser::stellaParser::new(token_stream);

    println!("\nstart parsing parser_test_csv");
    let result = parser.program();

    let program = build::build_program(&*result.expect("Parse Error"));

    typecheck::typecheck_program(&program).expect("Type Error");

    println!("Everything looks fine!");
}
