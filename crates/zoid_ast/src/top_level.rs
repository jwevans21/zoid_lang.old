use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{expression::Expression, statement::Statement, ty::Type};

pub type FunctionParams<'ast> = (&'ast [(&'ast str, &'ast Type<'ast>)], bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLevel<'ast> {
    Import(&'ast str),
    ImportC(&'ast str),

    VariableDeclaration(&'ast (&'ast str, &'ast Expression<'ast>, Option<&'ast Type<'ast>>)),

    ExternFunction(&'ast (&'ast str, &'ast [&'ast Type<'ast>], &'ast Type<'ast>, bool)),
    Function(
        &'ast (
            &'ast str,
            &'ast [(&'ast str, &'ast Type<'ast>)],
            &'ast Type<'ast>,
            &'ast [&'ast Statement<'ast>],
        ),
    ),
}

impl<'ast> TopLevel<'ast> {
    pub fn pretty_print(&self, indent: usize) -> String {
        match self {
            Self::Import(path) => format!("{}import \"{}\";", " ".repeat(indent), path),
            Self::ImportC(path) => format!("{}importc \"{}\";", " ".repeat(indent), path),
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
                    value
                )
            }
            Self::ExternFunction((name, params, ret, variadic)) => {
                format!(
                    "{}extern fn {}({}): {};",
                    " ".repeat(indent),
                    name,
                    {
                        let mut p = params
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        if *variadic {
                            p.push_str(", ...");
                        }
                        p
                    },
                    ret
                )
            }
            Self::Function((name, params, ret, body)) => {
                format!(
                    "{}fn {}({}): {} {{\n{}\n{}}}",
                    " ".repeat(indent),
                    name,
                    params
                        .iter()
                        .map(|(name, ty)| format!("{}: {}", name, ty))
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret,
                    body.iter()
                        .map(|stmt| stmt.pretty_print(indent + 4))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    " ".repeat(indent)
                )
            }
        }
    }
}

impl Display for TopLevel<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Import(path) => write!(f, "import \"{}\";", path),
            Self::ImportC(path) => write!(f, "cimport \"{}\";", path),
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
                write!(f, "): {};", ret)
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
