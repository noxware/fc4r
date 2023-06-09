// ChatGPT generated code.

// TODO: Improve.

use std::env;
use std::fs;
use std::path::Path;
use std::process;

const DIR_NAME: &str = "hXZGVgb4_fileclass_links";

fn main() {
    // Get the system temporary directory
    let temp_dir = env::temp_dir();

    // Create the target folder within the temporary directory
    let target_folder = temp_dir.join(DIR_NAME);

    // Remove the target folder if it already exists
    if target_folder.exists() {
        fs::remove_dir_all(&target_folder).unwrap();
    }

    // Create the target folder
    fs::create_dir(&target_folder).unwrap();

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
