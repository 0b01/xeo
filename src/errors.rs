use std::ops;
use oping::PingError;
use std::io::Error as IoError;
use fern::InitError;

#[derive(Debug)]
pub enum MDCError {
    PingError,
    IoError,
    LoggerError,
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
