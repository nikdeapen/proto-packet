/// Implements `DecodeFromReadPrefix` and `DecodeFromRead` for an enum type.
///
/// The enum must implement `From<proto_packet::io::TagNumber>`.
#[macro_export]
macro_rules! impl_enum_decode {
    ($enum_type:ty) => {
        impl enc::DecodeFromReadPrefix for $enum_type {
            fn decode_from_read_prefix_with_first_byte<R>(
                r: &mut R,
                first: u8,
            ) -> Result<Self, enc::Error>
            where
                R: std::io::Read,
            {
                let value: u32 =
                    enc::var_int::VarInt32::decode_from_read_prefix_with_first_byte(r, first)?
                        .value();
                let tag: $crate::io::TagNumber = $crate::io::TagNumber::new(value)
                    .ok_or(enc::Error::InvalidEncodedData { reason: None })?;
                Ok(<$enum_type>::from(tag))
            }
        }

        enc::impl_decode_from_read_by_prefix!($enum_type);
    };
}
