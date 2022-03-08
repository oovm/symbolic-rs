use crate::Span;
use std::sync::{MutexGuard, PoisonError};

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

impl<T> From<PoisonError<MutexGuard<'_, T>>> for SMError {
    fn from(_: PoisonError<MutexGuard<'_, T>>) -> Self {
        SMError::UnknownError
    }
}
