#[macro_export]
macro_rules! impl_decode_enum {
    ($name:ident) => {
        enc::impl_decode_from_read_by_prefix!($name);

        impl enc::DecodeFromReadPrefix for $name {
            fn decode_from_read_prefix_with_first_byte<R>(
                r: &mut R,
                first: u8,
            ) -> Result<Self, enc::Error>
            where
                R: std::io::Read,
            {
                use enc::var_int::VarInt32;
                use enc::DecodeFromReadPrefix;

                let tag: VarInt32 = VarInt32::decode_from_read_prefix_with_first_byte(r, first)?;
                let tag: u32 = tag.value();
                if let Some(tag) = $crate::io::TagNumber::new(tag) {
                    Ok(Self::from(tag))
                } else {
                    Err(enc::Error::InvalidEncodedData {
                        reason: Some(Box::new($crate::io::DecodingError::InvalidTagNumber(tag))),
                    })
                }
            }
        }
    };
}
