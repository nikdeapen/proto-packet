use crate::io::WireType;
use crate::io::WireType::*;
use enc::read_single_byte;
use std::io::{Read, Write};

impl WireType {
    //! Transfer

    /// Transfers the encoded value for this wire type from `r` to `w`.
    ///
    /// This reads the value bytes from `r` and writes them to `w`. The field header has already
    /// been consumed — this transfers only the value payload.
    pub fn transfer<R, W>(self, r: &mut R, w: &mut W) -> Result<(), enc::Error>
    where
        R: Read,
        W: Write,
    {
        match self {
            Fixed1Byte => Self::transfer_fixed(r, w, 1),
            Fixed2Byte => Self::transfer_fixed(r, w, 2),
            Fixed4Byte => Self::transfer_fixed(r, w, 4),
            Fixed8Byte => Self::transfer_fixed(r, w, 8),
            Fixed16Byte => Self::transfer_fixed(r, w, 16),
            VarInt => Self::transfer_varint(r, w),
            LengthPrefixed => Self::transfer_length_prefixed(r, w),
            List => Self::transfer_length_prefixed(r, w),
        }
    }

    /// Transfers `n` fixed bytes from `r` to `w`.
    fn transfer_fixed<R, W>(r: &mut R, w: &mut W, n: usize) -> Result<(), enc::Error>
    where
        R: Read,
        W: Write,
    {
        let mut buf: [u8; 16] = [0u8; 16];
        r.read_exact(&mut buf[..n])?;
        w.write_all(&buf[..n])?;
        Ok(())
    }

    /// Transfers a varint from `r` to `w`.
    fn transfer_varint<R, W>(r: &mut R, w: &mut W) -> Result<(), enc::Error>
    where
        R: Read,
        W: Write,
    {
        loop {
            let b: u8 = read_single_byte(r)?;
            w.write_all(&[b])?;
            if b & 0x80 == 0 {
                return Ok(());
            }
        }
    }

    /// Transfers a length-prefixed or list value from `r` to `w`.
    fn transfer_length_prefixed<R, W>(r: &mut R, w: &mut W) -> Result<(), enc::Error>
    where
        R: Read,
        W: Write,
    {
        use enc::DecodeFromReadPrefix;
        use enc::var_int::VarIntSize;

        let first: u8 = read_single_byte(r)?;
        let len: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)?.value();

        // Write the varint prefix bytes
        use enc::EncodeToWrite;
        VarIntSize::from(len).encode_to_write(w)?;

        // Transfer the payload
        let mut remaining: usize = len;
        let mut buf: [u8; 1024] = [0u8; 1024];
        while remaining > 0 {
            let chunk: usize = remaining.min(buf.len());
            r.read_exact(&mut buf[..chunk])?;
            w.write_all(&buf[..chunk])?;
            remaining -= chunk;
        }
        Ok(())
    }
}
