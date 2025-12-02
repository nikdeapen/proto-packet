/// An encoding operation.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum EncodeOp {
    EncodedLen,
    EncodeToSlice,
    EncodeToWrite,
}

impl EncodeOp {
    //! Strings

    /// Gets the encoding call function name.
    pub fn encode_call_fn_name(&self) -> &str {
        match self {
            Self::EncodedLen => "encoded_len",
            Self::EncodeToSlice => "encode_to_slice_unchecked",
            Self::EncodeToWrite => "encode_to_write",
        }
    }

    /// Gets the encoding call parameters.
    pub fn encode_call_params(&self) -> &str {
        match self {
            Self::EncodedLen => "",
            Self::EncodeToSlice => "&mut target[encoded_len..]",
            Self::EncodeToWrite => "w",
        }
    }

    /// Gets the full encoding call after the `.`.
    ///
    /// Uses the default parameter names. (`target` and `w`)
    pub fn encode_call(&self) -> String {
        format!(
            "{}({})",
            self.encode_call_fn_name(),
            self.encode_call_params()
        )
    }
}
