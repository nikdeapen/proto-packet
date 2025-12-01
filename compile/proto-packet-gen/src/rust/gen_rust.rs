use crate::rust::Naming;
use crate::{Generator, Reader, Writer};
use clerr::Report;

/// Responsible for generating rust code.
#[derive(Default)]
pub struct GenRust {
    pub(in crate::rust) naming: Naming,
}

impl Generator for GenRust {
    fn generate<R, W>(&self, schemas: &[R], writer: &W) -> Result<(), Report>
    where
        R: Reader,
        W: Writer,
    {
        todo!()
    }
}
