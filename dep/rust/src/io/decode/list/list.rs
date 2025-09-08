use crate::io::{Decoder, DecodingError, ListHeader, WireType};
use enc::{read_optional_byte, DecodeFromReadPrefix};
use std::cmp::max;
use std::io::{Read, Take};

impl Decoder {
    //! Generic: List

    /// Decodes a generic list value from the `Read` prefix with the `first` byte.
    pub fn decode_list<T, R, F>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
        decode_fn: F,
    ) -> Result<Vec<T>, DecodingError>
    where
        R: Read,
        F: Fn(WireType, &mut Take<&mut R>, u8) -> Result<T, DecodingError>,
    {
        match wire {
            WireType::List => {
                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(|e| DecodingError::from_list_header(e))?;
                self.decode_list_value(header, r, |r, first| {
                    decode_fn(header.wire_type(), r, first)
                })
            }
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }

    /// Decodes a list.
    pub fn decode_list_value<T, F, R>(
        &self,
        header: ListHeader,
        r: &mut R,
        decode_fn: F,
    ) -> Result<Vec<T>, DecodingError>
    where
        R: Read,
        F: Fn(&mut Take<&mut R>, u8) -> Result<T, DecodingError>,
    {
        let mut buffer: Vec<T> = Vec::with_capacity(Self::list_buffer_capacity(header));

        let mut r: Take<&mut R> = Read::take(r, header.size() as u64);
        while let Some(first) = read_optional_byte(&mut r).map_err(DecodingError::Source)? {
            let value: T = decode_fn(&mut r, first)?;
            buffer.push(value);
        }

        buffer.shrink_to_fit();
        Ok(buffer)
    }

    fn list_buffer_capacity(header: ListHeader) -> usize {
        if header.size() == 0 {
            0
        } else {
            let fixed: usize = match header.wire_type() {
                WireType::Fixed1Byte => 1,
                WireType::Fixed2Byte => 2,
                WireType::Fixed4Byte => 4,
                WireType::Fixed8Byte => 8,
                WireType::Fixed16Byte => 16,
                WireType::VarInt => return max(header.size() / 4, 16),
                WireType::LengthPrefixed => return 8,
                WireType::List => return 8,
            };
            debug_assert!(header.size() % fixed == 0);
            header.size() / fixed
        }
    }
}
