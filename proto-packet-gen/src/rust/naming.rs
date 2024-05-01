use crate::GenError;

/// Responsible for naming things.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Naming {
    _nothing: (),
}

impl Naming {
    //! File Names

    /// Gets the file name for the declared type name.
    pub fn file_name<S>(&self, type_name: S) -> Result<String, GenError>
    where
        S: AsRef<str>,
    {
        Ok(self.pascal_to_snake_case(type_name.as_ref()))
    }
}

impl Naming {
    //! Field Names

    /// Gets the field name for the declared field name.
    pub fn field_name<S>(&self, declared: S) -> Result<String, GenError>
    where
        S: Into<String>,
    {
        Ok(declared.into())
    }
}

impl Naming {
    //! Type Names

    /// Gets the type name for the declared type name.
    pub fn type_name<S>(&self, declared: S) -> Result<String, GenError>
    where
        S: Into<String>,
    {
        Ok(declared.into())
    }
}

impl Naming {
    //! Utilities

    /// Converts the pascal case string to a snake case string.
    pub fn pascal_to_snake_case(&self, pascal_case: &str) -> String {
        let mut snake_case: String = String::new();
        for (i, c) in pascal_case.chars().enumerate() {
            if c.is_uppercase() && i != 0 {
                snake_case.push('_');
            }
            snake_case.extend(c.to_lowercase());
        }
        snake_case
    }
}
