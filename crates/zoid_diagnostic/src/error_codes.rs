#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum ZoidErrorCode {
    UnknownToken,
    UnexpectedToken,
    UnexpectedEOF
}