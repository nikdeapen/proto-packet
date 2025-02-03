use clerr::Report;
use file_storage::FolderPath;

use proto_packet_tree::Project;

use crate::rust::{Naming, Typing};
use crate::{Generated, Generator};

/// Responsible for generating code for Rust projects.
#[derive(Debug)]
#[allow(dead_code)]
pub struct GenRust {
    pub(crate) naming: Naming,
    pub(crate) typing: Typing,
}

impl Default for GenRust {
    fn default() -> Self {
        Self {
            naming: Naming::default(),
            typing: Typing::default(),
        }
    }
}

impl Generator for GenRust {
    fn generate(&self, project: &Project, target: &FolderPath) -> Result<Generated, Report> {
        let mut generated: Generated = Generated::from(target.clone());

        self.gen_type_dec_files(project, &mut generated)
            .map_err(|e| e.to_report())?;

        Ok(generated)
    }
}
