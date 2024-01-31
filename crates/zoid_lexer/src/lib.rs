use std::{iter::Peekable, str::Chars};

use token::{Token, TokenKind};

pub mod token;

impl<'fname, 'source> Iterator for Lexer<'fname, 'source> {
    type Item = Token<'fname, 'source>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Debug, Clone)]
pub struct Lexer<'fname, 'source> {
    file: &'fname str,
    source: &'source str,
    chars: Peekable<Chars<'source>>,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'fname, 'source> Lexer<'fname, 'source> {
    pub fn new(file: &'fname str, source: &'source str) -> Self {
        Self {
            file,
            source,
            chars: source.chars().peekable(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn reset(&mut self) {
        self.chars = self.source.chars().peekable();
        self.pos = 0;
        self.line = 1;
        self.column = 1;
    }

    pub fn next_token(&mut self) -> Option<Token<'fname, 'source>> {
        self.consume_whitespace();
        let start = self.pos;
        let line = self.line;
        let col = self.column;

        match self.next_char()? {
            '+' => self.tok(start, line, col, TokenKind::OpAdd),
            '-' => self.tok(start, line, col, TokenKind::OpSub),
            '*' => self.tok(start, line, col, TokenKind::OpMul),
            '/' => match self.peek_char() {
                Some('/') => {
                    self.next_char();
                    self.consume_line_comment();
                    self.next_token()
                }
                Some('*') => {
                    self.next_char();
                    self.consume_block_comment();
                    self.next_token()
                }
                _ => self.tok(start, line, col, TokenKind::OpDiv),
            },
            '%' => self.tok(start, line, col, TokenKind::OpRem),

            '=' => self.tok(start, line, col, TokenKind::OpAssign),

            ';' => self.tok(start, line, col, TokenKind::Semicolon),
            ':' => self.tok(start, line, col, TokenKind::Colon),
            ',' => self.tok(start, line, col, TokenKind::Comma),
            '(' => self.tok(start, line, col, TokenKind::LParen),
            ')' => self.tok(start, line, col, TokenKind::RParen),
            '{' => self.tok(start, line, col, TokenKind::LBrace),
            '}' => self.tok(start, line, col, TokenKind::RBrace),

            '0'..='9' => self.tokenize_numeric_literal(start, line, col),
            'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier(start, line, col),

            _ => self.tok(start, line, col, TokenKind::Unknown),
        }
    }

    fn tokenize_numeric_literal(
        &mut self,
        start: usize,
        line: usize,
        col: usize,
    ) -> Option<Token<'fname, 'source>> {
        self.consume_numeric_literal();
        let value = &self.source[start..self.pos];
        self.tok_v(start, line, col, TokenKind::IntegerLiteral, value)
    }

    fn tokenize_identifier(
        &mut self,
        start: usize,
        line: usize,
        col: usize,
    ) -> Option<Token<'fname, 'source>> {
        self.consume_identifier();
        let value = &self.source[start..self.pos];

        let kind = match value {
            "i8" => TokenKind::TypeI8,
            "i16" => TokenKind::TypeI16,
            "i32" => TokenKind::TypeI32,
            "i64" => TokenKind::TypeI64,
            "i128" => TokenKind::TypeI128,
            "u8" => TokenKind::TypeU8,
            "u16" => TokenKind::TypeU16,
            "u32" => TokenKind::TypeU32,
            "u64" => TokenKind::TypeU64,
            "u128" => TokenKind::TypeU128,
            "f32" => TokenKind::TypeF32,
            "f64" => TokenKind::TypeF64,
            "fn" => TokenKind::KeywordFn,
            "let" => TokenKind::KeywordLet,
            "return" => TokenKind::KeywordReturn,
            _ => TokenKind::Identifier,
        };
        if kind == TokenKind::Identifier {
            self.tok_v(start, line, col, kind, value)
        } else {
            self.tok(start, line, col, kind)
        }
    }

    fn tok(
        &self,
        start: usize,
        line: usize,
        col: usize,
        kind: TokenKind,
    ) -> Option<Token<'fname, 'source>> {
        Some(Token {
            file: self.file,
            start,
            end: self.pos,
            line,
            column: col,
            kind,
            value: None,
        })
    }

    fn tok_v(
        &self,
        start: usize,
        line: usize,
        col: usize,
        kind: TokenKind,
        value: &'source str,
    ) -> Option<Token<'fname, 'source>> {
        Some(Token {
            file: self.file,
            start,
            end: self.pos,
            line,
            column: col,
            kind,
            value: Some(value),
        })
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += 1;
        self.column += 1;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        }
        Some(c)
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn consume_while<F>(&mut self, mut f: F) -> &'source str
    where
        F: FnMut(char) -> bool,
    {
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if !f(c) {
                break;
            }
            self.next_char();
        }
        &self.source[start..self.pos]
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_line_comment(&mut self) {
        self.consume_while(|c| c != '\n');
    }

    fn consume_block_comment(&mut self) {
        let mut depth = 1;
        while depth > 0 {
            let c = self.next_char().unwrap();
            match c {
                '/' => {
                    if self.peek_char() == Some('*') {
                        self.next_char();
                        depth += 1;
                    }
                }
                '*' => {
                    if self.peek_char() == Some('/') {
                        self.next_char();
                        depth -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn consume_identifier(&mut self) -> &'source str {
        self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    fn consume_numeric_literal(&mut self) -> &'source str {
        self.consume_while(|c| c.is_ascii_digit())
    }
}
