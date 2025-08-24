use crate::io::Encoder;
use enc::var_int::{VarInt128, VarInt16, VarInt32, VarInt64};
use enc::{impl_encode_to_write_stack_buf, EncodeToSlice, EncodedLen, Error};

macro_rules! constants {
    ($primitive:ident, $var_int:ident) => {
        impl Encoder<'_, $primitive> {
            //! Constants

            /// The fixed encoded length.
            const FIXED_ENCODED_LEN: usize = $primitive::BITS as usize / 8;

            /// The maximum encoded length.
            const MAX_ENCODED_LEN: usize = $var_int::MAX_ENCODED_LEN;
        }
    };
}

macro_rules! encoded_len {
    ($primitive:ident, $var_int:ident) => {
        impl EncodedLen for Encoder<'_, $primitive> {
            fn encoded_len(&self) -> Result<usize, Error> {
                if self.fixed {
                    Ok(Self::FIXED_ENCODED_LEN)
                } else {
                    $var_int::from(self.value).encoded_len()
                }
            }
        }
    };
}

macro_rules! encode_to_slice {
    ($primitive:ident, $var_int:ident) => {
        impl EncodeToSlice for Encoder<'_, $primitive> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                if self.fixed {
                    (&mut target[..Self::FIXED_ENCODED_LEN])
                        .copy_from_slice(&self.value.to_le_bytes());
                    Ok(Self::FIXED_ENCODED_LEN)
                } else {
                    $var_int::from(self.value).encode_to_slice_unchecked(target)
                }
            }
        }
    };
}

macro_rules! encode {
    ($primitive:ident, $var_int:ident) => {
        constants!($primitive, $var_int);

        encoded_len!($primitive, $var_int);

        encode_to_slice!($primitive, $var_int);

        impl_encode_to_write_stack_buf!(
            Encoder<'_, $primitive>,
            Encoder::<'_, $primitive>::MAX_ENCODED_LEN
        );
    };
}

encode!(u16, VarInt16);
encode!(u32, VarInt32);
encode!(u64, VarInt64);
encode!(u128, VarInt128);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;

    #[test]
    fn encode_u16() {
        let test_cases: &[(u16, bool, &[u8])] = &[
            (0, true, &[0, 0]),
            (0, false, &[0]),
            (0x7F, true, &[0x7F, 0]),
            (0x7F, false, &[0x7F]),
            (0xFF, true, &[255, 0]),
            (0xFF, false, &[255, 1]),
            (u16::MAX, true, &[0xFF, 0xFF]),
            (u16::MAX, false, &[0xFF, 0xFF, 3]),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, u16> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }

    #[test]
    fn encode_u32() {
        let test_cases: &[(u32, bool, &[u8])] = &[
            (0, true, &[0, 0, 0, 0]),
            (0, false, &[0]),
            (0x7F, true, &[0x7F, 0, 0, 0]),
            (0x7F, false, &[0x7F]),
            (0xFF, true, &[255, 0, 0, 0]),
            (0xFF, false, &[255, 1]),
            (u32::MAX, true, &[0xFF, 0xFF, 0xFF, 0xFF]),
            (u32::MAX, false, &[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, u32> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }

    #[test]
    fn encode_u64() {
        let test_cases: &[(u64, bool, &[u8])] = &[
            (0, true, &[0, 0, 0, 0, 0, 0, 0, 0]),
            (0, false, &[0]),
            (0x7F, true, &[0x7F, 0, 0, 0, 0, 0, 0, 0]),
            (0x7F, false, &[0x7F]),
            (0xFF, true, &[255, 0, 0, 0, 0, 0, 0, 0]),
            (0xFF, false, &[255, 1]),
            (
                u64::MAX,
                true,
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            ),
            (
                u64::MAX,
                false,
                &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
            ),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, u64> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }

    #[test]
    fn encode_u128() {
        let test_cases: &[(u128, bool, &[u8])] = &[
            (0, true, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            (0, false, &[0]),
            (
                0x7F,
                true,
                &[0x7F, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (0x7F, false, &[0x7F]),
            (
                0xFF,
                true,
                &[255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (0xFF, false, &[255, 1]),
            (
                u128::MAX,
                true,
                &[
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF,
                ],
            ),
            (
                u128::MAX,
                false,
                &[
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x03,
                ],
            ),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, u128> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }
}
