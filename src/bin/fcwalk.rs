// TODO: Add a way to ignore certain directories. Specially the `fileclass` dir.

use clap::Parser;
use std::env;
use std::fs;
use std::io;

use fileclass::core::config::{Config, STD_CONFIG_DIR};
use fileclass::extra::ipc::Message;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Do not load any config file
    #[arg(long)]
    no_config: bool,

    /// Do not output files/documents
    #[arg(long)]
    no_walk: bool,

    /// Forward stdin to stdout after processing
    #[arg(short, long)]
    forward: bool,
}

fn main() {
    let args = Args::parse();

    let config_flag = !args.no_config;
    let walk_flag = !args.no_walk;
    let forward_flag = args.forward;

    let current_dir = env::current_dir().unwrap();

    if config_flag {
        load_config();
    }

    if walk_flag {
        traverse_directory(&current_dir, &current_dir);
    }

    if forward_flag {
        io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();
    }
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
