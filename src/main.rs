pub mod parser;
pub mod scanner;
use std::{env, io::Write, process};

use parser::{interpreter::Interpreter, parser::Parser};

fn report_error(line: i32, message: &str) {
    println!("[line {line}] Error: {message}");
}
fn main() {
    let args: Vec<String> = env::args().collect();

    // let expression: Expr = Expr::Binary(Binary {
    //     operator: Token {
    //         token_type: TokenType::STAR,
    //         lexeme: String::from("*"),
    //         literal: LiteralType::String(String::from("")),
    //         line: 1,
    //     },
    //     left: Box::new(Expr::Unary(Unary {
    //         operator: Token {
    //             token_type: TokenType::MINUS,
    //             lexeme: String::from("-"),
    //             literal: LiteralType::String(String::from("")),
    //             line: 1,
    //         },
    //         right: Box::new(Expr::Literal(Literal {
    //             value: LiteralType::Float(123.0),
    //         })),
    //     })),
    //     right: Box::new(Expr::Grouping(Grouping {
    //         expression: Box::new(Expr::Literal(Literal {
    //             value: LiteralType::Float(45.67),
    //         })),
    //     })),
    // });

    if args.len() > 2 {
        println!("Too many arguments");
        process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &String) {
    let content = std::fs::read_to_string(file_path).expect("File not found or something");
    println!("Running file {}", file_path);
    run(content);
}

fn run_prompt() {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        stdin.read_line(&mut line).expect("error when reading line");
        if line.trim() == "exit" {
            break;
        }
        run(line)
    }
}

fn run(source: String) {
    let mut scanner = scanner::scanner::new(source);

    let tokens = scanner.scan_tokens();
    for t in tokens.iter() {
        println!("{:?}", t);
    }
    // TO DO - return error from scan_tokens();

    let mut parser: Parser = Parser::new(tokens.clone());
    let expression = match parser.parse() {
        Err(_) => return,
        Ok(e) => e,
    };

    let mut interpreter = Interpreter::new();
    match interpreter.interpret(expression) {
        Err(e) => report_error(e.line, &e.message),
        _ => (),
    }
    // let mut printer: AstPrinter = AstPrinter {};
    // println!("{}", printer.print(expression).unwrap());
    // for t in tokens.iter() {
    //     println!("{:?}", t);
    // }
}
