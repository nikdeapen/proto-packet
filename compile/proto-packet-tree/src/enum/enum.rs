use crate::{
    EnumCase, TypeName, TypeNameRef, WithCaseName, WithComments, WithTagNumber, WithTypeName,
};
use proto_packet::io::TagNumber;

/// An enum.
///
/// # Invariants
/// 1. No two cases can have the same name.
/// 2. No two cases can have the same tag number.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Enum {
    comments: Vec<String>,
    enum_name: TypeName,
    cases: Vec<EnumCase>,
}

impl<N: Into<TypeName>> From<N> for Enum {
    fn from(enum_name: N) -> Self {
        let enum_name: TypeName = enum_name.into();
        Self {
            comments: Vec::default(),
            enum_name,
            cases: Vec::default(),
        }
    }
}

impl WithComments for Enum {
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

impl WithTypeName for Enum {
    fn type_name(&self) -> TypeNameRef<'_> {
        self.enum_name.to_ref()
    }
}

impl Enum {
    //! Cases

    /// Gets the cases.
    pub fn cases(&self) -> &[EnumCase] {
        self.cases.as_slice()
    }

    /// Gets the optional case with the given `case_name`.
    pub fn case_with_name<S>(&self, case_name: S) -> Option<&EnumCase>
    where
        S: AsRef<str>,
    {
        self.cases.iter().find(|c| c.case_name() == case_name)
    }

    /// Gets the optional case with the given `tag`.
    pub fn case_with_tag(&self, tag: TagNumber) -> Option<&EnumCase> {
        self.cases.iter().find(|f| f.tag() == tag)
    }

    /// Checks if the `case` can be added.
    ///
    /// Returns `true` if:
    ///     1. The case name is not yet present.
    ///     2. The tag number is not yet present.
    pub fn can_add_case(&self, case: &EnumCase) -> bool {
        self.case_with_name(case.case_name()).is_none() && self.case_with_tag(case.tag()).is_none()
    }

    /// Adds the `case`.
    ///
    /// # Safety
    /// The `case` must be able to be added.
    pub unsafe fn add_case<C>(&mut self, case: C)
    where
        C: Into<EnumCase>,
    {
        let case: EnumCase = case.into();

        debug_assert!(self.can_add_case(&case));

        self.cases.push(case);
    }

    /// Adds the `case`.
    ///
    /// # Safety
    /// The `case` must be able to be added.
    pub unsafe fn with_case<C>(mut self, case: C) -> Self
    where
        C: Into<EnumCase>,
    {
        self.add_case(case);
        self
    }
}
