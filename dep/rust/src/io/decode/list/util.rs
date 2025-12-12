use crate::io::{DecodingError, ListHeader, WireType};
use std::cmp::max;
use std::io::Read;

/// Decodes a generic list from the `Read` prefix with the `header`.
pub(in crate::io::decode::list) fn decode_generic_list<R, T, F>(
    r: &mut R,
    header: ListHeader,
    decode_fn: F,
) -> Result<Vec<T>, DecodingError>
where
    R: Read,
    F: Fn(WireType, &mut R, u8) -> Result<T, DecodingError>,
{
    let mut list: Vec<T> = Vec::with_capacity(list_capacity(header));
    while let Some(first) = enc::read_optional_byte(r)? {
        let value: T = decode_fn(header.wire(), r, first)?;
        list.push(value);
    }
    Ok(list)
}

/// Gets the pre-allocated list capacity.
fn list_capacity(header: ListHeader) -> usize {
    match header.wire() {
        WireType::Fixed1Byte => header.size(),
        WireType::Fixed2Byte => header.size() / 2,
        WireType::Fixed4Byte => header.size() / 4,
        WireType::Fixed8Byte => header.size() / 8,
        WireType::Fixed16Byte => header.size() / 16,
        WireType::VarInt => max(header.size() / 2, 8),
        WireType::LengthPrefixed | WireType::List => 8,
    }
}
