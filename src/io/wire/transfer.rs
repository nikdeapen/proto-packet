use crate::io::WireType;
use crate::io::WireType::*;
use enc::read_single_byte;
use enc::var_int::VarInt128;
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
        let mut buf: [u8; VarInt128::MAX_ENCODED_LEN] = [0u8; VarInt128::MAX_ENCODED_LEN];
        let mut len: usize = 0;
        while len < VarInt128::MAX_ENCODED_LEN {
            let b: u8 = read_single_byte(r)?;
            buf[len] = b;
            len += 1;
            if b & 0x80 == 0 {
                w.write_all(&buf[..len])?;
                return Ok(());
            }
        }
        Err(enc::Error::InvalidEncodedData { reason: None }) // todo -- better error here
    }

    /// Transfers a length-prefixed or list value from `r` to `w`.
    ///
    /// The varint length prefix is read into a small stack buffer and forwarded directly (skipping
    /// the decode+re-encode round trip), while still being validated by the canonical varint
    /// decoder. The payload bytes are then forwarded blindly — they are not validated since they
    /// may contain any nested packet structure.
    fn transfer_length_prefixed<R, W>(r: &mut R, w: &mut W) -> Result<(), enc::Error>
    where
        R: Read,
        W: Write,
    {
        use enc::DecodeFromReadPrefix;
        use enc::var_int::VarIntSize;

        // Read varint bytes into a stack buffer until the high-bit-clear terminator.
        let mut prefix_buf: [u8; VarIntSize::MAX_ENCODED_LEN] = [0u8; VarIntSize::MAX_ENCODED_LEN];
        let mut prefix_len: usize = 0;
        loop {
            if prefix_len >= VarIntSize::MAX_ENCODED_LEN {
                return Err(enc::Error::InvalidEncodedData { reason: None });
            }
            let b: u8 = read_single_byte(r)?;
            prefix_buf[prefix_len] = b;
            prefix_len += 1;
            if b & 0x80 == 0 {
                break;
            }
        }

        // Decode the buffered prefix to validate it and extract the payload length.
        let mut prefix_rest: &[u8] = &prefix_buf[1..prefix_len];
        let len: usize =
            VarIntSize::decode_from_read_prefix_with_first_byte(&mut prefix_rest, prefix_buf[0])?
                .value();

        // Forward the prefix bytes verbatim.
        w.write_all(&prefix_buf[..prefix_len])?;

        // Transfer the payload.
        let mut remaining: usize = len;
        let mut buf: [u8; 1024] = [0u8; 1024]; // todo -- why 1024?
        while remaining > 0 {
            let chunk: usize = remaining.min(buf.len());
            r.read_exact(&mut buf[..chunk])?;
            w.write_all(&buf[..chunk])?;
            remaining -= chunk;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType;
    use crate::io::WireType::*;
    use std::io::Cursor;

    #[test]
    #[allow(clippy::type_complexity)]
    fn transfer() {
        // 1500 byte payload to exercise the multi-chunk path in `transfer_length_prefixed`
        // (chunk size = 1024). varint(1500) = [0xDC, 0x0B].
        let mut large_lp: Vec<u8> = vec![0xDC, 0x0B];
        large_lp.extend(std::iter::repeat_n(0xAB, 1500));

        // 20 bytes all with the continuation bit set — exceeds VarInt128::MAX_ENCODED_LEN (19).
        let overlong_varint: [u8; 20] = [0xFF; 20];

        // 16 bytes all with the continuation bit set — exceeds the usize varint max for any
        // supported pointer width (max is 10 on 64-bit, 5 on 32-bit).
        let overlong_lp_prefix: [u8; 16] = [0xFF; 16];

        // Each case: (wire type, input bytes, Some(expected output) for Ok, None for Err).
        let cases: &[(WireType, &[u8], Option<&[u8]>)] = &[
            // Fixed widths — output equals input.
            (Fixed1Byte, &[0x42], Some(&[0x42])),
            (Fixed2Byte, &[1, 2], Some(&[1, 2])),
            (Fixed4Byte, &[1, 2, 3, 4], Some(&[1, 2, 3, 4])),
            (
                Fixed8Byte,
                &[1, 2, 3, 4, 5, 6, 7, 8],
                Some(&[1, 2, 3, 4, 5, 6, 7, 8]),
            ),
            (
                Fixed16Byte,
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
                Some(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            ),
            // VarInt: 1, 2, 3, and 5 byte forms.
            (VarInt, &[0x42], Some(&[0x42])),
            (VarInt, &[0xAC, 0x02], Some(&[0xAC, 0x02])),
            (VarInt, &[0xFF, 0xFF, 0x03], Some(&[0xFF, 0xFF, 0x03])),
            (
                VarInt,
                &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F],
                Some(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
            ),
            // VarInt: overlong — should reject.
            (VarInt, &overlong_varint, None),
            // LengthPrefixed: empty body, small body, large body that crosses the chunk boundary.
            (LengthPrefixed, &[0x00], Some(&[0x00])),
            (
                LengthPrefixed,
                &[0x03, b'a', b'b', b'c'],
                Some(&[0x03, b'a', b'b', b'c']),
            ),
            (
                LengthPrefixed,
                large_lp.as_slice(),
                Some(large_lp.as_slice()),
            ),
            // LengthPrefixed: overlong prefix — should reject.
            (LengthPrefixed, &overlong_lp_prefix, None),
            // List uses the same path as LengthPrefixed.
            (List, &[0x00], Some(&[0x00])),
            (List, &[0x03, 1, 2, 3], Some(&[0x03, 1, 2, 3])),
        ];

        for (wire, input, expected) in cases {
            let mut r: Cursor<&[u8]> = Cursor::new(*input);
            let mut w: Vec<u8> = Vec::new();
            let result: Result<(), enc::Error> = wire.transfer(&mut r, &mut w);
            match (result, expected) {
                (Ok(()), Some(exp)) => {
                    assert_eq!(
                        w.as_slice(),
                        *exp,
                        "wire={wire:?} input_len={}",
                        input.len()
                    );
                }
                (Err(_), None) => {}
                (Ok(()), None) => {
                    panic!(
                        "wire={wire:?} input_len={}: expected error, got Ok",
                        input.len()
                    )
                }
                (Err(e), Some(_)) => {
                    panic!(
                        "wire={wire:?} input_len={}: expected Ok, got error: {e:?}",
                        input.len()
                    )
                }
            }
        }
    }
}
