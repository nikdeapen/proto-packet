use crate::rust::GenRust;
use code_gen::rust::{ImplBlock, WithComments as RustWithComments};
use proto_packet::io::TagNumber;
use proto_packet_tree::{WithComments, WithFieldName, WithTypeName, WithTypeTag};

impl GenRust {
    //! Gen Field: Impl block

    /// Generates the impl block for the `element` and `field`.
    pub(in crate::rust) fn gen_field_impl_block<T, F>(
        &self,
        element: &T,
        field: &F,
        tag_number: Option<TagNumber>,
    ) -> ImplBlock
    where
        T: WithTypeName,
        F: WithComments + WithFieldName + WithTypeTag,
    {
        let mut block: ImplBlock = ImplBlock::from(self.naming.type_name(element));

        block.add_comment(format!(" Field: `{}`", field.field_name()));
        block.add_comment(" ");
        for comment in field.comments() {
            block.add_comment(format!(" //{}", comment));
        }
        if let Some(tag_number) = tag_number {
            block.add_comment(format!(
                " {}: {} = {};",
                field.field_name(),
                field.type_tag(),
                tag_number
            ))
        } else {
            block.add_comment(format!(" {}: {};", field.field_name(), field.type_tag()))
        }

        block
    }
}
