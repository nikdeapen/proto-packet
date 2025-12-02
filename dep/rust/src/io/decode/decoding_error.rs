use crate::io::DecodingError::*;
use crate::io::WireType;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// A decoding error.
#[derive(Debug)]
pub enum DecodingError {
    /// There was an error reading the stream.
    Stream(std::io::Error),

    /// The wire type was invalid for the semantic type.
    InvalidWireType(WireType),

    /// The decoded value was out of range for the semantic type.
    ValueOutOfRange,

    /// The decoded length-prefix was out of range of `usize`.
    LengthPrefixOutOfRange,

    /// An error decoding a packet.
    PacketDecoding(enc::Error),

    /// The string was not valid UTF-8.
    InvalidString(FromUtf8Error),
}

impl DecodingError {
    //! Construction

    /// Creates a decoding error from the `error` reading a var-int value.
    pub fn from_var_int_error(error: enc::Error) -> Self {
        match error {
            enc::Error::Stream(error) => Stream(error),
            _ => ValueOutOfRange,
        }
    }

    /// Creates a decoding error from the `error` reading a var-int length-prefix.
    pub fn from_length_prefix_error(error: enc::Error) -> Self {
        match error {
            enc::Error::Stream(error) => Stream(error),
            _ => LengthPrefixOutOfRange,
        }
    }
}

impl From<std::io::Error> for DecodingError {
    fn from(error: std::io::Error) -> Self {
        Stream(error)
    }
}

impl From<DecodingError> for enc::Error {
    fn from(error: DecodingError) -> Self {
        match error {
            DecodingError::Stream(error) => error.into(),
            _ => enc::Error::InvalidEncodedData {
                reason: Some(Box::new(error)),
            },
        }
    }
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DecodingError {}
