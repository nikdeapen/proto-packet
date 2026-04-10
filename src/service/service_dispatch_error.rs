use crate::service::ServiceError;
use std::fmt::{Display, Formatter};

/// An error dispatching a service call.
#[derive(Debug)]
pub enum ServiceDispatchError {
    /// An error decoding the request.
    Decode(Box<dyn std::error::Error + Send + Sync>),

    /// The service call returned an error.
    Service(ServiceError),

    /// An error encoding the response.
    Encode(Box<dyn std::error::Error + Send + Sync>),
}

impl ServiceDispatchError {
    /// Creates a decode error.
    pub fn decode<E: std::error::Error + Send + Sync + 'static>(error: E) -> Self {
        Self::Decode(Box::new(error))
    }

    /// Creates an encode error.
    pub fn encode<E: std::error::Error + Send + Sync + 'static>(error: E) -> Self {
        Self::Encode(Box::new(error))
    }
}

impl From<ServiceError> for ServiceDispatchError {
    fn from(error: ServiceError) -> Self {
        Self::Service(error)
    }
}

impl Display for ServiceDispatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(error) => write!(f, "decode error: {}", error),
            Self::Service(error) => write!(f, "service error: {}", error),
            Self::Encode(error) => write!(f, "encode error: {}", error),
        }
    }
}

impl std::error::Error for ServiceDispatchError {}
