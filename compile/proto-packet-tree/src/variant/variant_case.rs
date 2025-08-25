use proto_packet::io::TagNumber;

use crate::{
    CaseName, CaseNameRef, TypeTag, WithCaseName, WithComments, WithTagNumber, WithTypeTag,
};

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

    /// Creates a new variant case.
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
    fn case_name<'a>(&'a self) -> CaseNameRef<'a> {
        self.case_name.to_ref()
    }
}

impl WithTypeTag for VariantCase {
    fn type_tag(&self) -> &TypeTag {
        &self.type_tag
    }
}

impl WithTagNumber for VariantCase {
    fn tag_number(&self) -> TagNumber {
        self.tag_number
    }

    fn set_tag_number(&mut self, tag_number: TagNumber) {
        self.tag_number = tag_number;
    }
}
