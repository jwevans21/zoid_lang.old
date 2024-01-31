use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'fname, 'source> {
    pub file: &'fname str,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub kind: TokenKind,
    pub value: Option<&'source str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Unknown,

    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpRem,

    OpAssign,

    Semicolon,
    Colon,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Identifier,
    IntegerLiteral,

    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeI128,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeU128,
    TypeF32,
    TypeF64,
    
    TypeVoid,

    KeywordFn,
    KeywordLet,
    KeywordReturn,
}

impl Display for Token<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "[ {}:{}:{}, {:?}",
            self.file, self.line, self.column, self.kind
        )?;

        if let Some(value) = self.value {
            write!(f, ", \"{}\"", value)?;
        }

        write!(f, " ]")
    }
}
