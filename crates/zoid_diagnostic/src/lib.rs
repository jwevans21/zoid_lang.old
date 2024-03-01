use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    str::Lines,
};

use zoid_location::ZoidLocation;

pub mod error_codes;

pub use error_codes::ZoidErrorCode;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZoidDiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct ZoidDiagnostic<'fname, 'source, 'a> {
    location: ZoidLocation<'fname>,
    lines: Lines<'source>,
    severity: ZoidDiagnosticSeverity,
    code: ZoidErrorCode,
    message: &'a str,
}

impl<'fname, 'source, 'a> ZoidDiagnostic<'fname, 'source, 'a> {
    pub fn error(
        location: ZoidLocation<'fname>,
        lines: Lines<'source>,
        code: ZoidErrorCode,
        message: &'a str,
    ) -> Self {
        Self {
            location,
            lines,
            severity: ZoidDiagnosticSeverity::Error,
            code,
            message,
        }
    }
}

impl ZoidDiagnosticSeverity {
    pub fn color(&self) -> &'static str {
        match self {
            Self::Error => "\x1b[31m",
            Self::Info => "\x1b[34m",
            Self::Warning => "\x1b[33m",
        }
    }
}

impl Display for ZoidDiagnosticSeverity {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "\x1b[1m{}", self.color())?;
        match self {
            Self::Error => write!(f, "error"),
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
        }?;
        write!(f, "\x1b[0m")
    }
}

impl Display for ZoidDiagnostic<'_, '_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "{}\x1b[1m[E{:04}]: {}\x1b[0m", self.severity,self.code as u16, self.message)?;
        writeln!(
            f,
            " {:>3}{} {}:{}:{}",
            "",
            "\x1b[1;34m-->\x1b[0m",
            self.location.file_name,
            self.location.line,
            self.location.column
        )?;

        for (line, content) in self
            .lines
            .clone()
            .enumerate()
            .skip((self.location.line).saturating_sub(2))
            .take(3)
        {
            writeln!(f, " \x1b[1;34m{:>3} |\x1b[0m {}", line + 1, content)?;
            if line + 1 == self.location.line {
                writeln!(
                    f,
                    " \x1b[1;34m{:>3} |\x1b[0m \x1b[1m{}{}{}\x1b[0m",
                    " ",
                    self.severity.color(),
                    " ".repeat(self.location.column - 1),
                    "^".repeat(self.location.end - self.location.start),
                    // self.message,
                )?;
            }
        }

        Ok(())
    }
}
