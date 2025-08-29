use crate::io::encode::list::util::*;
use crate::io::Encoder;
use crate::io::WireType;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error, StreamError};
use std::io::Write;
use uuid::Uuid;

macro_rules! impl_encoder_generic_list {
    ($generic_type:ty, $wire:expr) => {
        impl EncodedLen for Encoder<'_, Vec<$generic_type>> {
            fn encoded_len(&self) -> Result<usize, Error> {
                encoded_len(self.value, $wire, |value| {
                    Encoder::new(value, self.fixed).encoded_len()
                })
            }
        }

        impl EncodeToSlice for Encoder<'_, Vec<$generic_type>> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                encode_to_slice(
                    self.value,
                    $wire,
                    target,
                    |value, target| Encoder::new(value, self.fixed).encode_to_slice(target),
                    |value| Encoder::new(value, self.fixed).encoded_len(),
                )
            }
        }

        impl EncodeToWrite for Encoder<'_, Vec<$generic_type>> {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
            where
                W: Write,
            {
                encode_to_write(
                    self.value,
                    $wire,
                    w,
                    |value, w| Encoder::new(value, self.fixed).encode_to_write(w),
                    |value| Encoder::new(value, self.fixed).encoded_len(),
                )
            }
        }
    };
}

impl_encoder_generic_list!(Uuid, WireType::Fixed16Byte);
impl_encoder_generic_list!(String, WireType::LengthPrefixed);
