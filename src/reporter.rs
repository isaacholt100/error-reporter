use crate::constants::*;
use crate::error::{Error, Severity};


macro_rules! style {
    ($text: expr, $styles: expr) => {
        format!("{}{}m{}{}", BEGIN_STYLE, $styles.join(";"), $text, END_STYLE)
    }
}

macro_rules! format_line_start {
    ($number: expr, $padding: expr) => {
        format!("{}", style!(format!("{:<width$} | ", $number, width = $padding), [LINE_NUMBER_COLOR]))
    }
}

pub struct ErrorReporter<'a> {
    _src: &'a str,
    lines: Vec<&'a str>,
    //location: Location,
}
impl<'a> ErrorReporter<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            _src: src,
            lines: src.lines().collect(),
        }
    }
    pub fn report(&self, errors: Vec<Error>) {
        println!("");

        let warnings = errors.iter().filter(|x| x.severity == Severity::Warning);
        let errors = errors.iter().filter(|x| x.severity == Severity::Error);
        let mut warning_count = 0;
        let mut error_count = 0;
        for warning in warnings {
            self.report_one(warning);
            warning_count += 1;
        }
        for error in errors {
            self.report_one(error);
            error_count += 1;
        }

        println!("{} {}{}, {} {}{} emitted\n", warning_count, style!("warning", [BOLD_STYLE, WARNING_COLOR]), if warning_count == 1 { "" } else { "s" }, error_count, style!("error", [BOLD_STYLE, ERROR_COLOR]), if error_count == 1 { "" } else { "s" });
    }
    fn report_one(&self, error: &Error) {
        let severity_color = match error.severity {
            Severity::Error => ERROR_COLOR,
            Severity::Warning => WARNING_COLOR,
        };
        let severity = match error.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };

        let start_col = (error.position.0).1;
        let end_col = (error.position.1).1;
        let start_line_number = (error.position.0).0;
        let end_line_number = (error.position.1).0;

        let title = style!(format!("{}: {}", severity, error.msg), [BOLD_STYLE, severity_color]);
        let source_info = format!("--> {}:{}:{}", error.location, start_line_number, start_col);

        println!("{}\n{}\n", title, source_info);

        let padding = end_line_number.to_string().len();
        let start_line = self.lines[start_line_number - 1];
        let end_line = self.lines[end_line_number - 1];

        if start_line_number > 1 {
            println!("{}", format_line_start!(start_line_number - 1, padding));
        }

        print!("{}{}", format_line_start!(start_line_number, padding), &start_line[0..start_col]);

        if start_line_number == end_line_number {
            println!("{}{}", style!(&start_line[start_col..end_col], [severity_color, UNDERLINE_STYLE]), &start_line[end_col..]);
        } else {
            println!("{}", style!(&start_line[start_col..], [severity_color, UNDERLINE_STYLE]));

            for line_no in (start_line_number + 1)..end_line_number {
                println!("{}{}", format_line_start!(line_no, padding), style!(&self.lines[line_no - 1], [severity_color, UNDERLINE_STYLE]));
            }

            println!("{}{}{}", format_line_start!(end_line_number, padding), style!(&end_line[0..end_col], [severity_color, UNDERLINE_STYLE]), &end_line[end_col..]);
        }

        if end_line_number < self.lines.len() {
            println!("{}", format_line_start!(end_line_number + 1, padding));
        }

        if let Some(help) = &error.help {
            println!("{}\n", style!(format!("\nhelp: {}", help), [HELP_COLOR]));
        } else {
            println!("");
        }
    }
}