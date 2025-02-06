use std::io::{Error, Write};

use enc::var_int::VarIntSize;
use enc::Error::IntegerOverflow;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::WireType::LengthPrefixed;
use crate::Packet;

pub fn encoded_len_slice_packet<P>(slice: &[P]) -> Result<usize, enc::Error>
where
    P: Packet,
{
    let mut encoded_len: usize = 0;
    if P::wire_type() == LengthPrefixed {
        for value in slice {
            let value_len: usize = value.encoded_len()?;
            encoded_len = encoded_len
                .checked_add(VarIntSize::from(value_len).encoded_len()?)
                .ok_or(IntegerOverflow)?;
            encoded_len = encoded_len
                .checked_add(value.encoded_len()?)
                .ok_or(IntegerOverflow)?;
        }
    } else {
        for value in slice {
            encoded_len = encoded_len
                .checked_add(value.encoded_len()?)
                .ok_or(IntegerOverflow)?;
        }
    }
    Ok(encoded_len)
}

pub unsafe fn encode_to_slice_slice_packet<P>(
    slice: &[P],
    target: &mut [u8],
) -> Result<usize, enc::Error>
where
    P: Packet,
{
    let mut encoded_len: usize = 0;
    if P::wire_type() == LengthPrefixed {
        for value in slice {
            let value_len: usize = value.encoded_len()?;
            encoded_len +=
                VarIntSize::from(value_len).encode_to_slice(&mut target[encoded_len..])?;
            encoded_len += value.encode_to_slice(&mut target[encoded_len..])?;
        }
    } else {
        for value in slice {
            encoded_len += value.encode_to_slice(&mut target[encoded_len..])?;
        }
    }
    Ok(encoded_len)
}

pub fn encode_to_write_slice_packet<P, W>(slice: &[P], w: &mut W) -> Result<usize, Error>
where
    P: Packet,
    W: Write,
{
    let mut encoded_len: usize = 0;
    if P::wire_type() == LengthPrefixed {
        for value in slice {
            let value_len: usize = value.encoded_len()?;
            encoded_len = encoded_len
                .checked_add(VarIntSize::from(value_len).encode_to_write(w)?)
                .ok_or(IntegerOverflow)?;
            encoded_len = encoded_len
                .checked_add(value.encode_to_write(w)?)
                .ok_or(IntegerOverflow)?;
        }
    } else {
        for value in slice {
            encoded_len = encoded_len
                .checked_add(value.encode_to_write(w)?)
                .ok_or(IntegerOverflow)?;
        }
    }
    Ok(encoded_len)
}
