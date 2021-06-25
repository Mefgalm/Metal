use crate::error::*;
use std::io;

impl From<reqwest::Error> for MetalError {
    fn from(_error: reqwest::Error) -> Self {
        MetalError::new("Request failed")
    }
}

impl From<serde_json::Error> for MetalError {
    fn from(error: serde_json::Error) -> Self {
        MetalError::new(&error.to_string())
    }
}

impl From<regex::Error> for MetalError {
    fn from(error: regex::Error) -> Self {
        MetalError::new(&error.to_string())
    }
}

impl From<io::Error> for MetalError {
    fn from(error: io::Error) -> Self {
        MetalError::new(&error.to_string())
    }
}

pub fn combine_errors<T: Clone, E: Clone>(errors: &Vec<Result<T, E>>) -> Result<Vec<T>, Vec<E>> {
    let mut oks = vec![];
    let mut errs = vec![];
    for error in errors {
        match error {
            Ok(value) => oks.push(value.clone()),
            Err(err) => errs.push(err.clone())
        }
    }
    if errs.is_empty() {
        Ok(oks)
    } else {
        Err(errs)
    }
}
