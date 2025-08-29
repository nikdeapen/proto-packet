use crate::io::{ListHeader, WireType};
use enc::Error::IntegerOverflow;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error, StreamError};
use std::io;

/// Gets the encoded length of the `list` value.
#[inline(always)]
pub fn list_size<T, ELF>(list: &[T], wire: WireType, encoded_len_fn: ELF) -> Result<usize, Error>
where
    ELF: Fn(&T) -> Result<usize, Error>,
{
    let fixed: usize = match wire {
        WireType::Fixed1Byte => return Ok(list.len()),
        WireType::Fixed2Byte => 2,
        WireType::Fixed4Byte => 4,
        WireType::Fixed8Byte => 8,
        WireType::Fixed16Byte => 16,
        _ => {
            let mut encoded_len: usize = 0;
            for value in list {
                encoded_len = encoded_len
                    .checked_add(encoded_len_fn(value)?)
                    .ok_or(IntegerOverflow)?;
            }
            return Ok(encoded_len);
        }
    };
    fixed.checked_mul(list.len()).ok_or(IntegerOverflow)
}

/// See `enc::EncodedLen`.
#[inline(always)]
pub fn encoded_len<T, ELF>(list: &[T], wire: WireType, encoded_len_fn: ELF) -> Result<usize, Error>
where
    ELF: Fn(&T) -> Result<usize, Error>,
{
    let size: usize = list_size(list, wire, encoded_len_fn)?;
    let header: usize = ListHeader::new(wire, size).encoded_len()?;
    header.checked_add(size).ok_or(IntegerOverflow)
}

/// See `enc::EncodeToSlice`.
#[inline(always)]
pub fn encode_to_slice<T, EF, ELF>(
    list: &[T],
    wire: WireType,
    target: &mut [u8],
    encode_fn: EF,
    encoded_len_fn: ELF,
) -> Result<usize, Error>
where
    EF: Fn(&T, &mut [u8]) -> Result<usize, Error>,
    ELF: Fn(&T) -> Result<usize, Error>,
{
    let size: usize = list_size(list, wire, encoded_len_fn)?;
    let header: usize = ListHeader::new(wire, size).encode_to_slice(target)?;

    let mut encoded_len: usize = header;
    for value in list {
        let current: usize = encode_fn(value, &mut target[encoded_len..])?;
        encoded_len = encoded_len.checked_add(current).ok_or(IntegerOverflow)?;
    }

    debug_assert_eq!(encoded_len, header + size);
    Ok(encoded_len)
}

/// See `enc::EncodeToWrite`.
#[inline(always)]
pub fn encode_to_write<T, W, EF, ELF>(
    list: &[T],
    wire: WireType,
    w: &mut W,
    encode_fn: EF,
    encoded_len_fn: ELF,
) -> Result<usize, StreamError>
where
    W: io::Write,
    EF: Fn(&T, &mut W) -> Result<usize, StreamError>,
    ELF: Fn(&T) -> Result<usize, Error>,
{
    let size: usize = list_size(list, wire, encoded_len_fn)?;
    let header: usize = ListHeader::new(wire, size).encode_to_write(w)?;

    let mut encoded_len: usize = header;
    for value in list {
        let current: usize = encode_fn(value, w)?;
        encoded_len = encoded_len.checked_add(current).ok_or(IntegerOverflow)?;
    }

    debug_assert_eq!(encoded_len, header + size);
    Ok(encoded_len)
}
