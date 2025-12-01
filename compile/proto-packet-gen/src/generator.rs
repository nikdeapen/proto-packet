use crate::{Reader, Writer};
use clerr::Report;

/// Responsible for generating code.
pub trait Generator {
    /// Generates and writes the code for the `schemas`.
    fn generate<R, W>(&self, schemas: &[R], writer: &W) -> Result<(), Report>
    where
        R: Reader,
        W: Writer;
}
