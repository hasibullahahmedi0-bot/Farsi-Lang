use std::env;
use std::fs;
use std::process;

mod lexer;
mod parser;
mod compiler;
mod runtime;

use lexer::Lexer;
use parser::Parser;
use compiler::Compiler;
use runtime::Runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("استفاده: farsi <فایل.fr>");
        process::exit(1);
    }
    
    let filename = &args[1];
    
    let source = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("خطا: نمی‌توان فایل '{}' را باز کرد - {}", filename, err);
            process::exit(1);
        }
    };
    
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(toks) => toks,
        Err(err) => {
            eprintln!("خطای Lexer: {}", err);
            process::exit(1);
        }
    };
    
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("خطای Parser: {}", err);
            process::exit(1);
        }
    };
    
    let mut compiler = Compiler::new();
    let bytecode = match compiler.compile(&ast) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("خطای Compiler: {}", err);
            process::exit(1);
        }
    };
    
    let mut runtime = Runtime::new();
    if let Err(err) = runtime.execute(&bytecode) {
        eprintln!("خطای Runtime: {}", err);
        process::exit(1);
    }
}
