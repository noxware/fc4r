// ChatGPT ported code from old Deno version, with small modifications.

// TODO: Improve.
// TODO: Support dirs.

use std::path::Path;
use std::process;
use std::{env, fs};

use fileclass::core::config::Config;
use fileclass::extra::input::{map_stdin_sources_to_target_folder, SourceTargetPair};

fn get_link_dir(args: &Vec<String>) -> String {
    match args.len() {
        1 => {
            // TODO: Handle.
            let config = Config::std_load().unwrap();
            config.settings.link_dir
        }
        2 => args[1].clone(),
        _ => {
            eprintln!("Usage: fclink <target_dir>");
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let link_dir = get_link_dir(&args);

    // Get the target folder path
    let target_folder = Path::new(&link_dir);

    // Remove the target folder if it already exists
    if target_folder.exists() {
        fs::remove_dir_all(target_folder).unwrap();
    }

    // Create the target folder
    fs::create_dir_all(target_folder).unwrap();

    map_stdin_sources_to_target_folder(target_folder.to_path_buf()).for_each(|p| {
        let SourceTargetPair { source, target } = p;

        println!("{}", target.to_str().unwrap());

        // Temporal safe guard for directories and other entities.
        // TODO: Support directories at least.
        if !Path::new(&source).is_file() {
            eprintln!(
                "Warning: \"{}\" is not a regular file, ignoring.",
                source.to_str().unwrap()
            );

            return;
        }

        // Hard link
        if let Err(err) = fs::hard_link(&source, &target) {
            eprintln!(
                "Failed to create hard link for \"{}\": {}",
                source.to_str().unwrap(),
                err
            );
            process::exit(1);
        }
    });
}
