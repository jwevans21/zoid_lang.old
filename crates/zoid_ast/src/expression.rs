use crate::ty::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Expression<'ast> {
    Variable(&'ast str),
    LiteralString(&'ast str),
    LiteralCString(&'ast str),
    LiteralChar(&'ast [char]),
    LiteralInteger(&'ast str),
    LiteralFloat(&'ast str),
    LiteralBool(bool),
    LiteralNull,

    Unary(&'ast (UnaryOperator, &'ast Expression<'ast>)),
    Binary(
        &'ast (
            BinaryOperator,
            &'ast Expression<'ast>,
            &'ast Expression<'ast>,
        ),
    ),

    Call(&'ast (&'ast Expression<'ast>, &'ast [&'ast Expression<'ast>])),
    Index(&'ast (&'ast Expression<'ast>, &'ast Expression<'ast>)),

    Cast(&'ast (&'ast Type<'ast>, &'ast Expression<'ast>)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negate,
    Not,
    BitNot,
    Deref,
    AddressOf,
    Unwrap,
    SizeOf,
    AlignOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}
