use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt,
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region<'a> {
    pub start: Location<'a>,
    pub end: Location<'a>,
}

impl<'a> Region<'a> {
    pub fn contains(&self, location: Location) -> bool {
        location >= self.start && location <= self.end
    }

    pub fn new(start: Location<'a>, end: Location<'a>) -> Self {
        Self { start, end }
    }
}

impl<'a> fmt::Display for Region<'a> {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Location<'a> {
    pub path: &'a Path,
    pub line: usize,
    pub column: usize,
}

impl<'a> Location<'a> {
    pub fn new(path: &'a Path, line: usize, column: usize) -> Self {
        Self { path, line, column }
    }
}

impl<'a> fmt::Display for Location<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_string = self.path.as_os_str().to_string_lossy();
        write!(f, "{}:{}:{}", path_string, self.line, self.column)
    }
}

impl<'a> Ord for Location<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.line.cmp(&other.line), self.column.cmp(&other.column)) {
            (Ordering::Equal, ord) => ord,
            (ord, _) => ord,
        }
    }
}

impl<'a> PartialOrd for Location<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
