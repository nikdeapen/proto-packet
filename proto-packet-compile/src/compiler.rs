use file_storage::FolderPath;

use proto_packet_gen::{Generated, Generator};
use proto_packet_link::ProjectLinker;
use proto_packet_tree::Project;

use crate::Error::{Gen, Storage};
use crate::{Error, ProjectReader, ProjectWriter};

/// Responsible for compiling projects.
pub struct Compiler {
    reader: ProjectReader,
    generator: Box<dyn Generator>,
    writer: ProjectWriter,
}

impl From<Box<dyn Generator>> for Compiler {
    fn from(generator: Box<dyn Generator>) -> Self {
        Self {
            reader: ProjectReader::default(),
            generator,
            writer: ProjectWriter::default(),
        }
    }
}

impl Compiler {
    //! Compile

    /// Compiles the project in the `source` folder into the `target` folder.
    pub fn compile(&self, source: &FolderPath, target: &FolderPath) -> Result<(), Error> {
        target.delete_files().map_err(|error| Storage(error))?;
        let project: Project = self.reader.read(source)?;
        let project: Project = ProjectLinker::default().link(&project)?;
        let generated: Generated = self
            .generator
            .generate(&project, target)
            .map_err(|report| Gen(report))?;
        self.writer.write(&generated)
    }
}
