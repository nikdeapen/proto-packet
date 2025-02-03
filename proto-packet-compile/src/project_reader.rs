use file_storage::{FilePath, FolderPath};
use lex::{Config, ParseContext, Token};

use proto_packet_parse::{create_parse_config, parse_schema_file, ParseSchemaFileError};
use proto_packet_tree::{ModPath, Project, SchemaFile};
use proto_packet_validate::validate_schema_file;

use crate::Error;
use crate::Error::*;

/// Responsible for reading projects.
#[derive(Debug)]
pub struct ProjectReader {
    dot_schema: String,
}

impl Default for ProjectReader {
    fn default() -> Self {
        Self {
            dot_schema: ".pps".to_string(),
        }
    }
}

impl ProjectReader {
    //! Read

    /// Reads the project in the `root` folder.
    pub fn read(&self, root: &FolderPath) -> Result<Project, Error> {
        let mut project: Project = Project::default();
        for file in self.files(root)? {
            let schema_file: SchemaFile = self.read_file(&file)?;
            let mod_path: ModPath = self
                .mod_path(root, &file)
                .map_err(|error| Gen(error.to_report()))?;
            project.add_schema(mod_path, schema_file);
        }
        Ok(project)
    }
}

impl ProjectReader {
    //! Utils

    /// Gets the schema files in the `root` folder.
    fn files(&self, root: &FolderPath) -> Result<Vec<FilePath>, Error> {
        let mut files: Vec<FilePath> = Vec::default();
        for file in root
            .list_files_as_vec_unsorted()
            .map_err(|error| Storage(error))?
        {
            if file.as_str().ends_with(self.dot_schema.as_str()) {
                files.push(file);
            }
        }
        Ok(files)
    }

    /// Reads the schema file from the `file`.
    fn read_file(&self, file: &FilePath) -> Result<SchemaFile, Error> {
        let content: String = file.read_as_string()?;
        let token: Token = Token::from(content.as_str());
        let config: Config = create_parse_config();
        let context: ParseContext = ParseContext::new(token, &config);
        match parse_schema_file(context) {
            Ok(tree) => match validate_schema_file(&tree) {
                Ok(schema_file) => Ok(schema_file),
                Err(error) => Err(Validate(error.report(file.as_str(), context))),
            },
            Err(e) => {
                let token: Token = e.token();
                let error: ParseSchemaFileError = e.to_error();
                Err(Parse(error.report(file.as_str(), context, token)))
            }
        }
    }
}

impl ProjectReader {
    //! Mod Paths

    /// Gets the mod path for the `schema` file in the `root` folder.
    fn mod_path(&self, root: &FolderPath, schema: &FilePath) -> Result<ModPath, Error> {
        if !schema.as_str().starts_with(root.as_str()) {
            unimplemented!()
        } else {
            let extension: &str = &schema.as_str()[root.as_str().len()..];
            self.mod_path_from_extension(schema, extension)
        }
    }

    /// Gets the mod path for the `extension` or schema path string not including the root folder.
    fn mod_path_from_extension(
        &self,
        schema: &FilePath,
        extension: &str,
    ) -> Result<ModPath, Error> {
        if !extension.ends_with(self.dot_schema.as_str()) {
            unimplemented!()
        } else {
            let slashed: &str = &extension[..(extension.len() - self.dot_schema.as_str().len())];
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
