use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZoidLocation<'fname> {
    pub file_name: &'fname str,
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

impl<'fname> ZoidLocation<'fname> {
    pub fn new(file_name: &'fname str, line: usize, column: usize, range: Range<usize>) -> Self {
        Self {
            file_name,
            line,
            column,
            start: range.start,
            end: range.end,
        }
    }

    pub fn extend_range(self, end: usize) -> Self {
        Self {
            file_name: self.file_name,
            line: self.line,
            column: self.column,
            start: self.start,
            end,
        }
    }

    pub fn new_range(self, range: Range<usize>) -> Self {
        Self {
            file_name: self.file_name,
            line: self.line,
            column: self.column,
            start: range.start,
            end: range.end,
        }
    }

    pub fn new_line(self, line: usize) -> Self {
        Self {
            file_name: self.file_name,
            line,
            column: self.column,
            start: self.start,
            end: self.end,
        }
    }

    pub fn new_col(self, col: usize) -> Self {
        Self {
            file_name: self.file_name,
            line: self.line,
            column: col,
            start: self.start,
            end: self.end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> ZoidLocation<'static> {
        ZoidLocation {
            file_name: "test",
            line: 1,
            column: 1,
            start: 0,
            end: 0,
        }
    }

    #[test]
    fn new_range() {
        let loc = init();

        assert_eq!(loc.new_range(20..50), ZoidLocation {
            file_name: "test",
            line: 1,
            column: 1,
            start: 20,
            end: 50,
        })
    }

    #[test]
    fn extend_range() {
        let loc = init();

        assert_eq!(loc.extend_range(20), ZoidLocation {
            file_name: "test",
            line: 1,
            column: 1,
            start: 0,
            end: 20,
        })
    }

    #[test]
    fn new_line() {
        let loc = init();

        assert_eq!(loc.new_line(20), ZoidLocation {
            file_name: "test",
            line: 20,
            column: 1,
            start: 0,
            end: 0,
        })
    }

    #[test]
    fn new_col() {
        let loc = init();

        assert_eq!(loc.new_col(20), ZoidLocation {
            file_name: "test",
            line: 1,
            column: 20,
            start: 0,
            end: 0,
        })
    }
}