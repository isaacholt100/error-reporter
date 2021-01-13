#[macro_export]
macro_rules! style {
    ($text: expr, $styles: expr) => {
        format!("{}{}m{}{}", BEGIN_STYLE, $styles.join(";"), $text, END_STYLE)
    }
}

#[macro_export]
macro_rules! format_line_start {
    ($number: expr, $padding: expr) => {
        format!("{}", style!(format!("{:<width$} | ", $number, width = $padding), [LINE_NUMBER_COLOR]))
    }
}

#[macro_export]
macro_rules! sgl_or_pl {
    ($number: expr, $text: expr) => {
        if $number == 1 {
            format!("{} {}", $number, $text)
        } else {
            format!("{} {}s", $number, $text)
        }
    }
}