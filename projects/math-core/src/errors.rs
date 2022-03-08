use crate::Span;

#[derive(Debug, Copy, Clone)]
pub enum SMError {
    UnknownError,
}

pub type Result<T> = std::result::Result<T, SMError>;

impl SMError {
    pub fn new(_span: Span, _: String) -> SMError {
        SMError::UnknownError
    }
}
