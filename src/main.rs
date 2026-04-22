mod ast;
mod checker;
mod error;
mod evaluator;
mod lexer;
mod parser;
mod token;
mod util;

use evaluator::{eval, values::Value};
use std::{env, fs, io::Read, path::PathBuf, process::exit};

fn main() {
    color_backtrace::install();

    let filepath_string = env::args()
        .nth(1)
        .expect("no filename provided with cli arguments");

    let filepath = PathBuf::from(filepath_string);

    let mut file = fs::File::open(&filepath).expect("failed to open source file");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read source file content");

    // match lexer::lex(&source) {
    //     Ok(tokens) => {
    //         dbg!(tokens);
    //     }
    //     Err(err) => {
    //         err.print(&source);
    //     }
    // };

    // match parser::parse(&source) {
    //     Ok(ast) => {
    //         dbg!(&ast);
    //     }
    //     Err(err) => {
    //         err.print(&source);
    //         panic!()
    //     }
    // };

    match eval(&source) {
        Ok(result) => match result {
            Value::Int(i) => exit(i as i32),
            _ => exit(1),
        },
        Err(err) => {
            err.print(&source);
            exit(1);
        }
    };
}
