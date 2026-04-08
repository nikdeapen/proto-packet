use crate::io::Encoder;
use enc::var_int::{VarInt16, VarInt32, VarInt64, VarInt128};
use enc::{EncodeToSlice, EncodedLen, Error, impl_encode_to_write_stack_buf};

macro_rules! encode {
    ($primitive:ident, $var_int:ident, $convert:ident) => {
        impl Encoder<'_, $primitive> {
            //! Constants

            /// The fixed encoded length.
            const FIXED_ENCODED_LEN: usize = $primitive::BITS as usize / 8;

            /// The maximum encoded length.
            const MAX_ENCODED_LEN: usize = $var_int::MAX_ENCODED_LEN;
        }

        // The stack buffer used by `impl_encode_to_write_stack_buf!` is sized to
        // `MAX_ENCODED_LEN`, so it must be at least as large as `FIXED_ENCODED_LEN`
        // for the fixed encoding to fit.
        const _: () = assert!(
            Encoder::<'_, $primitive>::MAX_ENCODED_LEN
                >= Encoder::<'_, $primitive>::FIXED_ENCODED_LEN,
            concat!(
                "Encoder<'_, ",
                stringify!($primitive),
                ">: MAX_ENCODED_LEN must be >= FIXED_ENCODED_LEN.",
            ),
        );

        impl EncodedLen for Encoder<'_, $primitive> {
            fn encoded_len(&self) -> Result<usize, Error> {
                if self.fixed {
                    Ok(Self::FIXED_ENCODED_LEN)
                } else {
                    $var_int::$convert(*self.value).encoded_len()
                }
            }
        }

        impl EncodeToSlice for Encoder<'_, $primitive> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                if self.fixed {
                    unsafe {
                        target
                            .as_mut_ptr()
                            .cast::<$primitive>()
                            .write_unaligned(self.value.to_le());
                    }
                    Ok(Self::FIXED_ENCODED_LEN)
                } else {
                    unsafe { $var_int::$convert(*self.value).encode_to_slice_unchecked(target) }
                }
            }
        }

        impl_encode_to_write_stack_buf!(
            Encoder<'_, $primitive>,
            Encoder::<'_, $primitive>::MAX_ENCODED_LEN
        );
    };
}

encode!(u16, VarInt16, from);
encode!(u32, VarInt32, from);
encode!(u64, VarInt64, from);
encode!(u128, VarInt128, from);
encode!(i16, VarInt16, from_zigzag);
encode!(i32, VarInt32, from_zigzag);
encode!(i64, VarInt64, from_zigzag);
encode!(i128, VarInt128, from_zigzag);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::var_int::{VarInt16, VarInt32, VarInt64, VarInt128};
    use enc::{EncodeToSlice, test};

    #[test]
    fn encode_u16() {
        let cases: &[(u16, bool)] = &[
            (0, true),
            (0, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (u16::MAX, true),
            (u16::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt16::from(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, u16> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }

    #[test]
    fn encode_u32() {
        let cases: &[(u32, bool)] = &[
            (0, true),
            (0, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (u32::MAX, true),
            (u32::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt32::from(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, u32> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }

    #[test]
    fn encode_u64() {
        let cases: &[(u64, bool)] = &[
            (0, true),
            (0, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (u64::MAX, true),
            (u64::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt64::from(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, u64> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }

    #[test]
    fn encode_u128() {
        let cases: &[(u128, bool)] = &[
            (0, true),
            (0, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (u128::MAX, true),
            (u128::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt128::from(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, u128> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }

    #[test]
    fn encode_i16() {
        let cases: &[(i16, bool)] = &[
            (0, true),
            (0, false),
            (-1, true),
            (-1, false),
            (1, true),
            (1, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (i16::MIN, true),
            (i16::MIN, false),
            (i16::MAX, true),
            (i16::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt16::from_zigzag(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, i16> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }

    #[test]
    fn encode_i32() {
        let cases: &[(i32, bool)] = &[
            (0, true),
            (0, false),
            (-1, true),
            (-1, false),
            (1, true),
            (1, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (i32::MIN, true),
            (i32::MIN, false),
            (i32::MAX, true),
            (i32::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt32::from_zigzag(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, i32> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }

    #[test]
    fn encode_i64() {
        let cases: &[(i64, bool)] = &[
            (0, true),
            (0, false),
            (-1, true),
            (-1, false),
            (1, true),
            (1, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (i64::MIN, true),
            (i64::MIN, false),
            (i64::MAX, true),
            (i64::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt64::from_zigzag(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, i64> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }

    #[test]
    fn encode_i128() {
        let cases: &[(i128, bool)] = &[
            (0, true),
            (0, false),
            (-1, true),
            (-1, false),
            (1, true),
            (1, false),
            (0x7F, true),
            (0x7F, false),
            (0xFF, true),
            (0xFF, false),
            (i128::MIN, true),
            (i128::MIN, false),
            (i128::MAX, true),
            (i128::MAX, false),
        ];

        for (value, fixed) in cases {
            let expected: Vec<u8> = if *fixed {
                value.to_le_bytes().to_vec()
            } else {
                VarInt128::from_zigzag(*value).encode_as_vec().unwrap()
            };
            let encoder: Encoder<'_, i128> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, &expected);
        }
    }
}
