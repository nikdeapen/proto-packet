use crate::io::WireType;
use enc::var_int::VarIntSize;
use enc::{impl_encode_to_write_stack_buf, DecodeFromReadPrefix, EncodeToSlice, EncodedLen, Error};
use std::io::Read;

/// A list header.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ListHeader {
    wire_type: WireType,
    size: usize,
}

impl ListHeader {
    //! Constants

    /// The maximum size (in bytes) of a list where the header can fit in a single byte. (30)
    pub const MAX_SINGLE_BYTE_LIST_SIZE: usize = 0x1E;

    /// The maximum encoded length of a list header.
    pub const MAX_ENCODED_LEN: usize = 1 + VarIntSize::MAX_ENCODED_LEN;
}

impl ListHeader {
    //! Construction

    /// Creates a new list header.
    pub const fn new(wire_type: WireType, size: usize) -> Self {
        Self { wire_type, size }
    }
}

impl ListHeader {
    //! Properties

    /// Gets the wire type. (of the list element)
    pub fn wire_type(&self) -> WireType {
        self.wire_type
    }

    /// Gets the list size. (in bytes)
    pub fn size(&self) -> usize {
        self.size
    }
}

impl EncodedLen for ListHeader {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(if self.size <= Self::MAX_SINGLE_BYTE_LIST_SIZE {
            1
        } else {
            let extra: usize = self.size - Self::MAX_SINGLE_BYTE_LIST_SIZE;
            let extra: usize = VarIntSize::from(extra).encoded_len()?;
            1 + extra
        })
    }
}

impl EncodeToSlice for ListHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        Ok(if self.size <= Self::MAX_SINGLE_BYTE_LIST_SIZE {
            let first: u8 = self.wire_type.to_high_3_bits() | (self.size as u8);
            *target.get_unchecked_mut(0) = first;
            1
        } else {
            *target.get_unchecked_mut(0) = self.wire_type.to_high_3_bits() | 0x1F;

            let extra: usize = self.size - Self::MAX_SINGLE_BYTE_LIST_SIZE;
            let extra: usize =
                VarIntSize::from(extra).encode_to_slice_unchecked(&mut target[1..])?;

            1 + extra
        })
    }
}

impl_encode_to_write_stack_buf!(ListHeader, Self::MAX_ENCODED_LEN);

impl DecodeFromReadPrefix for ListHeader {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        let wire: WireType = WireType::from_high_3_bits(first);
        let size: usize = (first & 0x1F) as usize;
        let size: usize = if size == 0x1F {
            let extra: usize = VarIntSize::decode_from_read_prefix(r)?.value();
            extra + Self::MAX_SINGLE_BYTE_LIST_SIZE
        } else {
            size
        };
        Ok(Self::new(wire, size))
    }
}

#[cfg(test)]
mod tests {
    // todo -- testing
}
