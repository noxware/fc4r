use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::label::LabelLibrary;
use std::error::Error;

pub struct Config {
    pub labels: LabelLibrary,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut labels_file = File::open(path)?;
        let mut labels_content = String::new();
        labels_file.read_to_string(&mut labels_content)?;

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
        let config = Config::load("config/labels.toml").unwrap();
        let labels = config.labels;

        let label_name = labels.resolve("alias").unwrap();
        let label_description = labels.get_description("label").unwrap();

        assert_eq!(label_name, "label");
        assert_eq!(label_description, "a label");
    }
}
