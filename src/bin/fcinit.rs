use std::fs;
use std::path::Path;

use fileclass::core::config::{LABELS_DIRNAME, STD_CONFIG_DIR};

const MAIN_LABELS_FILENAME: &str = "main.toml";

const LABELS_CONTENT: &str = r#"[label]
description = "a label"
aliases = ["alias"]
implies = ["implied"]

[implied]"#;

fn main() {
    let folder_path = Path::new(STD_CONFIG_DIR);
    let labels_path = folder_path.join(LABELS_DIRNAME);
    let main_labels_path = labels_path.join(MAIN_LABELS_FILENAME);

    // Create the folder if it doesn't exist
    if !folder_path.exists() {
        fs::create_dir_all(folder_path).expect("Failed to create folder");
    }

    // Create the labels directory if it doesn't exist
    if !labels_path.exists() {
        fs::create_dir(labels_path).expect("Failed to create labels directory");
    }

    // Generate labels.toml if it doesn't exist
    if !main_labels_path.exists() {
        fs::write(main_labels_path, LABELS_CONTENT).expect("Failed to generate labels.toml");
    }
}
