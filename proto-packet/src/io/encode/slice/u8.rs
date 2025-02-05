use std::io::{Error, Write};

pub fn encoded_len_slice_u8(value: &[u8]) -> Result<usize, enc::Error> {
    Ok(value.len())
}

pub unsafe fn encode_to_slice_slice_u8(
    value: &[u8],
    target: &mut [u8],
) -> Result<usize, enc::Error> {
    (&mut target[..value.len()]).copy_from_slice(value);
    Ok(value.len())
}

pub fn encode_to_write_slice_u8<W>(value: &[u8], w: &mut W) -> Result<usize, Error>
where
    W: Write,
{
    w.write_all(value)?;
    Ok(value.len())
}
