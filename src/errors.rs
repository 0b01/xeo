use oping::PingError;
use std::ops;

#[derive(Debug)]
pub enum MDCError {
    PingError,
}

pub enum MDCResult<T> {
    Ok(T),
    Err(MDCError),
}

impl From<PingError> for MDCError {
    fn from(_: PingError) -> Self {
        MDCError::PingError
    }
}

impl<T> ops::Try for MDCResult<T> {
    type Ok = T;
    type Error = MDCError;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            MDCResult::Ok(t) => Ok(t),
            MDCResult::Err(e) => Err(e),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        MDCResult::Err(v)
    }

    fn from_ok(v: Self::Ok) -> Self {
        MDCResult::Ok(v)
    }
}
