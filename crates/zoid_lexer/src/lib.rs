#![deny(missing_docs, missing_debug_implementations)]

//! # `zoid_lexer`
//!
//! Iterator style lexer that generates the tokens for the Zoid language

use std::str::Chars;

pub use token::ZoidToken;
use zoid_location::ZoidLocation;

use crate::token::ZoidTokenKind;

#[cfg(test)]
mod tests;
mod token;

#[derive(Debug, Clone)]
/// The lexer for the Zoid language. Used to generate tokens which are then parsed into an AST
pub struct ZoidLexer<'source, 'fname> {
    #[allow(unused)]
    /// The input used to match keywords
    input: &'source str,
    /// The iterator that is used for traversing the source
    source: Chars<'source>,
    /// The current location within the input
    location: ZoidLocation<'fname>,
}

impl<'source, 'fname> ZoidLexer<'source, 'fname> {
    /// Create a new lexer for the Zoid language
    ///
    /// The `input` should be the content of a file containing Zoid source code
    ///
    /// The `file_name` should contain the file name that the source code originates from.
    /// *This is only used for diagnostic reporting*
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

    /// Get the next token from the lexer
    ///
    /// If it finds a valid token then it will return `Ok(Some(token))`,
    /// otherwise if it has reached the end of the file it will return `Ok(None)`,
    /// however, when encountering an unknown symbol it will return the `Err` variant.
    ///
    /// This can be ignored by calling `tokenize` again to get the next token.
    pub fn tokenize(&mut self) -> Result<Option<ZoidToken>, String> {
        self.consume_whitespace();
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
                Some('=') => {
                    self.next_char();
                    ZoidTokenKind::OpLeq
                }
                Some('<') => {
                    self.next_char();
                    ZoidTokenKind::OpShl
                }
                _ => ZoidTokenKind::OpLt,
            },
            ':' => match self.peek_char() {
                Some('>') => {
                    self.next_char();
                    ZoidTokenKind::RGenericBracket
                }
                _ => ZoidTokenKind::Colon,
            },
            ',' => ZoidTokenKind::Comma,
            ';' => ZoidTokenKind::Semicolon,
            '>' => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    ZoidTokenKind::OpGeq
                }
                Some('>') => {
                    self.next_char();
                    ZoidTokenKind::OpShr
                }
                _ => ZoidTokenKind::OpGt,
            },
            '=' => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    ZoidTokenKind::OpEq
                }
                _ => ZoidTokenKind::OpAssign,
            },
            '!' => match self.peek_char() {
                Some('=') => {
                    self.next_char();
                    ZoidTokenKind::OpNe
                }
                _ => ZoidTokenKind::OpNot,
            },
            '&' => match self.peek_char() {
                Some('&') => {
                    self.next_char();
                    ZoidTokenKind::OpAnd
                }
                _ => ZoidTokenKind::OpBitAnd,
            },
            '|' => match self.peek_char() {
                Some('|') => {
                    self.next_char();
                    ZoidTokenKind::OpOr
                }
                _ => ZoidTokenKind::OpBitOr,
            },
            '~' => ZoidTokenKind::OpBitNot,
            '+' => ZoidTokenKind::OpAdd,
            '-' => ZoidTokenKind::OpSub,
            '*' => ZoidTokenKind::OpMul,
            '/' => ZoidTokenKind::OpDiv,
            '%' => ZoidTokenKind::OpRem,
            '.' => match self.peek_char() {
                Some('&') => {
                    self.next_char();
                    ZoidTokenKind::OpAddr
                }
                Some('*') => {
                    self.next_char();
                    ZoidTokenKind::OpDeref
                }
                Some('?') => {
                    self.next_char();
                    ZoidTokenKind::OpUnwrap
                }
                Some('.') => {
                    self.next_char();
                    match self.peek_char() {
                        Some('<') => {
                            self.next_char();
                            ZoidTokenKind::OpRangeExclusive
                        }
                        Some('.') => {
                            self.next_char();
                            ZoidTokenKind::VaArgs
                        }
                        Some('=') => {
                            self.next_char();
                            ZoidTokenKind::OpRangeInclusive
                        }
                        _ => ZoidTokenKind::OpRangeExclusive,
                    }
                }
                _ => ZoidTokenKind::OpDot,
            },
            _ => return Err(String::from("Unknown Character in Input")),
        };

        Ok(Some(ZoidToken {
            location: start.extend_range(self.location.end),
            kind,
        }))
    }

    /// Get the next char from the source and update the location to match
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

    /// Peek the next char, does not affect the current position
    fn peek_char(&mut self) -> Option<char> {
        self.source.clone().next()
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
                continue;
            } else {
                break;
            }
        }
    }
}
