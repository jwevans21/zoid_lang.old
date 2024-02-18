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
