use nom::{error::VerboseError, Err};

pub struct Error {
    message: String,
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {} }}: {}",
            file!(),
            line!(),
            self.message
        )
    }
}

impl<T> From<VerboseError<T>> for Error
where
    T: std::fmt::Debug,
{
    fn from(error: VerboseError<T>) -> Self {
        Error {
            message: format!("{:?}", error),
        }
    }
}

impl<T> From<Err<T>> for Error
where
    T: std::fmt::Debug,
{
    fn from(error: Err<T>) -> Self {
        Error {
            message: format!("{:?}", error),
        }
    }
}
