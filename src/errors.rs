use oping::PingError;
use std::io::Error as IoError;
use fern::InitError;
use bincode::ErrorKind;
use std::net::AddrParseError;

#[derive(Debug)]
pub enum XEOError {
    PingError,
    IoError,
    LoggerError,
    ReplExit,
    BincodeError,
    AddrParseError
}

impl From<IoError> for XEOError {
    fn from(_: IoError) -> Self {
        XEOError::IoError
    }
}

impl From<PingError> for XEOError {
    fn from(_: PingError) -> Self {
        XEOError::PingError
    }
}

impl From<InitError> for XEOError {
    fn from(_: InitError) -> Self {
        XEOError::LoggerError
    }
}

impl From<ErrorKind> for XEOError {
    fn from(_: ErrorKind) -> Self {
        XEOError::BincodeError
    }
}

impl<T> From<Box<T>> for XEOError {
    fn from(a: Box<T>) -> Self {
        XEOError::from(a)
    }
}

impl From<AddrParseError> for XEOError {
    fn from(a: AddrParseError) -> Self {
        XEOError::AddrParseError
    }
}
