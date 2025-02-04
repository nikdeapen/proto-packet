use std::str::Chars;

use proto_packet_tree::{CaseNameRef, FieldNameRef, ModName, TypeNameRef};

use crate::rust::{Error, FileNaming};

/// Responsible for naming things.
#[derive(Clone, Debug, Default)]
pub struct Naming {
    pub(in crate::rust) file_naming: FileNaming,
}

impl Naming {
    //! Field Names

    /// Gets the field name for the declared `field_name`.
    pub fn field_name(&self, field_name: FieldNameRef) -> String {
        field_name.to_string()
    }
}

impl Naming {
    //! Case Names

    /// Gets the case name for the declared `case_name`.
    pub fn case_name(&self, case_name: CaseNameRef) -> String {
        case_name.to_string()
    }
}

impl Naming {
    //! Type Names

    /// Gets the type name for the declared `type_name`.
    pub fn type_name(&self, type_name: TypeNameRef) -> String {
        type_name.to_string()
    }
}

impl Naming {
    //! Mod Names

    /// Converts the `type_name` into a `mod_name`.
    pub fn mod_name_for_type_name(&self, type_name: TypeNameRef) -> Result<ModName, Error> {
        let mod_name: String = self.pascal_to_snake_case(type_name.as_ref());
        ModName::try_from(mod_name).map_err(|(mod_name, error_message)| {
            Error::TypeNameNotConvertableToModName {
                type_name: type_name.to_owned(),
                mod_name,
                error_message,
            }
        })
    }
}

impl Naming {
    //! Utilities

    /// Converts the `pascal_case` string to a snake case string.
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

    /// Converts the `snake_case` string to a pascal case string.
    pub fn snake_to_pascal_case(&self, snake_case: &str) -> String {
        let mut pascal_case: String = String::with_capacity(snake_case.len());
        for segment in snake_case.split("_") {
            let mut chars: Chars = segment.chars();
            if let Some(first) = chars.next() {
                pascal_case.push(first.to_ascii_uppercase());
                pascal_case.push_str(&segment[first.len_utf8()..]);
            }
        }
        pascal_case
    }
}
