use std::fs;
use std::path::Path;

use fileclass::core::config::{LABELS_FILENAME, STD_CONFIG_DIR};

const LABELS_CONTENT: &str = r#"[label]
description = "a label"
aliases = ["alias"]
implies = ["implied"]

[implied]"#;

fn main() {
    let folder_path = Path::new(STD_CONFIG_DIR);
    let labels_path = folder_path.join(LABELS_FILENAME);

    // Create the folder if it doesn't exist
    if !folder_path.exists() {
        fs::create_dir(folder_path).expect("Failed to create folder");
    }

    // Generate labels.toml if it doesn't exist
    if !labels_path.exists() {
        fs::write(labels_path, LABELS_CONTENT).expect("Failed to generate labels.toml");
    }
}
