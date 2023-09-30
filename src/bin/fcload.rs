// TODO: Add a way to ignore certain directories. Specially the `fileclass` dir.

use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;

use fileclass::core::config::{Config, STD_CONFIG_DIR};
use fileclass::core::error::ErrorKind;
use fileclass::extra::ipc::Message;

const DEFAULT_WORKDIR: &str = ".";

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
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

    /// Load the specified directory instead
    #[arg(short, long, default_value = DEFAULT_WORKDIR)]
    workdir: String,
}

fn main() {
    let args = Args::parse();

    let config_flag = !args.no_config;
    let walk_flag = !args.no_walk;
    let forward_flag = args.forward;
    let workdir = args.workdir;

    if config_flag {
        load_config(&workdir);
    }

    if walk_flag {
        traverse_directory(&workdir, &workdir);
    }

    if forward_flag {
        io::copy(&mut io::stdin(), &mut io::stdout()).unwrap();
    }
}

fn traverse_directory<P: AsRef<Path>, B: AsRef<Path>>(path: P, base_path: B) {
    let path = path.as_ref();
    let base_path = base_path.as_ref();

    // Read the directory entries
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                // Omit the fileclass directory.
                if !entry_path.ends_with(STD_CONFIG_DIR) {
                    // let relative_path = entry_path.strip_prefix(base_path).unwrap();
                    // Print the relative entry path
                    // println!("{}", relative_path.display());

                    println!("{}", entry_path.display());

                    // Recursively traverse subdirectories
                    if entry_path.is_dir() {
                        traverse_directory(entry_path, base_path);
                    }
                }
            }
        }
    }
}

fn load_config(workdir: &str) {
    let config_dir = format!("{}/{}", workdir, STD_CONFIG_DIR);

    match Config::load(&config_dir) {
        Ok(config) => {
            let msg = Message::Config(config);
            println!("{}", msg.serialize());
        }
        Err(e) => match e.kind {
            ErrorKind::MissingConfig => {
                eprintln!("Warning: {}", e);
            }
            _ => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}
