use crate::rust::GenRust;
use code_gen::rust::WithComments as RustWithComments;
use proto_packet::TagNumber;
use proto_packet_tree::{TypeTag, WithComments, WithTypeName};

impl GenRust {
    //! Gen Comments

    /// Generates the comment header for the `element` `packet` type for the `result` type.
    pub(in crate::rust) fn gen_comments_type_dec<E, R>(
        &self,
        result: &mut R,
        element: &E,
        packet: &str,
    ) where
        E: WithComments + WithTypeName,
        R: RustWithComments,
    {
        for comment in element.comments() {
            result.add_comment(format!(" //{}", comment));
        }
        result.add_comment(format!(" {} {} {{", packet, element.type_name()));
    }

    /// Generates the comments for the struct or message field and adds them to the `result`.
    pub(in crate::rust) fn gen_comments_field<T, C, N>(
        &self,
        result: &mut T,
        field_comments: &C,
        field_name: N,
        field_type: &TypeTag,
        tag_number: Option<TagNumber>,
    ) where
        T: RustWithComments,
        C: WithComments,
        N: AsRef<str>,
    {
        result.add_comment("    ");
        for comment in field_comments.comments() {
            result.add_comment(format!("    //{}", comment));
        }
        if let Some(tag_number) = tag_number {
            result.add_comment(format!(
                "    {}: {} = {};",
                field_name.as_ref(),
                field_type.to_string(),
                tag_number
            ))
        } else {
            result.add_comment(format!(
                "    {}: {};",
                field_name.as_ref(),
                field_type.to_string()
            ));
        }
    }
}
