use crate::service::ServiceError;
use std::fmt::{Display, Formatter};

/// An error dispatching a service call.
#[derive(Debug)]
pub enum ServiceDispatchError {
    /// An error decoding the request.
    Decode(enc::Error),

    /// The service call returned an error.
    Service(ServiceError),

    /// An error encoding the response.
    Encode(enc::Error),
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

impl std::error::Error for ServiceDispatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Decode(error) => Some(error),
            Self::Service(error) => Some(error),
            Self::Encode(error) => Some(error),
        }
    }
}
