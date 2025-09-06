use proto_packet_tree::{CaseNameRef, FieldNameRef, TypeNameRef};
use std::str::Chars;

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

impl Naming {
    //! Field Names

    /// Gets the rust field name for the `field_name`.
    pub fn field_name(&self, field_name: FieldNameRef) -> String {
        field_name.to_string()
    }
}

impl Naming {
    //! Case Names

    /// Gets the rust case name for the `case_name`.
    pub fn case_name(&self, case_name: CaseNameRef) -> String {
        case_name.to_string()
    }
}

impl Naming {
    //! Type Names

    /// Gets the rust type name for the `type_name`.
    pub fn type_name(&self, type_name: TypeNameRef) -> String {
        type_name.to_string()
    }
}

impl Naming {
    //! Mod Names

    /// Gets the mod name for the declared `type_name`.
    pub fn mod_name_for_type_name(&self, type_name: TypeNameRef) -> String {
        self.pascal_to_snake_case(type_name.as_ref())
    }
}

impl Naming {
    //! Utilities

    /// Converts the `pascal_case` string to a snake case string.
    pub fn pascal_to_snake_case(&self, pascal_case: &str) -> String {
        let mut snake_case: String = String::new();
        for (i, c) in pascal_case.chars().enumerate() {
            if c.is_uppercase() && i != 0 {
                snake_case.push('_');
            }
            snake_case.extend(c.to_lowercase());
        }
        snake_case
    }

    /// Converts the `snake_case` string to a pascal case string.
    pub fn snake_to_pascal_case(&self, snake_case: &str) -> String {
        let mut pascal_case: String = String::with_capacity(snake_case.len());
        for segment in snake_case.split("_") {
            let mut chars: Chars = segment.chars();
            if let Some(first) = chars.next() {
                pascal_case.push(first.to_ascii_uppercase());
                pascal_case.push_str(&segment[first.len_utf8()..]);
            }
        }
        pascal_case
    }
}
