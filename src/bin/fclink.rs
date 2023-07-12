// ChatGPT ported code from old Deno version, with small modifications.

// TODO: Improve.
// TODO: Support dirs.

use std::fs;
use std::io::BufRead;
use std::path::Path;
use std::process;

use fileclass::core::config::Config;
use fileclass::utils::fs::get_unique_target;

fn main() {
    let config = Config::std_load().unwrap();
    let link_dir = config.settings.link_dir;

    // Get the target folder path
    let target_folder = Path::new(&link_dir);

    // Remove the target folder if it already exists
    if target_folder.exists() {
        fs::remove_dir_all(target_folder).unwrap();
    }

    // Create the target folder
    fs::create_dir_all(target_folder).unwrap();

    // Read standard input as lines of file paths
    let stdin = std::io::stdin().lock();
    let lines = stdin.lines();

    // Generate hard links to each file in the target folder
    for line in lines {
        if let Ok(file_path) = line {
            let source_path = Path::new(&file_path);
            let target_path = get_unique_target(source_path, target_folder);
            println!("{}", target_path.to_str().unwrap());

            // Temporal safe guard for directories and other entities.
            // TODO: Support directories at least.
            if !Path::new(&source_path).is_file() {
                eprintln!(
                    "Warning: \"{}\" is not a regular file, ignoring.",
                    source_path.to_str().unwrap()
                );

                continue;
            }

            // Hard link
            if let Err(err) = fs::hard_link(&source_path, &target_path) {
                eprintln!(
                    "Failed to create hard link for \"{}\": {}",
                    source_path.to_str().unwrap(),
                    err
                );
                process::exit(1);
            }
        }
    }
}
