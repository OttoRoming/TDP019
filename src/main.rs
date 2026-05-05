mod ast;
mod checker;
mod error;
mod evaluator;
mod lexer;
mod parser;
mod token;
mod util;

use clap::Parser;
use evaluator::{eval, values::Value};
use std::{fs, io::Read, path::PathBuf, process::exit};

// https://rust-cli.github.io/book/tutorial/cli-args.html
// https://docs.rs/clap/latest/clap/
/// The Oeno language interpreter
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path for the source code file to execute
    path: PathBuf,
}

fn main() {
    color_backtrace::install();

    let args = Cli::parse();

    let mut file = match fs::File::open(&args.path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("failed to open file, ({})", e);
            exit(1);
        }
    };

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read source file content");

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
