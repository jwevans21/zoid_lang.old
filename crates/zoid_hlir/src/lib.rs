use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HLIRProgram<'source> {
    pub globals: HashMap<&'source str, HLIRType>,
    pub prototypes: HashMap<&'source str, (Vec<HLIRType>, HLIRType)>,
    pub functions: Vec<HLIRFunction<'source>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HLIRFunction<'source> {
    pub name: &'source str,
    pub parameters: Vec<HLIRParameter<'source>>,
    pub return_type: HLIRType,
    pub body: Vec<HLIRStatement<'source>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HLIRParameter<'source> {
    pub name: &'source str,
    pub ty: HLIRType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HLIRStatement<'source> {
    VariableDeclaration {
        name: &'source str,
        ty: HLIRType,
        value: HLIRExpression<'source>,
    },
    Return(Option<HLIRExpression<'source>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HLIRExpression<'source> {
    Literal(HLIRLiteral<'source>, HLIRType),
    Variable(&'source str, HLIRType),
    BinaryOperation {
        lhs: Box<HLIRExpression<'source>>,
        op: HLIRBinaryOperator,
        rhs: Box<HLIRExpression<'source>>,
        ty: HLIRType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HLIRLiteral<'source> {
    Integer(&'source str, HLIRType),
    Float(&'source str, HLIRType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HLIRBinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[rustfmt::skip]
pub enum HLIRType {
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Void,

    /// A type variable, used for type inference. If it appears after inference, it is an error.
    Var(usize),
}

impl HLIRExpression<'_> {
    pub fn ty(&self) -> HLIRType {
        match self {
            HLIRExpression::Literal(_, ty) => *ty,
            HLIRExpression::Variable(_, ty) => *ty,
            HLIRExpression::BinaryOperation { ty, .. } => *ty,
        }
    }
}
