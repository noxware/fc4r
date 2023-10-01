use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::error::Error;
use super::label::LabelLibrary;

pub const STD_CONFIG_DIR: &str = "fileclass";
pub const LABELS_DIRNAME: &str = "labels";

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Config {
    pub labels: LabelLibrary,
}

impl Config {
    // TODO: Remove file system dependency from core.
    pub fn load(dir_path: &str) -> Result<Self, Error> {
        let labels_path = Path::new(dir_path).join(LABELS_DIRNAME);
        let rd = labels_path.read_dir().map_err(|e| {
            if let std::io::ErrorKind::NotFound = e.kind() {
                Error::missing_config(format!("labels directory not found in {}", dir_path))
            } else {
                Error::invalid_config(e.to_string())
            }
        })?;

        let mut labels_content = String::new();
        for entry in rd {
            let entry = entry.map_err(|_| {
                Error::unexpected(format!(
                    "could not read the labels directory in {}",
                    dir_path
                ))
            })?;
            let path = entry.path();
            let content = fs::read_to_string(&path).map_err(|e| {
                if let std::io::ErrorKind::NotFound = e.kind() {
                    Error::missing_config(format!(
                        "label file {} disappeared before reading it",
                        path.file_name().unwrap().to_string_lossy(),
                    ))
                } else {
                    Error::invalid_config(e.to_string())
                }
            })?;
            labels_content.push_str(&content);
            labels_content.push('\n');
        }

        let labels = LabelLibrary::from_toml(&labels_content)?;
        let config = Config { labels };
        Ok(config)
    }

    pub fn std_load() -> Result<Self, Error> {
        Config::load(STD_CONFIG_DIR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_works() {
        let config = Config::load("test_dir/fileclass").unwrap();
        let labels = config.labels;

        let label_name = labels.resolve("alias");
        let label_description = labels.get_description("label");
        assert_eq!(label_name, "label");
        assert_eq!(label_description, "a label");
    }
}
