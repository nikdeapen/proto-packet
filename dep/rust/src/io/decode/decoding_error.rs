use crate::io::WireType;
use enc::StreamError;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// A decoding error.
#[derive(Debug)]
pub enum DecodingError {
    /// There was an error reading the source.
    Source(std::io::Error),

    /// The value was out of range.
    ValueOutOfRange,

    /// The length-prefix was out of range.
    LengthPrefixOutOfRange,

    /// The wire type was invalid.
    InvalidWireType(WireType),

    /// The encoded boolean value was invalid.
    InvalidEncodedBool(u8),

    /// The string value was not valid UTF-8.
    InvalidString(FromUtf8Error),
}

impl DecodingError {
    //! Construction

    /// Creates a decoding error from the var-int stream `error`.
    pub fn from_var_int_error(error: StreamError) -> Self {
        match error {
            StreamError::Encoding(_) => Self::ValueOutOfRange,
            StreamError::Source(error) => Self::Source(error),
        }
    }

    /// Creates a decoding error from the length-prefix var-int stream `error`.
    pub fn from_length_prefix_error(error: StreamError) -> Self {
        match error {
            StreamError::Encoding(_) => Self::LengthPrefixOutOfRange,
            StreamError::Source(error) => Self::Source(error),
        }
    }
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DecodingError {}
