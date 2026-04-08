use crate::io::Encoder;
use enc::{EncodeToSlice, EncodedLen, Error, impl_encode_to_write_stack_buf};

macro_rules! encode {
    ($primitive:ty) => {
        impl Encoder<'_, $primitive> {
            //! Constants

            /// The fixed encoded length. (1)
            const FIXED_ENCODED_LEN: usize = 1;
        }

        impl EncodedLen for Encoder<'_, $primitive> {
            fn encoded_len(&self) -> Result<usize, Error> {
                Ok(Self::FIXED_ENCODED_LEN)
            }
        }

        impl EncodeToSlice for Encoder<'_, $primitive> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                unsafe { *target.get_unchecked_mut(0) = (*self.value) as u8 };
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
        let cases: &[(u8, &[u8])] = &[(0, &[0x00]), (0x7F, &[0x7F]), (0xFF, &[0xFF])];

        for (value, expected) in cases {
            let encoder: Encoder<'_, u8> = Encoder::new(value, false);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i8() {
        let cases: &[(i8, &[u8])] = &[
            (0, &[0x00]),
            (-1, &[0xFF]),
            (1, &[0x01]),
            (i8::MIN, &[0x80]),
            (i8::MAX, &[0x7F]),
        ];

        for (value, expected) in cases {
            let encoder: Encoder<'_, i8> = Encoder::new(value, false);
            test::test_encode(&encoder, expected);
        }
    }
}
