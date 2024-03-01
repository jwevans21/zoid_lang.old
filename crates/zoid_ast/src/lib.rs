pub mod expression;
pub mod statement;
pub mod top_level;
pub mod ty;

// use std::fmt::Debug;
// use std::io::{Result as IOResult, Write};

// use bumpalo::Bump;
pub use expression::*;
// use hashbrown::{hash_map::DefaultHashBuilder, HashMap};
pub use statement::*;
pub use top_level::*;
pub use ty::*;
// use zoid_lexer::ZoidToken;
// use zoid_location::ZoidLocation;

// pub struct ZoidModule<'arena, S = DefaultHashBuilder> {
//     name: &'arena str,
//     file: &'arena str,
//     symbols: HashMap<&'arena str, &'arena Symbol, S, &'arena Bump>,
//     functions: HashMap<&'arena str, &'arena Function<'arena>, S, &'arena Bump>,
// }

// pub struct Symbol {
//     // TODO
// }

// pub struct Function<'arena> {
//     name: &'arena str,
// }

// pub trait AstNode<'arena>: Debug {
//     fn location(&self) -> ZoidLocation<'arena>;
//     fn pretty_print(&self, buffer: &mut dyn Write, indent: usize) -> IOResult<()>;
// }

// pub trait ASTStatement<'arena>: AstNode<'arena> {}

// pub trait ASTExpression<'arena>: AstNode<'arena> {}

// pub trait ASTType<'arena>: AstNode<'arena> {}

// #[derive(Debug)]
// pub struct ASTError<'arena> {
//     pub location: ZoidLocation<'arena>,
//     pub tokens: &'arena [ZoidToken<'arena>],
// }

// impl<'arena> AstNode<'arena> for ASTError<'arena> {
//     fn location(&self) -> ZoidLocation<'arena> {
//         self.location
//     }

//     fn pretty_print(&self, buffer: &mut dyn Write, _indent: usize) -> IOResult<()> {
//         write!(buffer, "ERROR(")?;
//         for token in self.tokens {
//             write!(buffer, "{:?},", token.kind)?;
//         }
//         write!(buffer, ")")
//     }
// }

// impl<'arena> ASTStatement<'arena> for ASTError<'arena> {}
// impl<'arena> ASTExpression<'arena> for ASTError<'arena> {}
// impl<'arena> ASTType<'arena> for ASTError<'arena> {}

// #[derive(Debug)]
// pub struct BinaryOperation<'arena> {
//     pub location: ZoidLocation<'arena>,
//     pub operator: u8,
//     pub lhs: &'arena dyn ASTExpression<'arena>,
//     pub rhs: &'arena dyn ASTExpression<'arena>,
// }

// impl<'arena> AstNode<'arena> for BinaryOperation<'arena> {
//     fn location(&self) -> ZoidLocation<'arena> {
//         self.location
//     }

//     fn pretty_print(&self, buffer: &mut dyn Write, indent: usize) -> IOResult<()> {
//         self.lhs.pretty_print(buffer, indent)?;
//         write!(buffer, " {} ", self.operator as char)?;
//         self.rhs.pretty_print(buffer, indent)
//     }
// }

// impl<'arena> ASTExpression<'arena> for BinaryOperation<'arena> {}
