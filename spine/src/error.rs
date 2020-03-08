use std::ffi::NulError;
use std::result::Result as StdResult;
use std::str::Utf8Error;
use std::{error::Error as StdError, fmt, io};

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(Utf8Error),
    NullPointer,
    Other(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Io(ref error) => error.fmt(f),
            Utf8(ref error) => error.fmt(f),
            NullPointer => f.write_str("null pointer encountered"),
            Other(message) => message.fmt(f),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    #[inline]
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<Utf8Error> for Error {
    #[inline]
    fn from(error: Utf8Error) -> Self {
        Error::Utf8(error)
    }
}

impl From<&'static str> for Error {
    #[inline]
    fn from(message: &'static str) -> Self {
        Error::Other(message)
    }
}

impl From<NulError> for Error {
    #[inline]
    fn from(error: NulError) -> Self {
        io::Error::from(error).into()
    }
}
