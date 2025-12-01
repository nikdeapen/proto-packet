use crate::rust::GenRust;
use code_gen::rust::WithDerives;

impl GenRust {
    //! Gen Derives

    /// Generates the derives for the `type_dec`.
    pub(in crate::rust) fn gen_derives_type_dec<T>(
        &self,
        type_dec: &mut T,
        is_copy: bool,
        is_default: bool,
    ) where
        T: WithDerives,
    {
        if is_copy {
            type_dec.add_derive("Copy");
        }
        type_dec.add_derive("Clone");
        type_dec.add_derive("Ord");
        type_dec.add_derive("PartialOrd");
        type_dec.add_derive("Eq");
        type_dec.add_derive("PartialEq");
        type_dec.add_derive("Hash");
        type_dec.add_derive("Debug");
        if is_default {
            type_dec.add_derive("Default");
        }
        type_dec.add_derive("serde::Serialize");
        type_dec.add_derive("serde::Deserialize");
    }
}
