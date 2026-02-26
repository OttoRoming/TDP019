use std::{fmt, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Region {
    start: Location,
    end: Location,
}

impl Region {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else if self.start.path == self.end.path {
            let path_string = self.start.path.as_os_str().to_string_lossy();
            write!(
                f,
                "{}:({}:{} -> {}:{})",
                path_string, self.start.line, self.start.column, self.end.line, self.end.column
            )
        } else {
            write!(f, "{} -> {}", self.start, self.end)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Location {
    path: PathBuf,
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(path: PathBuf, line: usize, column: usize) -> Self {
        Self { path, line, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_string = self.path.as_os_str().to_string_lossy();
        write!(f, "{}:{}:{}", path_string, self.line, self.column)
    }
}
