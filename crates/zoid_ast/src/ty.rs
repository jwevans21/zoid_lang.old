use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type<'ast> {
    Pointer(&'ast Type<'ast>),
    Function(&'ast (&'ast [&'ast Type<'ast>], &'ast Type<'ast>)),
    Const(&'ast Type<'ast>),
    Volatile(&'ast Type<'ast>),

    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    F16,
    F32,
    F64,
    F128,
    Bool,
    Char,
    Void,
}

impl<'ast> Display for Type<'ast> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Pointer(inner) => write!(f, "*{}", inner),
            Self::Function((params, ret)) => write!(
                f,
                "fn({}): {}",
                params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                ret
            ),
            Self::Const(inner) => write!(f, "const {}", inner),
            Self::Volatile(inner) => write!(f, "volatile {}", inner),
            Self::U8 => write!(f, "u8"),
            Self::U16 => write!(f, "u16"),
            Self::U32 => write!(f, "u32"),
            Self::U64 => write!(f, "u64"),
            Self::U128 => write!(f, "u128"),
            Self::Usize => write!(f, "usize"),
            Self::I8 => write!(f, "i8"),
            Self::I16 => write!(f, "i16"),
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::I128 => write!(f, "i128"),
            Self::Isize => write!(f, "isize"),
            Self::F16 => write!(f, "f16"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::F128 => write!(f, "f128"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::Void => write!(f, "void"),
        }
    }
}
