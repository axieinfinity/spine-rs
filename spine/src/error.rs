use std::{error::Error as StdError, ffi::NulError, fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Render(Box<dyn StdError>),
    Other(Box<dyn StdError>),
}

impl Error {
    #[inline]
    pub fn invalid_input(error: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        io::Error::new(io::ErrorKind::InvalidInput, error.into()).into()
    }

    #[inline]
    pub fn invalid_data(error: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, error.into()).into()
    }

    #[inline]
    pub fn render(error: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Error::Render(error.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            Io(error) => error.fmt(f),
            Render(error) => error.fmt(f),
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

impl From<NulError> for Error {
    #[inline]
    fn from(error: NulError) -> Self {
        io::Error::from(error).into()
    }
}

#[derive(Debug)]
pub struct NullPointerError;

impl fmt::Display for NullPointerError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("null pointer encountered")
    }
}

impl StdError for NullPointerError {}
