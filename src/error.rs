#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub line: usize,
}
