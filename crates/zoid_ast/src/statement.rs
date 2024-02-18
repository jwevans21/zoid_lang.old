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
    ForRange(
        &'ast (
            &'ast str,
            &'ast Type<'ast>,
            &'ast (isize, isize),
            &'ast [Statement<'ast>],
        ),
    ),
    ForIn(&'ast (&'ast str, &'ast Expression<'ast>, &'ast [Statement<'ast>])),
    Block(&'ast [&'ast Statement<'ast>]),
    Break,
    Continue,
}
