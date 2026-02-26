use crate::{
    error::{self, Error},
    fs::File,
    token::{self, Token},
    util::{Location, Region},
};
use std::{io::Read, iter::Peekable, path::PathBuf, str::Chars};

fn error(region: Region, message: String) -> Error {
    Error {
        message: message,
        level: error::Level::Lexer,
        region: Some(region),
    }
}

struct Lexer<'a> {
    location: Location,
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn run_analysis(&mut self) -> Result<Vec<Token>, Error> {
        Err(error(self.current_region(), "not implemented".to_string()))
    }

    fn current_region(&self) -> Region {
        Region::new(self.location.clone(), self.location.clone())
    }

    pub fn new(filepath: PathBuf, source: &'a str) -> Self {
        Self {
            location: Location::new(filepath, 1, 1),
            iter: source.chars().peekable(),
        }
    }
}

pub fn lex(filepath: PathBuf) -> Result<Vec<Token>, Error> {
    let mut file = File::open(&filepath)?;

    let mut source = String::new();
    file.read_to_string(&mut source)?;

    let mut lexer = Lexer::new(filepath, &source);
    lexer.run_analysis()
}
