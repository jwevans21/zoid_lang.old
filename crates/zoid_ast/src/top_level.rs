use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{expression::Expression, statement::Statement, ty::Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLevel<'ast> {
    Import(&'ast str),
    CImport(&'ast str),

    VariableDeclaration(&'ast (&'ast str, &'ast Expression<'ast>, Option<&'ast Type<'ast>>)),

    ExternFunction(&'ast (&'ast str, &'ast [&'ast Type<'ast>], &'ast Type<'ast>, bool)),
    Function(
        &'ast (
            &'ast str,
            &'ast [(&'ast str, &'ast Type<'ast>)],
            &'ast Type<'ast>,
            &'ast [Statement<'ast>],
        ),
    ),
}

impl<'ast> Display for TopLevel<'ast> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Import(path) => write!(f, "import \"{}\";", path),
            Self::CImport(path) => write!(f, "cimport \"{}\";", path),
            Self::VariableDeclaration((name, value, ty)) => {
                write!(f, "let {}", name)?;
                if let Some(ty) = ty {
                    write!(f, ": {}", ty)?;
                }
                write!(f, " = {};", value)
            }
            Self::ExternFunction((name, params, ret, variadic)) => {
                write!(f, "extern fn {}(", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                if *variadic {
                    write!(f, ", ...")?;
                }
                write!(f, "): {}", ret)
            }
            Self::Function((name, params, ret, body)) => {
                write!(f, "fn {}(", name)?;
                for (i, param) in params.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", param.0, param.1)?;
                }
                write!(f, "): {}", ret)?;
                if body.is_empty() {
                    write!(f, " {{}}")
                } else {
                    writeln!(f, " {{")?;
                    for stmt in body.iter() {
                        writeln!(f, "\t{}", stmt)?;
                    }
                    write!(f, "}}")
                }
            }
        }
    }
}
