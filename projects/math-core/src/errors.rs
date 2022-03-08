#[derive(Debug, Copy, Clone)]
pub enum SMError {
    UnknownError,
}

pub type Result<T> = std::result::Result<T, SMError>;
