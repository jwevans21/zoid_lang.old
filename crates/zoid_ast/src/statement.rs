use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{expression::Expression, ty::Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Statement<'ast> {
    VariableDeclaration(&'ast (&'ast str, &'ast Expression<'ast>, Option<&'ast Type<'ast>>)),
    Expression(&'ast Expression<'ast>),
    Return(Option<&'ast Expression<'ast>>),
    If(
        &'ast (
            &'ast Expression<'ast>,
            &'ast [Statement<'ast>],
            Option<&'ast [Statement<'ast>]>,
        ),
    ),
    While(&'ast (&'ast Expression<'ast>, &'ast [Statement<'ast>])),
    Block(&'ast [&'ast Statement<'ast>]),
    Break,
    Continue,
}

impl<'ast> Display for Statement<'ast> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::VariableDeclaration((name, value, ty)) => {
                write!(f, "let {}", name)?;
                if let Some(ty) = ty {
                    write!(f, ": {}", ty)?;
                }
                write!(f, " = {};", value)
            }
            Self::Expression(expr) => write!(f, "{};", expr),
            Self::Return(None) => write!(f, "return;"),
            Self::Return(Some(expr)) => write!(f, "return {};", expr),
            Self::If((cond, then, els)) => {
                writeln!(f, "if {} {{", cond)?;
                for stmt in then.iter() {
                    writeln!(f, "\t{}", stmt)?;
                }
                if let Some(els) = els {
                    todo!("else branch")
                } else {
                    writeln!(f, "}}")?;
                }
                Ok(())
            }
            Self::While((cond, body)) => {
                writeln!(f, "while {} {{", cond)?;
                for stmt in body.iter() {
                    writeln!(f, "\t{}", stmt)?;
                }
                writeln!(f, "}}")
            }
            Self::Block(stmts) => {
                writeln!(f, "{{")?;
                for stmt in stmts.iter() {
                    writeln!(f, "\t{}", stmt)?;
                }
                writeln!(f, "}}")
            }
            Self::Break => write!(f, "break;"),
            Self::Continue => write!(f, "continue;"),
        }
    }
}
