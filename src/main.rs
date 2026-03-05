mod ast;
mod error;
mod lexer;
mod parser;
mod token;
mod util;

use std::{env, fs, io::Read, path::PathBuf};

fn main() {
    let filepath_string = env::args()
        .nth(1)
        .expect("no filename provided with cli arguments");

    let filepath = PathBuf::from(filepath_string);

    let mut file = fs::File::open(&filepath).expect("failed to open source file");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read source file content");

    match lexer::lex(&source) {
        Ok(tokens) => {
            dbg!(tokens);
        }
        Err(err) => {
            err.print(&source);
        }
    };
}
