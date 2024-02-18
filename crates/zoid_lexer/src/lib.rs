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
pub struct ZoidLexer<'fname, 'source> {
    #[allow(unused)]
    /// The input used to match keywords
    input: &'source str,
    /// The iterator that is used for traversing the source
    source: Chars<'source>,
    /// The current location within the input
    location: ZoidLocation<'fname>,
}

impl<'fname, 'source> Iterator for ZoidLexer<'fname, 'source> {
    type Item = ZoidToken<'fname>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize().ok().flatten()
    }
}

impl<'fname, 'source> ZoidLexer<'fname, 'source> {
    /// Create a new lexer for the Zoid language
    ///
    /// The `input` should be the content of a file containing Zoid source code
    ///
    /// The `file_name` should contain the file name that the source code originates from.
    /// *This is only used for diagnostic reporting*
    pub fn new(file_name: &'fname str, input: &'source str) -> Self {
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
    pub fn tokenize(&mut self) -> Result<Option<ZoidToken<'fname>>, String> {
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
            '/' => match self.peek_char() {
                Some('/') => self.consume_line_comment(),
                Some('*') => self.consume_block_comment(),
                _ => ZoidTokenKind::OpDiv,
            },
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
            'c' => match self.peek_char() {
                Some('"') => {
                    self.next_char();
                    self.consume_string_literal();
                    ZoidTokenKind::CStringLiteral
                }
                _ => self.consume_identifier(&start),
            },
            'r' => match self.peek_char() {
                Some('#') => self.consume_raw_string_literal(),
                _ => self.consume_identifier(&start),
            },
            // TODO: Implement Unicode XID
            'a'..='z' | 'A'..='Z' | '_' => self.consume_identifier(&start),
            '"' => self.consume_string_literal(),
            '\'' => self.consume_character_literal(),
            '0'..='9' => self.consume_numeric_literal(),
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
            }
            break;
        }
    }

    fn consume_identifier(&mut self, start: &ZoidLocation) -> ZoidTokenKind {
        while let Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') = self.peek_char() {
            self.next_char();
        }

        match &self.input[start.start..self.location.end] {
            "if" => ZoidTokenKind::KWIf,
            "else" => ZoidTokenKind::KWElse,
            "fn" => ZoidTokenKind::KWFn,
            "let" => ZoidTokenKind::KWLet,
            "return" => ZoidTokenKind::KWReturn,
            "for" => ZoidTokenKind::KWFor,
            "while" => ZoidTokenKind::KWWhile,
            "break" => ZoidTokenKind::KWBreak,
            "continue" => ZoidTokenKind::KWContinue,
            "in" => ZoidTokenKind::KWIn,
            "struct" => ZoidTokenKind::KWStruct,
            "enum" => ZoidTokenKind::KWEnum,
            "union" => ZoidTokenKind::KWUnion,
            "impl" => ZoidTokenKind::KWImpl,
            "trait" => ZoidTokenKind::KWTrait,
            "where" => ZoidTokenKind::KWWhere,
            "async" => ZoidTokenKind::KWAsync,
            "await" => ZoidTokenKind::KWAwait,
            "gen" => ZoidTokenKind::KWGen,
            "yield" => ZoidTokenKind::KWYield,
            "import" => ZoidTokenKind::KWImport,
            "importc" => ZoidTokenKind::KWImportC,
            "extern" => ZoidTokenKind::KWExtern,

            "and" => ZoidTokenKind::OpAnd,
            "or" => ZoidTokenKind::OpOr,
            "not" => ZoidTokenKind::OpNot,

            "true" => ZoidTokenKind::BoolLitTrue,
            "false" => ZoidTokenKind::BoolLitFalse,

            _ => ZoidTokenKind::Identifier,
        }
    }

    fn consume_string_literal(&mut self) -> ZoidTokenKind {
        while let Some(c) = self.peek_char() {
            self.next_char();
            if c == '"' {
                break;
            } else if c == '\\' {
                self.next_char();
            }
        }

        ZoidTokenKind::StringLiteral
    }

    fn consume_raw_string_literal(&mut self) -> ZoidTokenKind {
        let mut count = 0;
        while let Some(c) = self.peek_char() {
            if c == '#' {
                count += 1;
                self.next_char();
            } else {
                break;
            }
        }

        while let Some(c) = self.peek_char() {
            self.next_char();
            if c == '"' {
                let mut count2 = 0;
                while let Some(c) = self.peek_char() {
                    if c == '#' {
                        count2 += 1;
                        self.next_char();
                    } else {
                        break;
                    }
                }

                if count == count2 {
                    break;
                }
            }
        }

        ZoidTokenKind::RawStringLiteral
    }

    fn consume_character_literal(&mut self) -> ZoidTokenKind {
        while let Some(c) = self.peek_char() {
            self.next_char();
            if c == '\'' {
                break;
            } else if c == '\\' {
                self.next_char();
            }
        }

        ZoidTokenKind::CharLiteral
    }

    fn consume_numeric_literal(&mut self) -> ZoidTokenKind {
        let mut is_float = false;
        while let Some('0'..='9') = self.peek_char() {
            self.next_char();
        }

        if let Some('.') = self.peek_char() {
            self.next_char();

            while let Some('0'..='9') = self.peek_char() {
                self.next_char();
            }
            is_float = true;
        }

        if let Some('e') | Some('E') = self.peek_char() {
            self.next_char();
            if let Some('+') | Some('-') = self.peek_char() {
                self.next_char();
            }
            while let Some('0'..='9') = self.peek_char() {
                self.next_char();
            }
            is_float = true;
        }

        if is_float {
            ZoidTokenKind::FloatLiteral
        } else {
            ZoidTokenKind::IntLiteral
        }
    }

    fn consume_line_comment(&mut self) -> ZoidTokenKind {
        while let Some(c) = self.next_char() {
            if c == '\n' {
                break;
            }
        }

        ZoidTokenKind::LineComment
    }

    fn consume_block_comment(&mut self) -> ZoidTokenKind {
        let mut depth = 1;
        while let Some(c) = self.next_char() {
            if c == '/' {
                if let Some('*') = self.peek_char() {
                    self.next_char();
                    depth += 1;
                }
            } else if c == '*' {
                if let Some('/') = self.peek_char() {
                    self.next_char();
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
        }

        ZoidTokenKind::BlockComment
    }
}
