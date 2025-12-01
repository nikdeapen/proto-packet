use crate::config::ConfigReader;
use crate::Error::{Gen, ReadProject};
use crate::{Config, Error, Language, ProjectReader, SchemaReader, SourceWriter};
use clerr::Report;
use file_storage::FolderPath;
use proto_packet_gen::config::GenConfig;
use proto_packet_gen::rust::GenRust;
use proto_packet_gen::Generator;

/// Responsible for compiling projects.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Compiler {
    language: Language,
}

impl From<Language> for Compiler {
    fn from(language: Language) -> Self {
        Self { language }
    }
}

impl Compiler {
    //! Compile

    /// Compiles the project in the `source` folder into the `target` folder.
    pub fn compile(&self, source: &FolderPath, target: &FolderPath) -> Result<(), Error> {
        target
            .list_files_as_vec_unsorted()
            .map_err(|e| ReadProject(e))? // todo -- wrong error?
            .iter()
            .for_each(|f| f.delete().unwrap());

        let config: Config = ConfigReader::default().read_config(source)?;
        let reader: ProjectReader = ProjectReader::from(&config);
        let writer: SourceWriter = SourceWriter::from(target.clone());
        let schemas: Vec<SchemaReader> = reader.read(source)?;

        let config: GenConfig = GenConfig::default(); // todo
        let result: Result<(), Report> = match self.language {
            Language::Rust => GenRust::from(config).generate(schemas.as_slice(), &writer),
        };

        result.map_err(|e| Gen(e))
    }
}
