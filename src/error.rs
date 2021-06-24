use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct MetalError {
    message: String
}

impl MetalError {
    pub fn new(message: &str) -> Self {
        MetalError {
            message: message.to_owned()
        }
    }
}

impl fmt::Display for MetalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type MetalResult<T> = Result<T, MetalError>;
