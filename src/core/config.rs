use std::error::Error;
use std::fs;
use std::path::PathBuf;

use super::label::LabelLibrary;

pub struct Config {
    pub labels: LabelLibrary,
}

impl Config {
    // TODO: Remove file system dependency from core.
    pub fn load(dir_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut labels_path = PathBuf::from(dir_path);
        labels_path.push("labels.toml");

        let labels_content = fs::read_to_string(labels_path)?;
        let labels = LabelLibrary::from_toml(&labels_content)?;
        let config = Config { labels };

        Ok(config)
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
