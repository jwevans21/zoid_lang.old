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
