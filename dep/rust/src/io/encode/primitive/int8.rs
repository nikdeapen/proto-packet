use crate::io::Encoder;
use enc::{impl_encode_to_write_stack_buf, EncodeToSlice, EncodedLen, Error};

macro_rules! encode {
    ($primitive:ty) => {
        impl Encoder<'_, $primitive> {
            //! Constants

            /// The fixed encoded length. (1)
            pub const FIXED_ENCODED_LEN: usize = 1;
        }

        impl EncodedLen for Encoder<'_, $primitive> {
            fn encoded_len(&self) -> Result<usize, Error> {
                Ok(Self::FIXED_ENCODED_LEN)
            }
        }

        impl EncodeToSlice for Encoder<'_, $primitive> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                *target.get_unchecked_mut(0) = (*self.value) as u8;
                Ok(Self::FIXED_ENCODED_LEN)
            }
        }

        impl_encode_to_write_stack_buf!(
            Encoder<'_, $primitive>,
            Encoder::<'_, $primitive>::FIXED_ENCODED_LEN
        );
    };
}

encode!(u8);
encode!(i8);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_u8() {
        let test_cases: &[(u8, bool, &[u8])] = &[
            (0, true, &[0]),
            (0, false, &[0]),
            (0x7F, true, &[0x7F]),
            (0x7F, false, &[0x7F]),
            (0xFF, true, &[255]),
            (0xFF, false, &[255]),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, u8> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i8() {
        let test_cases: &[(i8, bool, &[u8])] = &[
            (0, true, &[0]),
            (0, false, &[0]),
            (0x7F, true, &[0x7F]),
            (0x7F, false, &[0x7F]),
            (i8::MIN, true, &[0x80]),
            (i8::MIN, false, &[0x80]),
            (i8::MAX, true, &[0x7F]),
            (i8::MAX, false, &[0x7F]),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, i8> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }
}
