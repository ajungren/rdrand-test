use std::error::Error as StdError;
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug)]
pub enum Error {
    InsufficientIterations { required: usize },
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            InsufficientIterations { required } => write!(
                f,
                "at least {} {} required",
                required,
                if *required == 1 {
                    "iteration is"
                } else {
                    "iterations are"
                }
            ),
        }
    }
}

impl StdError for Error {}
