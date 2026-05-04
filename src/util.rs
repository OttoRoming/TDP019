use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt,
    ops::Add,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub start: Location,
    pub end: Location,
}

impl Region {
    pub fn contains(&self, location: Location) -> bool {
        location >= self.start && location < self.end
    }

    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }

    #[allow(dead_code)]
    pub fn newi(
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
    ) -> Self {
        Self {
            start: Location::new(start_line, start_column),
            end: Location::new(end_line, end_column),
        }
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

#[derive(Debug, PartialEq, Eq, Clone)]
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

// https://stackoverflow.com/questions/28005134/how-do-i-implement-the-add-trait-for-a-reference-to-a-struct
impl Add<usize> for Location {
    type Output = Location;

    fn add(self, rhs: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + rhs,
        }
    }
}
