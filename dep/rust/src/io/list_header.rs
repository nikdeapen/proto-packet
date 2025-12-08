use crate::io::DecodingError::InvalidListHeader;
use crate::io::WireType;
use enc::var_int::VarIntSize;
use enc::{impl_encode_to_write_stack_buf, DecodeFromReadPrefix, EncodeToSlice, EncodedLen, Error};
use std::io::Read;

/// A list header.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ListHeader {
    wire: WireType,
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
    pub const fn new(wire: WireType, size: usize) -> Self {
        Self { wire, size }
    }
}

impl ListHeader {
    //! Properties

    /// Gets the wire type. (of the values in the list)
    pub fn wire(&self) -> WireType {
        self.wire
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
            let first: u8 = self.wire.to_high_3_bits() | (self.size as u8);
            *target.get_unchecked_mut(0) = first;
            1
        } else {
            *target.get_unchecked_mut(0) = self.wire.to_high_3_bits() | 0x1F;
            let target: &mut [u8] = &mut target[1..];
            let extra: usize = self.size - Self::MAX_SINGLE_BYTE_LIST_SIZE;
            let extra: usize = VarIntSize::from(extra).encode_to_slice_unchecked(target)?;
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
            let extra: usize = VarIntSize::decode_from_read_prefix(r)
                .map_err(|e| match e {
                    Error::Stream(e) => Error::Stream(e),
                    _ => InvalidListHeader.into(),
                })?
                .value();
            extra + Self::MAX_SINGLE_BYTE_LIST_SIZE
        } else {
            size
        };
        Ok(Self::new(wire, size))
    }
}

#[cfg(test)]
mod test {
    use crate::io::WireType::*;
    use crate::io::{ListHeader, WireType};
    use enc::test;

    #[test]
    fn io() {
        let test_cases: &[(WireType, usize, &[u8])] = &[
            (Fixed1Byte, 1, &[0b0000_0001]),
            (Fixed2Byte, 2, &[0b0010_0010]),
            (Fixed4Byte, 3, &[0b0100_0011]),
            (Fixed8Byte, 0x1E, &[0b0111_1110]),
            (Fixed8Byte, 0x1F, &[0b0111_1111, 1]),
            (Fixed16Byte, 0x20, &[0b1001_1111, 2]),
            (VarInt, 0x21, &[0b1011_1111, 3]),
            (List, 0xFFFF_FFFF, &[0b1111_1111, 225, 255, 255, 255, 15]),
        ];
        for (wire, len, expected) in test_cases {
            let header: ListHeader = ListHeader::new(*wire, *len);
            test::test_encode(&header, expected);
            test::test_decode_from_read_prefix(expected, &header, false);
        }
    }
}
