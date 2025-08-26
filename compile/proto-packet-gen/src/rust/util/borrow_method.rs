/// A reference method.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BorrowMethod {
    Copy,
    Ref,
    Deref,
}

impl BorrowMethod {
    //! Field Expression

    /// Generates the field reference expression.
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
