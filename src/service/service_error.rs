use std::fmt::{Display, Formatter};

/// A service error.
#[derive(Debug)]
pub struct ServiceError {
    cause: Box<dyn std::error::Error + Send + Sync>,
}

impl ServiceError {
    //! Construction

    /// Creates a new service error.
    pub fn new<E: std::error::Error + Send + Sync + 'static>(error: E) -> Self {
        Self {
            cause: Box::new(error),
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ServiceError {
    fn from(cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self { cause }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.cause, f)
    }
}

impl std::error::Error for ServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
