#[derive(Debug, Clone, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Type {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Void,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program<'source>(pub Vec<TopLevelExpression<'source>>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelExpression<'source> {
    Function {
        name: &'source str,
        parameters: Vec<Parameter<'source>>,
        return_type: Option<Type>,
        body: Vec<Statement<'source>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter<'source> {
    pub name: &'source str,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'source> {
    VariableDeclaration {
        name: &'source str,
        ty: Option<Type>,
        value: Expression<'source>,
    },
    Return(Option<Expression<'source>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'source> {
    Literal(Literal<'source>),
    Variable(&'source str),
    BinaryOperation {
        lhs: Box<Expression<'source>>,
        op: BinaryOperator,
        rhs: Box<Expression<'source>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Literal<'source> {
    Integer(&'source str),
    Float(&'source str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}
