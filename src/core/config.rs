use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use super::error::Error;
use super::label::LabelLibrary;

pub const STD_CONFIG_DIR: &str = "fileclass";
pub const LABELS_FILENAME: &str = "labels.toml";
pub const SETTINGS_FILENAME: &str = "settings.toml";

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Config {
    pub labels: LabelLibrary,
    pub settings: Settings,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Settings {
    // TODO: Use a default if missing and use that default in fcinit.
    pub link_dir: String,
}

impl Config {
    // TODO: Remove file system dependency from core.
    pub fn load(dir_path: &str) -> Result<Self, Error> {
        let labels_path = Path::new(dir_path).join(LABELS_FILENAME);
        let labels_content = match fs::read_to_string(labels_path) {
            Ok(content) => content,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(Error::missing_config(format!(
                        "labels file not found in {}",
                        dir_path
                    )))
                }
                _ => return Err(Error::invalid_config(e.to_string())),
            },
        };
        let labels = LabelLibrary::from_toml(&labels_content)?;

        // These settings will be removed in the near future. So let's just unwrap.
        let settings_path = Path::new(dir_path).join(SETTINGS_FILENAME);
        let settings_content = fs::read_to_string(settings_path).unwrap();
        let settings: Settings = toml::from_str(&settings_content).unwrap();

        let config = Config { labels, settings };

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
        let settings = config.settings;

        let label_name = labels.resolve("alias");
        let label_description = labels.get_description("label");
        assert_eq!(label_name, "label");
        assert_eq!(label_description, "a label");

        let link_dir = settings.link_dir;
        assert_eq!(link_dir, "test_dir/fileclass/temp/links");
    }
}
