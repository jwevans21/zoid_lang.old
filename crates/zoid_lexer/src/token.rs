use std::fmt::{Display, Formatter, Result as FmtResult};

use zoid_location::ZoidLocation;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// A token that contains the original source location (for diagnostics) and the kind of token.
/// If the token has associated data it needs to be retrieved from the input
pub struct ZoidToken<'fname> {
    /// The location where this token originated
    pub location: ZoidLocation<'fname>,
    /// The kind of token that is present at that location
    pub kind: ZoidTokenKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// The kinds of tokens known to the Zoid language
pub enum ZoidTokenKind {
    EOF,

    RParen,
    LParen,
    RBrace,
    LBrace,
    RBracket,
    LBracket,
    LGenericBracket,
    RGenericBracket,
    Colon,
    Semicolon,
    Comma,

    OpLeq,
    OpLt,
    OpGeq,
    OpGt,
    OpEq,
    OpAssign,
    OpNe,
    OpNot,
    OpAnd,
    OpBitAnd,
    OpOr,
    OpBitOr,
    OpBitNot,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpRem,
    OpShl,
    OpShr,
    OpAddr,
    OpDeref,
    OpUnwrap,
    OpRangeExclusive,
    VaArgs,
    OpRangeInclusive,
    OpDot,
    Identifier,
    KWIf,
    KWElse,
    KWFn,
    KWLet,
    KWReturn,
    KWFor,
    KWWhile,
    KWBreak,
    KWContinue,
    KWIn,
    KWStruct,
    KWEnum,
    KWUnion,
    KWImpl,
    KWTrait,
    KWWhere,
    KWAsync,
    KWAwait,
    KWGen,
    KWYield,
    KWImport,
    KWImportC,
    KWExtern,
    KWConst,
    KWStatic,
    KWType,
    KWVolatile,

    BoolLitTrue,
    BoolLitFalse,
    StringLiteral,
    CStringLiteral,
    RawStringLiteral,
    CharLiteral,
    IntLiteral,
    FloatLiteral,

    LineComment,
    BlockComment,
}

impl Display for ZoidTokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::EOF => write!(f, "EOF"),
            Self::RParen => write!(f, ")"),
            Self::LParen => write!(f, "("),
            Self::RBrace => write!(f, "}}"),
            Self::LBrace => write!(f, "{{"),
            Self::RBracket => write!(f, "]"),
            Self::LBracket => write!(f, "["),
            Self::LGenericBracket => write!(f, "<"),
            Self::RGenericBracket => write!(f, ">"),
            Self::Colon => write!(f, ":"),
            Self::Semicolon => write!(f, ";"),
            Self::Comma => write!(f, ","),
            Self::OpLeq => write!(f, "<="),
            Self::OpLt => write!(f, "<"),
            Self::OpGeq => write!(f, ">="),
            Self::OpGt => write!(f, ">"),
            Self::OpEq => write!(f, "=="),
            Self::OpAssign => write!(f, "="),
            Self::OpNe => write!(f, "!="),
            Self::OpNot => write!(f, "!"),
            Self::OpAnd => write!(f, "&&"),
            Self::OpBitAnd => write!(f, "&"),
            Self::OpOr => write!(f, "||"),
            Self::OpBitOr => write!(f, "|"),
            Self::OpBitNot => write!(f, "~"),
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpMul => write!(f, "*"),
            Self::OpDiv => write!(f, "/"),
            Self::OpRem => write!(f, "%"),
            Self::OpShl => write!(f, "<<"),
            Self::OpShr => write!(f, ">>"),
            Self::OpAddr => write!(f, "&"),
            Self::OpDeref => write!(f, "*"),
            Self::OpUnwrap => write!(f, "!"),
            Self::OpRangeExclusive => write!(f, "..<"),
            Self::VaArgs => write!(f, "..."),
            Self::OpRangeInclusive => write!(f, "..="),
            Self::OpDot => write!(f, "."),
            Self::Identifier => write!(f, "identifier"),
            Self::KWIf => write!(f, "if"),
            Self::KWElse => write!(f, "else"),
            Self::KWFn => write!(f, "fn"),
            Self::KWLet => write!(f, "let"),
            Self::KWReturn => write!(f, "return"),
            Self::KWFor => write!(f, "for"),
            Self::KWWhile => write!(f, "while"),
            Self::KWBreak => write!(f, "break"),
            Self::KWContinue => write!(f, "continue"),
            Self::KWIn => write!(f, "in"),
            Self::KWStruct => write!(f, "struct"),
            Self::KWEnum => write!(f, "enum"),
            Self::KWUnion => write!(f, "union"),
            Self::KWImpl => write!(f, "impl"),
            Self::KWTrait => write!(f, "trait"),
            Self::KWWhere => write!(f, "where"),
            Self::KWAsync => write!(f, "async"),
            Self::KWAwait => write!(f, "await"),
            Self::KWGen => write!(f, "gen"),
            Self::KWYield => write!(f, "yield"),
            Self::KWImport => write!(f, "import"),
            Self::KWImportC => write!(f, "importc"),
            Self::KWExtern => write!(f, "extern"),
            Self::KWConst => write!(f, "const"),
            Self::KWStatic => write!(f, "static"),
            Self::KWType => write!(f, "type"),
            Self::KWVolatile => write!(f, "volatile"),
            Self::BoolLitTrue => write!(f, "true"),
            Self::BoolLitFalse => write!(f, "false"),
            Self::StringLiteral => write!(f, "string literal"),
            Self::CStringLiteral => write!(f, "C string literal"),
            Self::RawStringLiteral => write!(f, "raw string literal"),
            Self::CharLiteral => write!(f, "character literal"),
            Self::IntLiteral => write!(f, "integer literal"),
            Self::FloatLiteral => write!(f, "float literal"),
            Self::LineComment => write!(f, "line comment"),
            Self::BlockComment => write!(f, "block comment"),
        }
    }
}
