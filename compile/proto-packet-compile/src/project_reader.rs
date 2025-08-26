use crate::Error::*;
use crate::{Config, Error};
use file_storage::{FilePath, FolderPath};
use lex::{Config as LexConfig, ParseContext, Token};
use proto_packet_parse::{parse_schema_file, ParseSchemaFileError};
use proto_packet_tree::{ModPath, Project, SchemaFile};
use proto_packet_validate::validate_schema_file;

/// Responsible for reading projects.
#[derive(Debug)]
pub struct ProjectReader<'a> {
    config: &'a Config,
}

impl<'a> From<&'a Config> for ProjectReader<'a> {
    fn from(config: &'a Config) -> Self {
        Self { config }
    }
}

impl<'a> ProjectReader<'a> {
    //! Read

    /// Reads the project in the `source` folder.
    pub fn read(&self, source: &FolderPath) -> Result<Project, Error> {
        let mut project: Project = Project::default();
        for file in self.files(source)? {
            let schema_file: SchemaFile = self.read_file(&file)?;
            let mod_path: ModPath = self
                .mod_path(source, &file)
                .map_err(|error| Gen(error.report()))?;
            unsafe { project.add_schema_file(mod_path, schema_file) }
        }
        Ok(project)
    }

    /// Gets the schema files in the `source` folder.
    fn files(&self, source: &FolderPath) -> Result<Vec<FilePath>, Error> {
        Ok(source
            .list_files_as_vec_unsorted()
            .map_err(|error| Storage(error))?
            .drain(..)
            .filter(|file| file.as_str().ends_with(self.config.dot_schema.as_str()))
            .collect())
    }

    /// Reads the schema file from the `file`.
    fn read_file(&self, file: &FilePath) -> Result<SchemaFile, Error> {
        let content: String = file.read_as_string()?;
        let token: Token = Token::from(content.as_str());
        let config: LexConfig = proto_packet_parse::config();
        let context: ParseContext = ParseContext::new(token, &config);
        match parse_schema_file(context) {
            Ok(tree) => match validate_schema_file(&tree) {
                Ok(schema_file) => Ok(schema_file),
                Err(error) => Err(Validate(error.to_report(file.as_str(), context))),
            },
            Err(e) => {
                let token: Token = e.token();
                let error: ParseSchemaFileError = e.to_error();
                Err(Parse(error.to_report(file.as_str(), context, token)))
            }
        }
    }
}

impl<'a> ProjectReader<'a> {
    //! Mod Paths

    /// Gets the mod path for the `schema` file in the `source` folder.
    fn mod_path(&self, source: &FolderPath, schema: &FilePath) -> Result<ModPath, Error> {
        if !schema.as_str().starts_with(source.as_str()) {
            unimplemented!()
        } else {
            let extension: &str = &schema.as_str()[source.as_str().len()..];
            self.mod_path_from_extension(schema, extension)
        }
    }

    /// Gets the mod path for the `extension` or schema path string not including the source folder.
    fn mod_path_from_extension(
        &self,
        schema: &FilePath,
        extension: &str,
    ) -> Result<ModPath, Error> {
        if !extension.ends_with(self.config.dot_schema.as_str()) {
            unimplemented!()
        } else {
            let slashed: &str =
                &extension[..(extension.len() - self.config.dot_schema.as_str().len())];
            self.mod_path_from_slashed(schema, slashed)
        }
    }

    /// Gets the mod path for the slashed schema file `extension`.
    fn mod_path_from_slashed(&self, schema: &FilePath, extension: &str) -> Result<ModPath, Error> {
        let fs: char = schema.path().file_separator();
        let mut buffer: [u8; 4] = [0u8; 4];
        let slash: &str = fs.encode_utf8(&mut buffer);
        let dotted: String = extension.replace(slash, ".");
        match ModPath::new(dotted.as_str()) {
            Ok(mod_path) => Ok(mod_path),
            Err(_error) => unimplemented!(),
        }
    }
}
