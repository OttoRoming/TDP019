use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    pub start: Location,
    pub end: Location,
}

impl Region {
    pub fn contains(&self, location: Location) -> bool {
        location >= self.start && location <= self.end
    }

    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(
                f,
                "{}:{} -> {}:{}",
                self.start.line, self.start.column, self.end.line, self.end.column
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.line.cmp(&other.line), self.column.cmp(&other.column)) {
            (Ordering::Equal, ord) => ord,
            (ord, _) => ord,
        }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
