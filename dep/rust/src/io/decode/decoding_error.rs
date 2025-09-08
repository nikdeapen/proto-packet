use crate::io::WireType;
use enc::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::string::FromUtf8Error;

/// A decoding error.
#[derive(Debug)]
pub enum DecodingError {
    /// There was an error reading the source.
    Source(io::Error),

    /// An invalid list header error.
    InvalidListHeader(Error),

    /// The list wire type was invalid.
    InvalidListWire(WireType),

    /// The value was out of range.
    ValueOutOfRange,

    /// The length-prefix was out of range.
    LengthPrefixOutOfRange,

    /// The wire type was invalid.
    InvalidWireType(WireType),

    /// The encoded boolean value was invalid.
    InvalidEncodedBoolean(u8),

    /// The string value was not valid UTF-8.
    InvalidString(FromUtf8Error),

    /// An error decoding a packet.
    PacketDecoding(Error),
}

impl DecodingError {
    //! Construction

    /// Creates a decoding error from the var-int stream `error`.
    pub fn from_var_int_error(error: Error) -> Self {
        match error {
            Error::Source(error) => Self::Source(error),
            _ => Self::ValueOutOfRange,
        }
    }

    /// Creates a decoding error from the length-prefix var-int stream `error`.
    pub fn error_reading_length_prefix(error: Error) -> Self {
        match error {
            Error::Source(error) => Self::Source(error),
            _ => Self::LengthPrefixOutOfRange,
        }
    }

    pub fn from_list_header(error: Error) -> Self {
        match error {
            Error::Source(error) => Self::Source(error),
            error => Self::InvalidListHeader(error),
        }
    }
}

impl From<DecodingError> for Error {
    fn from(error: DecodingError) -> Self {
        match error {
            DecodingError::Source(error) => error.into(),
            _ => Error::InvalidEncodedData {
                reason: Some(Box::new(error)),
            },
        }
    }
}

impl From<io::Error> for DecodingError {
    fn from(error: io::Error) -> Self {
        Self::Source(error)
    }
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DecodingError {}
