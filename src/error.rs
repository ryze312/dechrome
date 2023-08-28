use std::io;
use ureq;

// TODO: Well...
pub type DechromeResult<T> = Result<T, DechromeError>;

#[derive(Debug)]
pub enum DechromeError {
    FileNotFound,
    ExecutablePathNotFound,
    MismatchedQuotes,
    ContentLengthError,
    IOError(io::Error),
    NetworkError(Box<ureq::Error>),
}

impl From<io::Error> for DechromeError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<ureq::Error> for DechromeError {
    fn from(err: ureq::Error) -> Self {
        match err {
            ureq::Error::Status(code, _) if code == 404 => Self::FileNotFound,
            _ => Self::NetworkError(Box::new(err)),
        }
    }
}
