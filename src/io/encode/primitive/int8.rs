use crate::io::Encoder;
use enc::{EncodeToSlice, EncodedLen, Error, impl_encode_to_write_stack_buf};

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
        let test_cases: Vec<(u8, bool, Vec<u8>)> = [0u8, 0x7F, 0xFF]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| (*value, fixed, vec![*value]))
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, u8> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i8() {
        let test_cases: Vec<(i8, bool, Vec<u8>)> = [0i8, -1, 1, i8::MIN, i8::MAX]
            .iter()
            .flat_map(|value| [(value, true), (value, false)])
            .map(|(value, fixed)| (*value, fixed, vec![*value as u8]))
            .collect();

        for (value, fixed, expected) in &test_cases {
            let encoder: Encoder<'_, i8> = Encoder::new(value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }
}
