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
}
