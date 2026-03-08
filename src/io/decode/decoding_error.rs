use crate::io::WireType;
use std::fmt::{Display, Formatter};

/// A decoding error.
#[derive(Debug)]
pub enum DecodingError {
    /// There was an error reading the stream.
    Stream(std::io::Error),

    /// The wire type was invalid.
    InvalidWireType(WireType),

    /// The value was out of range.
    ValueOutOfRange,

    /// The length prefix is out of range.
    LengthPrefixOutOfRange,

    /// An error decoding a packet.
    InvalidPacket(enc::Error),

    /// The encoded boolean value was invalid. (must be 0 or 1)
    InvalidBool(u8),
}

impl DecodingError {
    //! Construction

    /// Creates a decoding error from the var-int decoding `error`.
    pub fn from_var_int_error(error: enc::Error) -> Self {
        match error {
            enc::Error::Stream(error) => Self::Stream(error),
            _ => Self::ValueOutOfRange,
        }
    }

    /// Creates a decoding error from the var-int length-prefix decoding `error`.
    pub fn from_length_prefix_error(error: enc::Error) -> Self {
        match error {
            enc::Error::Stream(error) => Self::Stream(error),
            _ => Self::LengthPrefixOutOfRange,
        }
    }

    /// Creates a decoding error from the packet decoding `error`.
    pub fn from_packet_error(error: enc::Error) -> Self {
        match error {
            enc::Error::Stream(error) => Self::Stream(error),
            _ => Self::InvalidPacket(error),
        }
    }
}

impl From<std::io::Error> for DecodingError {
    fn from(error: std::io::Error) -> Self {
        Self::Stream(error)
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
