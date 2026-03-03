use crate::util::{Location, Region};
use std::{convert::From, error, fmt, io, path::PathBuf};

use owo_colors::OwoColorize;

#[derive(Debug)]
#[allow(dead_code)]
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
pub struct Error {
    pub message: String,
    pub level: Level,
    pub path: Option<PathBuf>,
    pub region: Option<Region>,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} error: {}", self.level, self.message)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self {
            message: format!("{}", value),
            level: Level::IO,
            path: None,
            region: None,
        }
    }
}

impl Error {
    fn print_message(&self) {
        eprintln!(
            "{} {}: {}",
            self.level.yellow().bold(),
            "error".red().bold(),
            self.message.cyan()
        )
    }

    pub fn print(&self, source: &str) {
        self.print_message();

        if let Some(region) = self.region {
            match &self.path {
                Some(path) => eprintln!(
                    "  {} {}:{}",
                    "-->".blue().bold(),
                    path.as_os_str().to_string_lossy(),
                    region
                ),
                None => eprintln!("  {} {}", "-->".blue().bold(), region),
            }

            let lines = source.lines();

            // skip to one line above the error
            let skip_amount = region.start.line.saturating_sub(2);
            let print_amount = region.end.line - region.start.line + 3;
            let gutter_width = (region.end.line.ilog10() + 1) as usize;

            for (line_index_raw, line) in lines.skip(skip_amount).take(print_amount).enumerate() {
                let line_index = line_index_raw + skip_amount + 1;
                eprint!(
                    "{:gutter_width$} {} ",
                    line_index.blue().bold(),
                    "|".blue().bold()
                );

                for (char_index, char) in line.chars().enumerate() {
                    let location = Location::new(line_index, char_index + 1);
                    if region.contains(location) {
                        eprint!("{}", char.red().bold())
                    } else {
                        eprint!("{}", char)
                    }
                }
                eprintln!();
            }
        }
    }
}
