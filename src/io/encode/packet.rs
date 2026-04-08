use crate::Packet;
use crate::io::Encoder;
use crate::io::WireType::LengthPrefixed;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl<P: Packet> Encoder<'_, P> {
    //! Utilities

    /// Gets the byte length of the length prefix.
    ///
    /// Returns `0` if [P] is not length-prefixed.
    fn prefix_len(body_len: usize) -> Result<usize, Error> {
        if P::wire() == LengthPrefixed {
            VarIntSize::from(body_len).encoded_len()
        } else {
            Ok(0)
        }
    }

    /// Gets the length-prefix for the value.
    ///
    /// Returns `None` if [P] is not length-prefixed.
    fn length_prefix(&self) -> Result<Option<VarIntSize>, Error> {
        if P::wire() == LengthPrefixed {
            Ok(Some(VarIntSize::from(self.value.encoded_len()?)))
        } else {
            Ok(None)
        }
    }
}

impl<P: Packet> EncodedLen for Encoder<'_, P> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let body_len: usize = self.value.encoded_len()?;
        let prefix_len: usize = Self::prefix_len(body_len)?;
        Ok(prefix_len + body_len)
    }
}

impl<P: Packet> EncodeToSlice for Encoder<'_, P> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let prefix_len: usize = match self.length_prefix()? {
            Some(varint) => unsafe { varint.encode_to_slice_unchecked(target)? },
            None => 0,
        };
        let written: usize = unsafe {
            self.value
                .encode_to_slice_unchecked(target.get_unchecked_mut(prefix_len..))?
        };
        Ok(prefix_len + written)
    }
}

impl<P: Packet> EncodeToWrite for Encoder<'_, P> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let prefix_len: usize = match self.length_prefix()? {
            Some(varint) => varint.encode_to_write(w)?,
            None => 0,
        };
        let written: usize = self.value.encode_to_write(w)?;
        Ok(prefix_len + written)
    }
}
