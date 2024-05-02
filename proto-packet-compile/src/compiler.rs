use std::collections::HashMap;
use std::fs;
use std::path::Path;

use proto_packet_gen::Generator;
use proto_packet_tree::SchemaFile;

use crate::CompilerError::{FileWriteError, GeneratorError, Other};
use crate::{CompilerError, SchemaFileReader, SchemaFileResolver};

/// Responsible for compiling ProtoPacket projects.
pub struct Compiler {
    resolver: SchemaFileResolver,
    reader: SchemaFileReader,
    generator: Box<dyn Generator>,
}

impl Compiler {
    //! Constants

    /// The rust file extension with a dot prefix.
    const DOT_RUST_EXTENSION: &'static str = ".rs";
}

impl<G: 'static + Generator> From<G> for Compiler {
    fn from(generator: G) -> Self {
        Self {
            resolver: SchemaFileResolver::default(),
            reader: SchemaFileReader::default(),
            generator: Box::new(generator),
        }
    }
}

impl Compiler {
    //! Compile

    /// Compiles the schema directory into the target directory.
    pub fn compile(&self, schema_dir: &str, target_dir: &str) -> Result<(), CompilerError> {
        let file_names: Vec<String> = self.resolver.resolve_schema_files(schema_dir)?;
        for file_name in file_names {
            let schema_file: SchemaFile = self
                .reader
                .read_schema_file(file_name.as_str())
                .map_err(|e| Other(e.to_string()))?;
            let source_files: HashMap<String, String> = self
                .generator
                .generate(&schema_file)
                .map_err(|e| GeneratorError(e))?;
            for source_file in source_files {
                let output_name: &str = source_file.0.as_str();
                let source: &str = source_file.1.as_str();
                self.write_source_file(
                    schema_dir,
                    target_dir,
                    file_name.as_str(),
                    output_name,
                    source,
                )?;
            }
        }
        Ok(())
    }

    /// Writes the source file.
    fn write_source_file(
        &self,
        schema_dir: &str,
        target_dir: &str,
        file_name: &str,
        output_name: &str,
        source: &str,
    ) -> Result<(), CompilerError> {
        let output_file_name: String =
            self.output_file_name(schema_dir, target_dir, file_name, output_name)?;
        fs::create_dir_all(Path::new(output_file_name.as_str()).parent().unwrap())
            .map_err(|e| FileWriteError(output_file_name.to_string(), e))?;
        fs::write(output_file_name.as_str(), source.as_bytes())
            .map_err(|e| FileWriteError(output_file_name.to_string(), e))
    }

    /// Gets the output file name.
    fn output_file_name(
        &self,
        schema_dir: &str,
        target_dir: &str,
        file_name: &str,
        output_name: &str,
    ) -> Result<String, CompilerError> {
        if !file_name.starts_with(schema_dir)
            || !file_name.ends_with(SchemaFileResolver::DOT_SCHEMA_EXTENSION)
        {
            return Err(Other(format!("invalid output file name: {}", output_name)));
        }
        let package_name: &str = &file_name
            [schema_dir.len()..(file_name.len() - SchemaFileResolver::DOT_SCHEMA_EXTENSION.len())];

        let mut output_file_name: String = String::with_capacity(
            target_dir.len()
                + package_name.len()
                + 1
                + output_name.len()
                + Self::DOT_RUST_EXTENSION.len(),
        );
        output_file_name.push_str(target_dir);
        output_file_name.push_str(package_name);
        output_file_name.push_str("/");
        output_file_name.push_str(output_name);
        output_file_name.push_str(Self::DOT_RUST_EXTENSION);

        Ok(output_file_name)
    }
}
