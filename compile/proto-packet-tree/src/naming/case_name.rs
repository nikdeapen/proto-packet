use custom_string::custom_string;

use crate::naming::validate_name::validate_name;

custom_string!(
    #[doc = "The name of an enum or variant case."],
    CaseName,
    CaseNameRef,
    |s| validate_case_name(s)
);

/// Validates the `case_name`.
pub fn validate_case_name(case_name: &str) -> Result<(), &'static str> {
    validate_name(case_name)?;

    if !case_name.as_bytes()[0].is_ascii_uppercase() {
        Err("case names must start with an uppercase letter")
    } else {
        Ok(())
    }
}

/// An element with a case name.
pub trait WithCaseName {
    /// Gets the case name.
    fn case_name(&self) -> CaseNameRef<'_>;
}
