use std::fmt::{Display, Formatter, Result as FmtResult};

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

    UnaryPrefix(&'ast (PrefixOperator, &'ast Expression<'ast>)),
    UnaryPostfix(&'ast (PostfixOperator, &'ast Expression<'ast>)),
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
pub enum PrefixOperator {
    Negate,
    Not,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostfixOperator {
    Deref,
    AddressOf,
    Unwrap,
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

impl<'ast> Display for Expression<'ast> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match  self {
            Self::Variable(name) => write!(f, "{}", name),
            Self::LiteralString(s) => write!(f, "\"{}\"", s),
            Self::LiteralCString(s) => write!(f, "c\"{}\"", s),
            Self::LiteralChar(c) => write!(f, "'{}'", c.iter().collect::<String>()),
            Self::LiteralInteger(i) => write!(f, "{}", i),
            Self::LiteralFloat(l) => write!(f, "{}", l),
            Self::LiteralBool(b) => write!(f, "{}", b),

            Self::UnaryPrefix((op, expr)) => write!(f, "{}{}", op, expr),
            Self::UnaryPostfix((op, expr)) => write!(f, "{}.{}", expr, op),
            Self::Binary((op, lhs, rhs)) => write!(f, "({} {} {})", lhs, op, rhs),

            Self::Call((callee, args)) => {
                write!(f, "{}({})", callee, args.iter().map(|arg| format!("{}", arg)).collect::<Vec<_>>().join(", "))
            }
            Self::Index((lhs, rhs)) => write!(f, "{}[{}]", lhs, rhs),

            Self::Cast((ty, expr)) => write!(f, "({} :{})", expr, ty),
        }
    }
}

impl<'ast> Display for PrefixOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Negate => write!(f, "-"),
            Self::Not => write!(f, "!"),
            Self::BitNot => write!(f, "~"),
        }
    }
}

impl<'ast> Display for PostfixOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Deref => write!(f, "*"),
            Self::AddressOf => write!(f, "&"),
            Self::Unwrap => write!(f, "?"),
        }
    }
}

impl<'ast> Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Rem => write!(f, "%"),
            Self::BitAnd => write!(f, "&"),
            Self::BitOr => write!(f, "|"),
            Self::BitXor => write!(f, "^"),
            Self::Shl => write!(f, "<<"),
            Self::Shr => write!(f, ">>"),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Le => write!(f, "<="),
            Self::Gt => write!(f, ">"),
            Self::Ge => write!(f, ">="),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
        }
    }
}
