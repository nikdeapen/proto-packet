use crate::io::Encoder;
use crate::io::WireType::LengthPrefixed;
use crate::Packet;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl<P: Packet> EncodedLen for Encoder<'_, P> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let value: usize = self.value.encoded_len()?;
        let prefix: usize = if P::wire_type() == LengthPrefixed {
            VarIntSize::from(value).encoded_len()?
        } else {
            0
        };
        Ok(prefix + value)
    }
}

impl<P: Packet> EncodeToSlice for Encoder<'_, P> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        if P::wire_type() == LengthPrefixed {
            let value: usize = self.value.encoded_len()?;
            let prefix: usize = VarIntSize::from(value).encode_to_slice_unchecked(target)?;
            self.value
                .encode_to_slice_unchecked(&mut target[prefix..])?;
            Ok(prefix + value)
        } else {
            self.value.encode_to_slice_unchecked(target)
        }
    }
}

impl<P: Packet> EncodeToWrite for Encoder<'_, P> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        if P::wire_type() == LengthPrefixed {
            let value: usize = self.value.encoded_len()?;
            let prefix: usize = VarIntSize::from(value).encode_to_write(w)?;
            self.value.encode_to_write(w)?;
            Ok(prefix + value)
        } else {
            self.value.encode_to_write(w)
        }
    }
}

// todo -- unit tests
