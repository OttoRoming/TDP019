mod error;
mod lexer;
mod token;
mod util;

use std::{env, error::Error, fs, io::Read, path::PathBuf};

fn main() {
    let filepath_string = env::args()
        .nth(2)
        .expect("no filename provided with cli arguments");

    let filepath = PathBuf::from(filepath_string);
    match lexer::lex(&filepath) {
        Ok(tokens) => {
            dbg!(tokens);
        }
        Err(err) => {
            err.print();
        }
    };
}
