use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

use super::label::LabelLibrary;

pub const STD_CONFIG_DIR: &str = "fileclass";
pub const LABELS_FILENAME: &str = "labels.toml";
pub const SETTINGS_FILENAME: &str = "settings.toml";

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub labels: LabelLibrary,
    pub settings: Settings,
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    // TODO: Use a default if missing and use that default in fcinit.
    pub link_dir: String,
}

impl Config {
    // TODO: Remove file system dependency from core.
    pub fn load(dir_path: &str) -> Result<Self, Box<dyn Error>> {
        let labels_path = Path::new(dir_path).join(LABELS_FILENAME);
        let labels_content = fs::read_to_string(labels_path)?;
        let labels = LabelLibrary::from_toml(&labels_content)?;

        let settings_path = Path::new(dir_path).join(SETTINGS_FILENAME);
        let settings_content = fs::read_to_string(settings_path)?;
        let settings: Settings = toml::from_str(&settings_content)?;

        let config = Config { labels, settings };

        Ok(config)
    }

    pub fn std_load() -> Result<Self, Box<dyn Error>> {
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
