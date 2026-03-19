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
                    (&mut target[..Self::FIXED_ENCODED_LEN])
                        .copy_from_slice(&self.value.to_le_bytes());
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
        let test_cases: Vec<(u16, bool, Vec<u8>)> = [0u16, 0x7F, 0xFF, u16::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| {
                let expected: Vec<u8> = if fixed {
                    value.to_le_bytes().to_vec()
                } else {
                    VarInt16::from(value).encode_as_vec().unwrap()
                };
                (*value, fixed, expected)
            })
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, u16> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }

    #[test]
    fn encode_u32() {
        let test_cases: Vec<(u32, bool, Vec<u8>)> = [0u32, 0x7F, 0xFF, u32::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| {
                let expected: Vec<u8> = if fixed {
                    value.to_le_bytes().to_vec()
                } else {
                    VarInt32::from(value).encode_as_vec().unwrap()
                };
                (*value, fixed, expected)
            })
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, u32> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }

    #[test]
    fn encode_u64() {
        let test_cases: Vec<(u64, bool, Vec<u8>)> = [0u64, 0x7F, 0xFF, u64::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| {
                let expected: Vec<u8> = if fixed {
                    value.to_le_bytes().to_vec()
                } else {
                    VarInt64::from(value).encode_as_vec().unwrap()
                };
                (*value, fixed, expected)
            })
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, u64> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }

    #[test]
    fn encode_u128() {
        let test_cases: Vec<(u128, bool, Vec<u8>)> = [0u128, 0x7F, 0xFF, u128::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| {
                let expected: Vec<u8> = if fixed {
                    value.to_le_bytes().to_vec()
                } else {
                    VarInt128::from(value).encode_as_vec().unwrap()
                };
                (*value, fixed, expected)
            })
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, u128> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }

    #[test]
    fn encode_i16() {
        let test_cases: Vec<(i16, bool, Vec<u8>)> = [0i16, -1, 1, 0x7F, 0xFF, i16::MIN, i16::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| {
                let expected: Vec<u8> = if fixed {
                    value.to_le_bytes().to_vec()
                } else {
                    VarInt16::from_zigzag(*value).encode_as_vec().unwrap()
                };
                (*value, fixed, expected)
            })
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, i16> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }

    #[test]
    fn encode_i32() {
        let test_cases: Vec<(i32, bool, Vec<u8>)> = [0i32, -1, 1, 0x7F, 0xFF, i32::MIN, i32::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| {
                let expected: Vec<u8> = if fixed {
                    value.to_le_bytes().to_vec()
                } else {
                    VarInt32::from_zigzag(*value).encode_as_vec().unwrap()
                };
                (*value, fixed, expected)
            })
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, i32> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }

    #[test]
    fn encode_i64() {
        let test_cases: Vec<(i64, bool, Vec<u8>)> = [0i64, -1, 1, 0x7F, 0xFF, i64::MIN, i64::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| {
                let expected: Vec<u8> = if fixed {
                    value.to_le_bytes().to_vec()
                } else {
                    VarInt64::from_zigzag(*value).encode_as_vec().unwrap()
                };
                (*value, fixed, expected)
            })
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, i64> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }

    #[test]
    fn encode_i128() {
        let test_cases: Vec<(i128, bool, Vec<u8>)> =
            [0i128, -1, 1, 0x7F, 0xFF, i128::MIN, i128::MAX]
                .iter()
                .flat_map(|value| [(value, true), (value, false)])
                .map(|(value, fixed)| {
                    let expected: Vec<u8> = if fixed {
                        value.to_le_bytes().to_vec()
                    } else {
                        VarInt128::from_zigzag(*value).encode_as_vec().unwrap()
                    };
                    (*value, fixed, expected)
                })
                .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, i128> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected.as_slice());
        }
    }
}
