/// Responsible for naming things.
#[derive(Clone, Debug)]
pub struct Naming {
    pub(in crate::rust) dot_schema: String,
    pub(in crate::rust) dot_target: String,
}

impl Default for Naming {
    fn default() -> Self {
        Self {
            dot_schema: ".pps".to_owned(),
            dot_target: ".rs".to_owned(),
        }
    }
}

impl Naming {
    //! Constants

    /// The unrecognized enum or variant case name.
    pub const UNRECOGNIZED_CASE_NAME: &'static str = "Unrecognized";
}

impl Naming {
    //! Type Names

    /// The `FieldHeader` type name.
    pub const FIELD_HEADER: &'static str = "proto_packet::io::FieldHeader";
}
