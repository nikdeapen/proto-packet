use crate::{
    TypeName, TypeNameRef, VariantCase, WithCaseName, WithComments, WithTagNumber, WithTypeName,
};
use proto_packet::io::TagNumber;

/// A variant.
///
/// # Invariants
/// 1. No two cases can have the same name.
/// 2. No two cases can have the same tag number.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Variant {
    comments: Vec<String>,
    variant_name: TypeName,
    cases: Vec<VariantCase>,
}

impl<N: Into<TypeName>> From<N> for Variant {
    fn from(variant_name: N) -> Self {
        let variant_name: TypeName = variant_name.into();
        Self {
            comments: Vec::default(),
            variant_name,
            cases: Vec::default(),
        }
    }
}

impl WithComments for Variant {
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

impl WithTypeName for Variant {
    fn type_name(&self) -> TypeNameRef<'_> {
        self.variant_name.to_ref()
    }
}

impl Variant {
    //! Cases

    /// Gets the cases.
    pub fn cases(&self) -> &[VariantCase] {
        self.cases.as_slice()
    }

    /// Gets the optional case with the given `case_name`.
    pub fn case_with_name<S>(&self, case_name: S) -> Option<&VariantCase>
    where
        S: AsRef<str>,
    {
        self.cases
            .iter()
            .filter(|f| f.case_name() == case_name)
            .next()
    }

    /// Gets the optional case with the given `tag_number`.
    pub fn case_with_number(&self, tag_number: TagNumber) -> Option<&VariantCase> {
        self.cases
            .iter()
            .filter(|f| f.tag_number() == tag_number)
            .next()
    }

    /// Checks if the `case` can be added.
    ///
    /// Returns `true` if:
    ///     1. The case name is not yet present.
    ///     2. The tag number is not yet present.
    pub fn can_add_case(&self, case: &VariantCase) -> bool {
        self.case_with_name(case.case_name()).is_none()
            && self.case_with_number(case.tag_number()).is_none()
    }

    /// Adds the `case`.
    ///
    /// # Unsafe
    /// The `case` must be able to be added.
    pub unsafe fn add_case<F>(&mut self, case: F)
    where
        F: Into<VariantCase>,
    {
        let case: VariantCase = case.into();

        debug_assert!(self.can_add_case(&case));

        self.cases.push(case.into());
    }

    /// Adds the `case`.
    ///
    /// # Unsafe
    /// The `case` must be able to be added.
    pub unsafe fn with_case<F>(mut self, case: F) -> Self
    where
        F: Into<VariantCase>,
    {
        self.add_case(case);
        self
    }
}
