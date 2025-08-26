use file_storage::{FilePath, FolderPath};
use proto_packet_gen::rust::GenRust;
use proto_packet_gen::{CodeGenerator, GeneratedCode};
use proto_packet_link::ProjectLinker;
use proto_packet_tree::Project;

use crate::Error::Gen;
use crate::{Config, Error, Language, ProjectReader, ProjectWriter};

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
        let config: Config = self.read_config(source)?;
        println!("config: {:#?}", config);
        let project: Project = ProjectReader::from(&config).read(source)?;
        let project: Project = ProjectLinker::default().link(&project)?;
        let generated: GeneratedCode = self
            .create_generator(&config)
            .generate(&project, target)
            .map_err(|report| Gen(report))?;
        ProjectWriter::new(self.language, &config).write(&generated)
    }

    /// Creates the code generator.
    fn create_generator(&self, config: &Config) -> Box<dyn CodeGenerator> {
        match self.language {
            Language::Rust => Box::new(GenRust::new(config.gen.clone())),
        }
    }
}

impl Compiler {
    //! Config

    /// Reads the compiler config.
    fn read_config(&self, source: &FolderPath) -> Result<Config, Error> {
        let file: FilePath = source.clone_append("config.json").to_file().unwrap(); // todo -- unwrap()
        if let Some(content) = file.read_as_string_if_exists()? {
            let config: Config = serde_json::from_str(content.as_str())?;
            Ok(config)
        } else {
            Ok(serde_json::from_str("{}")?)
        }
    }
}
