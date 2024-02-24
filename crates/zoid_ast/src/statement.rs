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
            &'ast Statement<'ast>,
            Option<&'ast Statement<'ast>>,
        ),
    ),
    While(&'ast (&'ast Expression<'ast>, &'ast Statement<'ast>)),
    Block(&'ast [&'ast Statement<'ast>]),
    Break,
    Continue,
}

impl Statement<'_> {
    fn pretty_print_if(if_stmt: &Self, indent: usize) -> String {
        assert!(matches!(if_stmt, Self::If(_)));

        let (cond, then, els) = match if_stmt {
            Self::If((cond, then, els)) => (cond, then, els),
            _ => unreachable!(),
        };

        format!(
            "if {} {}{}\n",
            cond.pretty_print(),
            then.pretty_print(indent),
            match els {
                Some(els) => match els {
                    Self::If(_) => format!(" else {}", Self::pretty_print_if(els, indent)),
                    Self::Block(_) => format!(" else {}", els.pretty_print(indent)),
                    _ => format!(" else\n{}", els.pretty_print(indent)),
                },
                None => String::new(),
            }
        )
    }
    pub fn pretty_print(&self, indent: usize) -> String {
        match self {
            Self::VariableDeclaration((name, value, ty)) => {
                format!(
                    "{}let {}{} = {};",
                    " ".repeat(indent),
                    name,
                    if let Some(ty) = ty {
                        format!(": {}", ty)
                    } else {
                        String::new()
                    },
                    value.pretty_print()
                )
            }
            Self::Expression(expr) => format!("{}{};", " ".repeat(indent), expr.pretty_print()),
            Self::Return(None) => format!("{}return;", " ".repeat(indent)),
            Self::Return(Some(expr)) => {
                format!("{}return {};", " ".repeat(indent), expr.pretty_print())
            }
            Self::If(_) => format!(
                "{}{}",
                " ".repeat(indent),
                Self::pretty_print_if(self, indent)
            ),
            Self::While((cond, body)) => {
                format!(
                    "{}while {} {}\n",
                    " ".repeat(indent),
                    cond,
                    match body {
                        Self::Block(_) => body.pretty_print(indent),
                        _ => format!("\n{}", body.pretty_print(indent + 4)),
                    }
                )
            }
            Self::Block(stmts) => {
                format!(
                    "{{\n{}\n{}}}",
                    stmts
                        .iter()
                        .map(|stmt| stmt.pretty_print(indent + 4))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    " ".repeat(indent)
                )
            }
            Self::Break => format!("{}break;", " ".repeat(indent)),
            Self::Continue => format!("{}continue;", " ".repeat(indent)),
        }
    }
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
                write!(f, "if {} ", cond)?;
                match then {
                    Self::Block(_) => {}
                    _ => writeln!(f, "")?,
                }
                writeln!(f, "{}", then)?;

                if let Some(els) = els {
                    writeln!(f, "}} else {}", els)?;
                } else {
                    writeln!(f, "}}")?;
                }
                Ok(())
            }
            Self::While((cond, body)) => {
                write!(f, "while {}", cond)?;
                match body {
                    Self::Block(_) => {
                        writeln!(f, " {}", body)
                    }
                    _ => {
                        writeln!(f, " ")?;
                        writeln!(f, "\t{}", body)
                    }
                }
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
