use proto_packet::io::TagNumber;

use crate::{
    MessageField, TypeName, TypeNameRef, WithComments, WithFieldName, WithTagNumberOptional,
    WithTypeName,
};

/// A message.
///
/// # Invariants
/// 1. No two fields can have the same name.
/// 2. No two fields can have the same tag number.
/// 3. All fields without a tag number must come before fields with tag numbers.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Message {
    comments: Vec<String>,
    message_name: TypeName,
    fields: Vec<MessageField>,
}

impl<N: Into<TypeName>> From<N> for Message {
    fn from(message_name: N) -> Self {
        let message_name: TypeName = message_name.into();
        Self {
            comments: Vec::default(),
            message_name,
            fields: Vec::default(),
        }
    }
}

impl WithComments for Message {
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

impl WithTypeName for Message {
    fn type_name(&self) -> TypeNameRef {
        self.message_name.to_ref()
    }
}

impl Message {
    //! Fields

    /// Gets the fields.
    pub fn fields(&self) -> &[MessageField] {
        self.fields.as_slice()
    }

    /// Gets the optional field with the given `field_name`.
    pub fn field_with_name<S>(&self, field_name: S) -> Option<&MessageField>
    where
        S: AsRef<str>,
    {
        self.fields
            .iter()
            .filter(|f| f.field_name() == field_name)
            .next()
    }

    /// Gets the optional field with the given `tag_number`.
    pub fn field_with_number(&self, tag_number: TagNumber) -> Option<&MessageField> {
        self.fields
            .iter()
            .filter(|f| f.tag_number() == Some(tag_number))
            .next()
    }

    /// Checks if the `field` can be added.
    ///
    /// Returns `true` if:
    ///     1. The field name is not already present.
    ///     2. The tag number is not already present.
    ///     3. If the tag number is `None` all the current tag numbers are also `None`.
    pub fn can_add_field(&self, field: &MessageField) -> bool {
        if self.field_with_name(field.field_name()).is_some() {
            false
        } else if let Some(tag_number) = field.tag_number() {
            self.field_with_number(tag_number).is_none()
        } else {
            self.fields.iter().all(|f| f.tag_number() == None)
        }
    }

    /// Adds the `field`.
    ///
    /// # Unsafe
    /// The `field` must be able to be added.
    pub unsafe fn add_field<F>(&mut self, field: F)
    where
        F: Into<MessageField>,
    {
        let field: MessageField = field.into();

        debug_assert!(self.can_add_field(&field));

        self.fields.push(field.into());
    }

    /// Adds the `field`.
    ///
    /// # Unsafe
    /// The `field` must be able to be added.
    pub unsafe fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<MessageField>,
    {
        self.add_field(field);
        self
    }
}
