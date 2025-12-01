use code_gen::rust::RustType;
use code_gen::{Source, WithStatements};

/// A borrow method.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BorrowMethod {
    Copy,
    Ref,
    Deref,
}

impl BorrowMethod {
    //! Field Expression

    /// Generates the field borrow expression.
    pub fn gen_field_exp<S>(&self, field_name: S, optional: bool) -> String
    where
        S: AsRef<str>,
    {
        match self {
            Self::Copy => format!("self.{}", field_name.as_ref()),
            Self::Ref => {
                if optional {
                    format!("self.{}.as_ref()", field_name.as_ref())
                } else {
                    format!("&self.{}", field_name.as_ref())
                }
            }
            Self::Deref => {
                if optional {
                    format!("self.{}.as_deref()", field_name.as_ref())
                } else {
                    format!("self.{}.as_ref()", field_name.as_ref())
                }
            }
        }
    }
}

impl BorrowMethod {
    //! Swap Source

    /// Generates the source code to set the field.
    pub fn set_source(&self, field_name: &str, field_type: &RustType) -> Source {
        let mut s: Source = Source::default();
        match self {
            Self::Copy => {
                s.add_semi(format!(
                    "let old_{}: {} = self.{}",
                    field_name, field_type, field_name
                ));
                s.add_semi(format!("self.{} = {}.into()", field_name, field_name));
                s.add_literal(format!("old_{}", field_name));
            }
            _ => s.add_literal(format!(
                "std::mem::replace(&mut self.{}, {}.into())",
                field_name, field_name,
            )),
        }
        s
    }
}
