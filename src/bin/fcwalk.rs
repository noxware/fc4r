// TODO: Add a way to ignore certain directories. Specially the `fileclass` dir.

use std::env;
use std::fs;

use fileclass::core::config::{Config, STD_CONFIG_DIR};
use fileclass::extra::ipc::Message;

fn main() {
    // Get the current directory
    let current_dir = env::current_dir().unwrap();

    load_config();

    // Recursively traverse the directory tree
    traverse_directory(&current_dir, &current_dir);
}

fn traverse_directory(path: &std::path::Path, base_path: &std::path::Path) {
    // Read the directory entries
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                // Omit the fileclass directory.
                if !entry_path.ends_with(STD_CONFIG_DIR) {
                    let relative_path = entry_path.strip_prefix(base_path).unwrap();

                    // Print the relative entry path
                    println!("{}", relative_path.display());

                    // Recursively traverse subdirectories
                    if entry_path.is_dir() {
                        traverse_directory(&entry_path, base_path);
                    }
                }
            }
        }
    }
}

fn load_config() {
    match Config::std_load() {
        Ok(config) => {
            let msg = Message::Config(config);
            println!("{}", msg.serialize());
        }
        Err(e) => {
            // TODO: Only warn if missing.
            // TODO: Fail if the config is invalid.
            eprintln!("Error loading config: {}", e);
        }
    }
}
