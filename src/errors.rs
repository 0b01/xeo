use oping::PingError;

#[derive(Debug)]
pub struct MDCError {}

impl From<PingError> for MDCError {
    fn from(_: PingError) -> Self {
        MDCError {}
    }
}
