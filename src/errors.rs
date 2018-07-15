use oping::PingError;
use std::io::Error as IoError;
use fern::InitError;
use bincode::ErrorKind;
use std::net::AddrParseError;

#[derive(Debug)]
pub enum MDCError {
    PingError,
    IoError,
    LoggerError,
    ReplExit,
    BincodeError,
    AddrParseError
}

impl From<IoError> for MDCError {
    fn from(_: IoError) -> Self {
        MDCError::IoError
    }
}

impl From<PingError> for MDCError {
    fn from(_: PingError) -> Self {
        MDCError::PingError
    }
}

impl From<InitError> for MDCError {
    fn from(_: InitError) -> Self {
        MDCError::LoggerError
    }
}

impl From<ErrorKind> for MDCError {
    fn from(_: ErrorKind) -> Self {
        MDCError::BincodeError
    }
}

impl<T> From<Box<T>> for MDCError {
    fn from(a: Box<T>) -> Self {
        MDCError::from(a)
    }
}

impl From<AddrParseError> for MDCError {
    fn from(a: AddrParseError) -> Self {
        MDCError::AddrParseError
    }
}
