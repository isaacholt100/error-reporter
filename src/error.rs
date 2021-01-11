use std::fmt::{self, Display, Formatter};

type Position = (usize, usize);

#[derive(PartialEq)]
pub enum Severity {
    Error,
    Warning,
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