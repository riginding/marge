#[macro_export]
macro_rules! marge_error {
    ($expression:expr) => {{
        let bold = Style::new().bold().red();
        let error_style = Style::new().red();
        println!(
            "{} {}",
            bold.apply_to("Error:"),
            error_style.apply_to($expression)
        );
    }};
}
