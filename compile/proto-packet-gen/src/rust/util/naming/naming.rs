/// Responsible for naming things.
#[derive(Clone, Debug)]
pub struct Naming {
    pub(in crate::rust) unrecognized_fields_name: String,
    pub(in crate::rust) tag_number_type_name: String,
    pub(in crate::rust) cursor_type_name: String,
    pub(in crate::rust) encoder_type_name: String,
    pub(in crate::rust) field_header_type_name: String,
    pub(in crate::rust) decoder_type_name: String,
}

impl Default for Naming {
    fn default() -> Self {
        Self {
            unrecognized_fields_name: "packet_unrecognized_fields".into(),
            tag_number_type_name: "proto_packet::io::TagNumber".into(),
            cursor_type_name: "std::io::Cursor".into(),
            encoder_type_name: "proto_packet::io::Encoder".into(),
            field_header_type_name: "proto_packet::io::FieldHeader".into(),
            decoder_type_name: "proto_packet::io::Decoder".into(),
        }
    }
}
