use crate::rust::{GenRust, ModTree};
use crate::{Reader, Writer};
use clerr::Report;

impl GenRust {
    //! Gen Files: Type Declarations

    /// Generates the type declaration files.
    pub(in crate::rust) fn gen_type_dec_files<R, W>(
        &self,
        _schemas: &[R],
        _writer: &W,
    ) -> Result<ModTree, Report>
    where
        R: Reader,
        W: Writer,
    {
        todo!()
    }
}
