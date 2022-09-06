use crate::constants::*;
use crate::error::{Error, Severity};
use crate::{format_line_start, style, sgl_or_pl};
use crate::slice::Slice;

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
    fn print_summary(warning_count: usize, error_count: usize) {
        println!(
            "{warning}, {error} emitted\n",
            warning = style!(sgl_or_pl!(warning_count, "warning"), [BOLD_STYLE, WARNING_COLOR]),
            error = style!(sgl_or_pl!(error_count, "error"), [BOLD_STYLE, ERROR_COLOR])
        );
    }
    pub fn report(&self, errors: Vec<Error>) {
        println!("");

        let warnings = errors.iter().filter(|x| {
            x.severity == Severity::Warning
        });
        let errors = errors.iter().filter(|x| {
            x.severity == Severity::Error
        });

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

        Self::print_summary(warning_count, error_count);
    }
    fn report_one(&self, error: &Error) {
        let (severity_name, severity_color) = error.severity.name_and_color();

        let start_col = (error.position.0).1;
        let end_col = (error.position.1).1 - 1; // So indexing works properly
        let start_line_number = (error.position.0).0;
        let end_line_number = (error.position.1).0;

        let padding = end_line_number.to_string().len();
        let start_line = self.lines[start_line_number - 1];
        let end_line = self.lines[end_line_number - 1];

        let title = format!(
            "{}: {}",
            style!(severity_name, [BOLD_STYLE, severity_color]),
            style!(error.msg, [BOLD_STYLE]),
        );
        let source_info = format!(
            "--> {location}{colon}{line}{colon}{col}",
            location = error.location,
            line = start_line_number,
            col = start_col,
            colon = style!(":", [DIM_STYLE]),
        );
        
        let start_col = start_col - 1; // So indexing works properly

        println!("{}\n{}\n", title, source_info);

        if start_line_number > 1 {
            println!("{}", format_line_start!(start_line_number - 1, padding));
        }

        print!(
            "{}{}",
            format_line_start!(start_line_number, padding),
            &start_line.slice(0, start_col)
        );

        if start_line_number == end_line_number {
            println!(
                "{}{}",
                style!(&start_line.slice(start_col, end_col), [severity_color, UNDERLINE_STYLE]),
                &start_line.slice_to_end(end_col)
            );
        } else {
            println!("{}", style!(&start_line.slice_to_end(start_col), [severity_color, UNDERLINE_STYLE]));

            for line_no in (start_line_number + 1)..end_line_number {
                println!(
                    "{}{}",
                    format_line_start!(line_no, padding),
                    style!(&self.lines[line_no - 1], [severity_color, UNDERLINE_STYLE])
                );
            }

            println!(
                "{}{}{}",
                format_line_start!(end_line_number, padding),
                style!(&end_line.slice(0, end_col), [severity_color, UNDERLINE_STYLE]),
                &end_line.slice_to_end(end_col)
            );
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