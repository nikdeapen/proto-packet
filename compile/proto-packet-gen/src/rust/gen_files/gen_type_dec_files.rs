use crate::rust::{GenRust, ModTree};
use crate::{Reader, Writer};
use clerr::Report;
use code_gen::Source;
use proto_packet_tree::{ModPath, TypeDec, WithTypeName};

impl GenRust {
    //! Gen Files: Type Declarations

    /// Generates the type declaration files.
    pub(in crate::rust) fn gen_type_dec_files<R, W>(
        &self,
        schemas: &[R],
        writer: &W,
    ) -> Result<ModTree, Report>
    where
        R: Reader,
        W: Writer,
    {
        todo!()
    }
}
