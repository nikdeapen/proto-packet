use crate::Error::Storage;
use crate::Language::Rust;
use crate::{Config, Error, Language};
use clerr::{Code, Report};
use colored::Colorize;
use file_storage::{FilePath, FolderPath};
use proto_packet_gen::GeneratedCode;
use proto_packet_tree::ModPathRef;

/// Responsible for writing generated code.
#[derive(Clone, Debug)]
pub struct ProjectWriter<'a> {
    language: Language,
    config: &'a Config,
}

impl<'a> ProjectWriter<'a> {
    //! Construction

    /// Creates a new project writer.
    pub fn new(language: Language, config: &'a Config) -> Self {
        Self { language, config }
    }
}

impl<'a> ProjectWriter<'a> {
    //! Write

    /// Writes the `generated` source code.
    pub fn write(&self, generated: &GeneratedCode) -> Result<(), Error> {
        self.clean(generated.target())?;

        if let Some(error) = generated
            .sources()
            .iter()
            .flat_map(|(file, code)| self.write_file(file, code).err())
            .next()
        {
            Err(error)
        } else {
            Ok(())
        }
    }

    /// Writes the source `code` to the `file`.
    fn write_file(&self, file: &FilePath, code: &str) -> Result<(), Error> {
        file.write_str(code).map_err(|error| Storage(error))
    }
}

impl<'a> ProjectWriter<'a> {
    //! Clean

    /// Cleans the `target` folder.
    fn clean(&self, target: &FolderPath) -> Result<(), Error> {
        let retain: Vec<FolderPath> = self.retain(target)?;
        'outer: for file in target.list_files_as_vec()? {
            for retain in &retain {
                if file.as_str().starts_with(retain.as_str()) {
                    println!("checking: {}", file);
                    println!("skipping: {}", file);
                    continue 'outer;
                }
            }
            file.delete()?;
        }
        Ok(())
    }

    /// Gets the folders to retain when cleaning the `target` folder.
    fn retain(&self, target: &FolderPath) -> Result<Vec<FolderPath>, Error> {
        let fs: char = target.path().file_separator();
        let mut retain: Vec<FolderPath> = Vec::default();
        if self.language == Rust {
            for module in &self.config.gen.rust.mods.retain {
                let module: ModPathRef = ModPathRef::new(module.as_str()).map_err(|_| {
                    Error::Other({
                        let code: Code = Code::error(
                            "CONFIG",
                            "invalid rust mod path in config.gen.rust.mods.retain",
                        );
                        Report::new(code).with_entry(vec![module.normal()])
                    })
                })?;
                let extension: String = module
                    .value()
                    .chars()
                    .map(|c| if c == '.' { fs } else { c })
                    .collect();
                let target: FolderPath = target
                    .clone_with_extra_capacity(fs.len_utf8() + extension.len())
                    .with_appended(extension)
                    .with_appended_char(fs)
                    .to_folder()
                    .unwrap();
                retain.push(target)
            }
        }
        Ok(retain)
    }
}
