use zoid_location::ZoidLocation;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ZoidToken<'fname> {
    pub location: ZoidLocation<'fname>,
    pub kind: ZoidTokenKind,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ZoidTokenKind {
    RParen,
    LParen,
    RBrace,
    LBrace,
    RBracket,
    LBracket,
    LGenericBracket,
    RGenericBracket,
}