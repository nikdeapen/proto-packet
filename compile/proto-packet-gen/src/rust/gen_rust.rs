use crate::config::GenConfig;
use crate::rust::{ModTree, Naming, Typing};
use crate::{Generator, Reader, Writer};
use clerr::Report;

/// Responsible for generating rust code.
pub struct GenRust {
    pub(in crate::rust) config: GenConfig,
    pub(in crate::rust) naming: Naming,
    pub(in crate::rust) typing: Typing,
}

impl From<GenConfig> for GenRust {
    fn from(config: GenConfig) -> Self {
        Self {
            config,
            naming: Naming::default(),
            typing: Typing::default(),
        }
    }
}

impl Generator for GenRust {
    fn generate<R, W>(&self, schemas: &[R], writer: &W) -> Result<(), Report>
    where
        R: Reader,
        W: Writer,
    {
        let tree: ModTree = self.gen_type_dec_files(schemas, writer)?;
        self.gen_mod_files(&tree, writer)
    }
}
