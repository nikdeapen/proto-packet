use crate::Error::ReadConfig;
use crate::{Config, Error};
use file_storage::{FilePath, FolderPath};

/// Responsible for reading project configs.
#[derive(Copy, Clone, Debug, Default)]
pub struct ConfigReader {
    _nothing: (),
}

impl ConfigReader {
    //! Read Config

    /// Reads the config for the `source` project root folder.
    pub fn read_config(&self, source: &FolderPath) -> Result<Config, Error> {
        let file: FilePath = source.clone_append("config.json").to_file().unwrap(); // todo -- unwrap()
        if let Some(content) = file.read_as_string_if_exists().map_err(|e| ReadConfig(e))? {
            let config: Config =
                serde_json::from_str(content.as_str()).map_err(|e| Error::ParseConfig(e))?;
            Ok(config)
        } else {
            Ok(serde_json::from_str("{}").map_err(|e| Error::ParseConfig(e))?)
        }
    }
}
