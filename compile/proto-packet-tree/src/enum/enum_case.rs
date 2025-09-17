use crate::{CaseName, CaseNameRef, WithCaseName, WithComments, WithTagNumber};
use proto_packet::io::TagNumber;

/// An enum case.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EnumCase {
    comments: Vec<String>,
    case_name: CaseName,
    tag_number: TagNumber,
}

impl EnumCase {
    //! Construction

    /// Creates a new enum case.
    pub fn new<N, T>(case_name: N, tag_number: T) -> Self
    where
        N: Into<CaseName>,
        T: Into<TagNumber>,
    {
        let case_name: CaseName = case_name.into();
        let tag_number: TagNumber = tag_number.into();
        Self {
            comments: Vec::default(),
            case_name,
            tag_number,
        }
    }
}

impl WithComments for EnumCase {
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

impl WithCaseName for EnumCase {
    fn case_name(&self) -> CaseNameRef<'_> {
        self.case_name.to_ref()
    }
}

impl WithTagNumber for EnumCase {
    fn tag_number(&self) -> TagNumber {
        self.tag_number
    }

    fn set_tag_number(&mut self, tag_number: TagNumber) {
        self.tag_number = tag_number.into();
    }
}
