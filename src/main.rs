mod constants;
mod error;
mod reporter;
mod macros;
mod slice;

use reporter::ErrorReporter;
use error::{Error, Severity, Location};

fn main() {
    let reporter = ErrorReporter::new("Hello world!\nHello errors!\nMultiline yayyy!\nAnother line...\nAnd yet more!");
    let test_error = Error {
        position: ((2, 3), (4, 5)),
        severity: Severity::Error,
        msg: String::from("This is a test error"),
        help: Some(String::from("This is a help message")),
        location: Location::Repl,
    };
    let test_error_2 = Error {
        position: ((1, 1), (1, 8)),
        severity: Severity::Error,
        msg: String::from("Another error"),
        help: Some(String::from("This error is right at the start")),
        location: Location::Repl,
    };
    let test_warning = Error {
        position: ((3, 3), (3, 6)),
        severity: Severity::Warning,
        msg: String::from("This is a test warning"),
        location: Location::Repl,
        help: None,
    };
    let test_warning_2 = Error {
        position: ((5, 1), (5, 6)),
        severity: Severity::Warning,
        msg: String::from("Another warning"),
        location: Location::Repl,
        help: Some(String::from("This warning is right at the end")),
    };
    reporter.report(vec![test_error, test_warning, test_warning_2, test_error_2]);
}