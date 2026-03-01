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
    pub fn print(&self, source: &str) {
        self.print_message();

        if let Some(region) = self.region {
            eprintln!("  {} {}", "-->".dimmed(), region);
            let lines = source.lines();

            // skip to one line above the error
            let skip_amount = region.start.line.checked_sub(2).unwrap_or(0);
            let print_amount = region.end.line - region.start.line + 2;
            let gutter_width = (region.end.line.ilog10() + 1) as usize;

            // print the error lines + 2
            let mut line_index = skip_amount;
            for line in lines.skip(skip_amount).take(print_amount) {
                eprintln!(
                    "{:gutter_width$} {} {}",
                    line_index.dimmed(),
                    "|".dimmed(),
                    line
                );

                line_index += 1;
            }
        }
    }
}
