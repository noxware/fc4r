// ChatGPT ported code from old Deno version, with small modifications.

// TODO: Improve.
// TODO: Support dirs.

use std::path::Path;
use std::process;
use std::{env, fs};

use fileclass::extra::input::{map_stdin_sources_to_target_folder, SourceTargetPair};

#[cfg(windows)]
fn symlink<S: AsRef<Path>, L: AsRef<Path>>(source: S, link: L) -> std::io::Result<()> {
    use std::os::windows::fs::{symlink_dir, symlink_file};
    if source.as_ref().is_dir() {
        symlink_dir(source, link)
    } else {
        symlink_file(source, link)
    }
}

#[cfg(not(windows))]
fn symlink<S: AsRef<Path>, L: AsRef<Path>>(source: S, link: L) -> std::io::Result<()> {
    use std::os::unix::fs::symlink;
    symlink(source, link)
}

fn get_link_dir(args: &Vec<String>) -> String {
    match args.len() {
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
        // fs::hard_link(&source, &target)
        if let Err(err) = symlink(fs::canonicalize(&source).unwrap(), &target) {
            eprintln!(
                "Failed to create link for \"{}\": {}",
                source.to_str().unwrap(),
                err
            );
            process::exit(1);
        }
    });
}
