use crate::{
    CaseName, CaseNameRef, TypeTag, WithCaseName, WithComments, WithTagNumber, WithTypeTag,
};
use proto_packet::io::TagNumber;

/// A variant case.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VariantCase {
    comments: Vec<String>,
    case_name: CaseName,
    type_tag: TypeTag,
    tag: TagNumber,
}

impl VariantCase {
    //! Construction

    /// Creates a new variant case.
    pub fn new<N, T>(case_name: N, type_tag: T, tag: TagNumber) -> Self
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
            tag,
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
    fn case_name(&self) -> CaseNameRef<'_> {
        self.case_name.to_ref()
    }
}

impl WithTypeTag for VariantCase {
    fn type_tag(&self) -> &TypeTag {
        &self.type_tag
    }
}

impl WithTagNumber for VariantCase {
    fn tag(&self) -> TagNumber {
        self.tag
    }

    fn set_tag(&mut self, tag: TagNumber) {
        self.tag = tag;
    }
}
