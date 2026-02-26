use crate::util::Region;
use std::{convert::From, error, fmt, io};

use owo_colors::OwoColorize;

#[derive(Debug)]
pub enum Level {
    IO,
    Lexer,
    Parse,
    Check,
    Evaluation,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Error<'a> {
    pub message: String,
    pub level: Level,
    pub region: Option<Region<'a>>,
}

impl<'a> error::Error for Error<'a> {}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} error: {}", self.level, self.message)
    }
}

impl<'a> From<io::Error> for Error<'a> {
    fn from(value: io::Error) -> Self {
        Self {
            message: format!("{}", value),
            level: Level::IO,
            region: None,
        }
    }
}

impl<'a> Error<'a> {
    fn print_message(&self) {
        eprintln!(
            "{} {}: {}",
            self.level.yellow().bold(),
            "error".red(),
            self.message.cyan()
        )
    }
    pub fn print(&self) {
        self.print_message();
    }
}
