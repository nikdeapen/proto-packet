use crate::io::WireType;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error, read_single_byte};
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};

/// A list header.
///
/// # Encoding
/// The first byte encodes the element wire type in the high 3 bits and the size in bytes in the
/// low 5 bits. If the low 5 bits are `0x1F` (the overflow sentinel), the size is encoded as a
/// `VarIntSize` in the subsequent bytes, adjusted by `MAX_SINGLE_BYTE_SIZE`.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ListHeader {
    wire: WireType,
    size: usize,
}

impl ListHeader {
    //! Constants

    /// The maximum size (in bytes) of a list with a single byte list header. (30)
    pub const MAX_SINGLE_BYTE_SIZE: usize = 0x1E;

    /// The overflow sentinel value in the low 5 bits. (31)
    const OVERFLOW_SENTINEL: u8 = 0x1F;
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

    /// Gets the wire type of the list elements.
    pub const fn wire(self) -> WireType {
        self.wire
    }

    /// Gets the size of the list. (in bytes)
    pub const fn size(self) -> usize {
        self.size
    }
}

impl ListHeader {
    //! Encoding Utilities

    /// Checks if the list header is encodable within a single byte.
    fn is_single_byte(self) -> bool {
        self.size <= Self::MAX_SINGLE_BYTE_SIZE
    }

    /// Encodes the first byte.
    fn first_byte(self) -> u8 {
        let wire: u8 = self.wire.to_high_3_bits();
        if self.is_single_byte() {
            wire | (self.size as u8)
        } else {
            wire | Self::OVERFLOW_SENTINEL
        }
    }
}

impl EncodedLen for ListHeader {
    fn encoded_len(&self) -> Result<usize, Error> {
        if self.is_single_byte() {
            Ok(1)
        } else {
            let adjusted: usize = self.size - Self::MAX_SINGLE_BYTE_SIZE;
            let size_len: usize = VarIntSize::from(adjusted).encoded_len()?;
            Ok(1 + size_len)
        }
    }
}

impl EncodeToSlice for ListHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        target[0] = self.first_byte();
        if self.is_single_byte() {
            Ok(1)
        } else {
            let adjusted: usize = self.size - Self::MAX_SINGLE_BYTE_SIZE;
            let written: usize =
                unsafe { VarIntSize::from(adjusted).encode_to_slice_unchecked(&mut target[1..])? };
            Ok(1 + written)
        }
    }
}

impl EncodeToWrite for ListHeader {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        w.write_all(&[self.first_byte()])?;
        if self.is_single_byte() {
            Ok(1)
        } else {
            let adjusted: usize = self.size - Self::MAX_SINGLE_BYTE_SIZE;
            let written: usize = VarIntSize::from(adjusted).encode_to_write(w)?;
            Ok(1 + written)
        }
    }
}

impl enc::DecodeFromReadPrefix for ListHeader {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        let wire: WireType = WireType::from_high_3_bits(first);
        let size_bits: u8 = first & 0x1F;
        let size: usize = if size_bits != Self::OVERFLOW_SENTINEL {
            size_bits as usize
        } else {
            let first: u8 = read_single_byte(r)?;
            let adjusted: usize =
                VarIntSize::decode_from_read_prefix_with_first_byte(r, first)?.value();
            adjusted + Self::MAX_SINGLE_BYTE_SIZE
        };
        Ok(Self { wire, size })
    }
}

impl Debug for ListHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ListHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.wire, self.size)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType;
    use crate::io::header::list_header::ListHeader;
    use enc::{DecodeFromReadPrefix, EncodeToSlice, EncodedLen};

    #[test]
    fn empty_list() {
        let header: ListHeader = ListHeader::new(WireType::VarInt, 0);
        assert_eq!(header.encoded_len().unwrap(), 1);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        assert_eq!(encoded[0] & 0x1F, 0);

        let decoded: ListHeader =
            ListHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn compact_max_size() {
        let header: ListHeader = ListHeader::new(WireType::Fixed4Byte, 30);
        assert_eq!(header.encoded_len().unwrap(), 1);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        assert_eq!(encoded[0] & 0x1F, 30);

        let decoded: ListHeader =
            ListHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn overflow_size() {
        let header: ListHeader = ListHeader::new(WireType::Fixed8Byte, 31);
        assert!(header.encoded_len().unwrap() > 1);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        assert_eq!(encoded[0] & 0x1F, 0x1F);

        let decoded: ListHeader =
            ListHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn large_size_roundtrip() {
        let header: ListHeader = ListHeader::new(WireType::LengthPrefixed, 100_000);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        let decoded: ListHeader =
            ListHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn all_wire_types_compact() {
        for wire_id in 0u8..8 {
            let wire: WireType = WireType::from_low_3_bits(wire_id);
            let header: ListHeader = ListHeader::new(wire, 10);

            let encoded: Vec<u8> = header.encode_as_vec().unwrap();
            let decoded: ListHeader =
                ListHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
            assert_eq!(decoded, header);
        }
    }
}
