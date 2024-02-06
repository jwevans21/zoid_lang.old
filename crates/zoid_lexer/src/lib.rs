use std::str::Chars;

pub use token::ZoidToken;
use zoid_location::ZoidLocation;

use crate::token::ZoidTokenKind;

mod token;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct ZoidLexer<'source, 'fname> {
    input: &'source str,
    source: Chars<'source>,
    location: ZoidLocation<'fname>,
}

impl<'source, 'fname> ZoidLexer<'source, 'fname> {
    pub fn new(input: &'source str, file_name: &'fname str) -> Self {
        Self {
            input,
            source: input.chars(),
            location: ZoidLocation {
                file_name,
                line: 1,
                column: 1,
                start: 0,
                end: 0,
            },
        }
    }

    pub fn tokenize(&mut self) -> Result<Option<ZoidToken>, String> {
        let start = self.location;

        let c = if let Some(c) = self.next_char() {
            c
        } else {
            return Ok(None);
        };

        let kind = match c {
            '(' => ZoidTokenKind::LParen,
            ')' => ZoidTokenKind::RParen,
            '{' => ZoidTokenKind::LBrace,
            '}' => ZoidTokenKind::RBrace,
            '[' => ZoidTokenKind::LBracket,
            ']' => ZoidTokenKind::RBracket,
            '<' => match self.peek_char() {
                Some(':') => {
                    self.next_char();
                    ZoidTokenKind::LGenericBracket
                }
                _ => todo!("Add Less Than Operator")
            }
            ':' => match self.peek_char() {
                Some('>') => {
                    self.next_char();
                    ZoidTokenKind::RGenericBracket
                }
                _ => todo!("Add Colon")
            }
            _ => return Err(String::from("Unknown Character in Input"))
        };

        Ok(Some(ZoidToken {
            location: start.extend_range(self.location.end),
            kind,
        }))
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.source.next()?;
        self.location.start += c.len_utf8();
        self.location.end += c.len_utf8();

        if c == '\n' {
            self.location.line += 1;
            self.location.column = 1;
        } else {
            self.location.column += 1;
        }

        Some(c)
    }

    fn peek_char(&mut self) -> Option<char> {
        self.source.clone().next()
    }
}