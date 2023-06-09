// ChatGPT ported code from old Deno version, with small modifications.

// TODO: Improve.
// TODO: Support dirs.

use std::fs;
use std::path::Path;
use std::process;

use fileclass::core::config::Config;

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

    // Output the target folder path
    println!("{}", target_folder.display());

    // Read standard input as lines of file paths
    let stdin = std::io::stdin();
    let lines = stdin.lines();

    // Generate hard links to each file in the target folder
    for line in lines {
        if let Ok(file_path) = line {
            let file_path = file_path.trim();
            let file_name = Path::new(&file_path).file_name().unwrap();
            let mut link_path = target_folder.join(file_name);
            let mut index = 1;

            while link_path.exists() {
                let file_name = Path::new(&file_path).file_stem().unwrap();
                let file_ext = Path::new(&file_path).extension().unwrap();
                let link_name = format!(
                    "{} ({}).{}",
                    file_name.to_str().unwrap(),
                    index,
                    file_ext.to_str().unwrap()
                );
                link_path = target_folder.join(link_name);
                index += 1;
            }

            // Hard link
            if let Err(err) = fs::hard_link(&file_path, &link_path) {
                eprintln!("Failed to create hard link: {}", err);
                process::exit(1);
            }
        }
    }
}
