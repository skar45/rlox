pub mod parser_errors;
pub mod scanner_errors;

trait ReportError {
    fn line(&self) -> usize;
    fn column(&self) -> usize;
    fn line_text(&self) -> Option<String>;
}
