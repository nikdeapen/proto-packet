/// An encoding operation.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum EncodeOp {
    EncodedLen,
    EncodeToSlice,
    EncodeToWrite,
}

impl EncodeOp {
    //! Strings

    pub fn encode_call(&self) -> &str {
        match self {
            Self::EncodedLen => "encoded_len()",
            Self::EncodeToSlice => "encode_to_slice_unchecked(&mut target[encoded_len..])",
            Self::EncodeToWrite => "encode_to_write(w)",
        }
    }

    pub fn encode_tag(&self) -> &str {
        match self {
            Self::EncodedLen => "encoded_len",
            Self::EncodeToSlice => "encode_to_slice_unchecked",
            Self::EncodeToWrite => "encode_to_write",
        }
    }

    pub fn encode_params(&self) -> &str {
        match self {
            Self::EncodedLen => "",
            Self::EncodeToSlice => "&mut target[encoded_len..]",
            Self::EncodeToWrite => "w",
        }
    }

    pub fn encode_extra_params(&self) -> &str {
        match self {
            Self::EncodedLen => "",
            Self::EncodeToSlice => ", &mut target[encoded_len..]",
            Self::EncodeToWrite => ", w",
        }
    }
}
