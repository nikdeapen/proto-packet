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
                    $var_int::from_zig_zag(*self.value).encoded_len()
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
                    $var_int::from_zig_zag(*self.value).encode_to_slice_unchecked(target)
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

encode!(i16, VarInt16);
encode!(i32, VarInt32);
encode!(i64, VarInt64);
encode!(i128, VarInt128);
