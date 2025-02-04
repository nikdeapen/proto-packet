use proto_packet::io::TagNumber;

use crate::{CaseName, CaseNameRef, TypeTag, WithCaseName, WithComments, WithTypeTag};

/// A variant case.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VariantCase {
    comments: Vec<String>,
    case_name: CaseName,
    type_tag: TypeTag,
    tag_number: TagNumber,
}

impl VariantCase {
    //! Construction

    /// Creates a new `VariantCase`.
    pub fn new<N, T>(case_name: N, type_tag: T, tag_number: TagNumber) -> Self
    where
        N: Into<CaseName>,
        T: Into<TypeTag>,
    {
        let case_name: CaseName = case_name.into();
        let type_tag: TypeTag = type_tag.into();
        Self {
            comments: Vec::default(),
            case_name,
            type_tag,
            tag_number,
        }
    }
}

impl WithComments for VariantCase {
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

impl WithCaseName for VariantCase {
    fn case_name(&self) -> CaseNameRef {
        self.case_name.to_ref()
    }
}

impl WithTypeTag for VariantCase {
    fn type_tag(&self) -> &TypeTag {
        &self.type_tag
    }
}

impl VariantCase {
    //! Tag Number

    /// Gets the tag number.
    pub fn tag_number(&self) -> TagNumber {
        self.tag_number
    }

    /// Sets the tag number.
    pub fn set_tag_number<T>(&mut self, tag_number: T)
    where
        T: Into<TagNumber>,
    {
        self.tag_number = tag_number.into();
    }

    /// Sets the tag number.
    pub fn with_tag_number<T>(mut self, tag_number: T) -> Self
    where
        T: Into<TagNumber>,
    {
        self.set_tag_number(tag_number);
        self
    }
}
