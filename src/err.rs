use thiserror::Error;

use crate::seq::Span;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Invalid Character at {0:?}")]
    InvalidChar(Span)
}

#[derive(Error, Debug)]
pub enum RuntimeError {}
