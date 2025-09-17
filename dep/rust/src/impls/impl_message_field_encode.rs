#[macro_export]
macro_rules! impl_message_field_encoded_len {
    ($field:expr, $fixed:literal, $tag_number:literal, $wire_type:expr, $encoded_len:ident) => {
        if let Some(value) = $field {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked($tag_number) };
            $encoded_len += FieldHeader::new($wire_type, tag_number).encoded_len()?;
            $encoded_len += Encoder::new(value, $fixed).encoded_len()?;
        }
    };
}

#[macro_export]
macro_rules! impl_message_field_encode_to_slice_unchecked {
    ($field:expr, $fixed:literal, $tag_number:literal, $wire_type:expr, $encoded_len:ident, $target:expr) => {
        if let Some(value) = $field {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked($tag_number) };
            $encoded_len += FieldHeader::new($wire_type, tag_number).encode_to_slice($target)?;
            $encoded_len += Encoder::new(value, $fixed).encode_to_slice($target)?;
        }
    };
}

#[macro_export]
macro_rules! impl_message_field_encode_to_write {
    ($field:expr, $fixed:literal, $tag_number:literal, $wire_type:expr, $encoded_len:ident, $write:expr) => {
        if let Some(value) = $field {
            let tag_number: TagNumber = unsafe { TagNumber::new_unchecked($tag_number) };
            $encoded_len += FieldHeader::new($wire_type, tag_number).encode_to_write($write)?;
            $encoded_len += Encoder::new(value, $fixed).encode_to_write($write)?;
        }
    };
}
