use std::fmt::{self, Display, Formatter};
use crate::constants::{ERROR_COLOR, WARNING_COLOR};

type Position = (usize, usize);

#[derive(PartialEq)]
pub enum Severity {
    Error,
    Warning,
}
impl Severity {
    pub fn name_and_color(&self) -> (&str, &str) {
        match self {
            Severity::Error => ("error", ERROR_COLOR),
            Severity::Warning => ("warning", WARNING_COLOR),
        }
    }
}

pub struct Error {
    pub position: (Position, Position),
    pub severity: Severity,
    pub msg: String,
    pub help: Option<String>,
    pub location: Location,
}

pub enum Location {
    _FileName(String),
    Repl,
}
impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let display_name = match self {
            Location::_FileName(file_name) => &file_name,
            Location::Repl => "[REPL]",
        };
        write!(f, "{}", display_name)
    }
}