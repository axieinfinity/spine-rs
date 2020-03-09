use std::ffi::NulError;
use std::str::Utf8Error;
use std::{error::Error as StdError, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Utf8(Utf8Error),
    NullPointer,
    Other(Box<dyn StdError>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Io(error) => error.fmt(f),
            Utf8(error) => error.fmt(f),
            NullPointer => f.write_str("null pointer encountered"),
            Other(error) => error.fmt(f),
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

impl From<Box<dyn StdError>> for Error {
    #[inline]
    fn from(error: Box<dyn StdError>) -> Self {
        Error::Other(error)
    }
}

impl From<&str> for Error {
    #[inline]
    fn from(message: &str) -> Self {
        Error::Other(message.into())
    }
}

impl From<NulError> for Error {
    #[inline]
    fn from(error: NulError) -> Self {
        io::Error::from(error).into()
    }
}
