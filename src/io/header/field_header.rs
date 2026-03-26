use crate::io::{TagNumber, WireType};
use enc::var_int::VarInt32;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error, read_single_byte};
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};

/// A field header.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FieldHeader {
    wire: WireType,
    tag: TagNumber,
}

impl FieldHeader {
    //! Constants

    /// The maximum tag number of a field header that fits in 1 byte. (31)
    pub const MAX_SINGLE_BYTE_TAG_NUMBER: u32 = 0x1F;
}

impl FieldHeader {
    //! Construction

    /// Creates a new field header.
    pub const fn new(wire: WireType, tag: TagNumber) -> Self {
        Self { wire, tag }
    }
}

impl FieldHeader {
    //! Properties

    /// Gets the wire type.
    pub const fn wire(self) -> WireType {
        self.wire
    }

    /// Gets the tag number.
    pub const fn tag(self) -> TagNumber {
        self.tag
    }
}

impl FieldHeader {
    //! Encoding Utilities

    /// Checks if the field header is encodable within a single byte.
    fn is_single_byte(self) -> bool {
        self.tag.value() <= Self::MAX_SINGLE_BYTE_TAG_NUMBER
    }

    /// Encodes the first byte.
    fn first_byte(self) -> u8 {
        let wire: u8 = self.wire.to_high_3_bits();
        if self.is_single_byte() {
            wire | (self.tag.value() as u8)
        } else {
            wire
        }
    }
}

impl EncodedLen for FieldHeader {
    fn encoded_len(&self) -> Result<usize, Error> {
        if self.is_single_byte() {
            Ok(1)
        } else {
            let adjusted: u32 = self.tag.value() - Self::MAX_SINGLE_BYTE_TAG_NUMBER;
            let tag_len: usize = VarInt32::from(adjusted).encoded_len()?;
            Ok(1 + tag_len)
        }
    }
}

impl EncodeToSlice for FieldHeader {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        target[0] = self.first_byte();
        if self.is_single_byte() {
            Ok(1)
        } else {
            let adjusted: u32 = self.tag.value() - Self::MAX_SINGLE_BYTE_TAG_NUMBER;
            let written: usize =
                unsafe { VarInt32::from(adjusted).encode_to_slice_unchecked(&mut target[1..])? };
            Ok(1 + written)
        }
    }
}

impl EncodeToWrite for FieldHeader {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        w.write_all(&[self.first_byte()])?;
        if self.is_single_byte() {
            Ok(1)
        } else {
            let adjusted: u32 = self.tag.value() - Self::MAX_SINGLE_BYTE_TAG_NUMBER;
            let written: usize = VarInt32::from(adjusted).encode_to_write(w)?;
            Ok(1 + written)
        }
    }
}

impl enc::DecodeFromReadPrefix for FieldHeader {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        let wire: WireType = WireType::from_high_3_bits(first);
        let tag_bits: u8 = first & 0x1F;
        let tag_value: u32 = if tag_bits != 0 {
            tag_bits as u32
        } else {
            let first: u8 = read_single_byte(r)?;
            let adjusted: u32 =
                VarInt32::decode_from_read_prefix_with_first_byte(r, first)?.value();
            adjusted + Self::MAX_SINGLE_BYTE_TAG_NUMBER
        };
        let tag: TagNumber =
            TagNumber::new(tag_value).ok_or(Error::InvalidEncodedData { reason: None })?;
        Ok(Self { wire, tag })
    }
}

impl Debug for FieldHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for FieldHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.wire, self.tag)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::{FieldHeader, TagNumber, WireType};
    use enc::{DecodeFromReadPrefix, EncodeToSlice, EncodedLen};

    #[test]
    fn compact_roundtrip() {
        let tag: TagNumber = TagNumber::new(1).unwrap();
        let header: FieldHeader = FieldHeader::new(WireType::VarInt, tag);
        assert_eq!(header.encoded_len().unwrap(), 1);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        assert_eq!(encoded.len(), 1);
        assert_eq!(encoded[0] & 0xE0, WireType::VarInt.to_high_3_bits());
        assert_eq!(encoded[0] & 0x1F, 1);

        let decoded: FieldHeader =
            FieldHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn compact_max_tag() {
        let tag: TagNumber = TagNumber::new(31).unwrap();
        let header: FieldHeader = FieldHeader::new(WireType::LengthPrefixed, tag);
        assert_eq!(header.encoded_len().unwrap(), 1);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        let decoded: FieldHeader =
            FieldHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn overflow_tag() {
        let tag: TagNumber = TagNumber::new(32).unwrap();
        let header: FieldHeader = FieldHeader::new(WireType::Fixed4Byte, tag);
        assert!(header.encoded_len().unwrap() > 1);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        assert_eq!(encoded[0] & 0x1F, 0);

        let decoded: FieldHeader =
            FieldHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn large_tag_roundtrip() {
        let tag: TagNumber = TagNumber::new(TagNumber::MAX_TAG_NUMBER).unwrap();
        let header: FieldHeader = FieldHeader::new(WireType::List, tag);

        let encoded: Vec<u8> = header.encode_as_vec().unwrap();
        let decoded: FieldHeader =
            FieldHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, header);
    }

    #[test]
    fn all_wire_types_compact() {
        for wire_id in 0u8..8 {
            let wire: WireType = WireType::from_low_3_bits(wire_id);
            let tag: TagNumber = TagNumber::new(5).unwrap();
            let header: FieldHeader = FieldHeader::new(wire, tag);

            let encoded: Vec<u8> = header.encode_as_vec().unwrap();
            let decoded: FieldHeader =
                FieldHeader::decode_from_read_prefix(&mut encoded.as_slice()).unwrap();
            assert_eq!(decoded, header);
        }
    }
}
