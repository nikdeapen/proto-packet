use crate::{
    FieldName, FieldNameRef, TypeTag, WithComments, WithFieldName, WithTagNumber, WithTypeTag,
};
use proto_packet::io::TagNumber;

/// A message field.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MessageField {
    comments: Vec<String>,
    field_name: FieldName,
    type_tag: TypeTag,
    tag: TagNumber,
}

impl MessageField {
    //! Construction

    /// Creates a new message field.
    pub fn new<N, T>(field_name: N, type_tag: T, tag: TagNumber) -> Self
    where
        N: Into<FieldName>,
        T: Into<TypeTag>,
    {
        let field_name: FieldName = field_name.into();
        let type_tag: TypeTag = type_tag.into();
        Self {
            comments: Vec::default(),
            field_name,
            type_tag,
            tag,
        }
    }
}

impl WithComments for MessageField {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithFieldName for MessageField {
    fn field_name(&self) -> FieldNameRef<'_> {
        self.field_name.to_ref()
    }
}

impl WithTypeTag for MessageField {
    fn type_tag(&self) -> &TypeTag {
        &self.type_tag
    }
}

impl WithTagNumber for MessageField {
    fn tag(&self) -> TagNumber {
        self.tag
    }

    fn set_tag(&mut self, tag: TagNumber) {
        self.tag = tag;
    }
}
