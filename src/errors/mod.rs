pub mod interpreter_errors;
pub mod parser_errors;
pub mod scanner_errors;

pub trait ReportError {
    fn get_line(&self) -> usize;
    fn get_column(&self) -> usize;
    fn get_msg(&self) -> &str;
}
