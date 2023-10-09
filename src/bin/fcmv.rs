use fileclass::extra::input::{map_stdin_sources_to_target_folder, SourceTargetPair};
use std::{env, fs, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: fcmv <target_folder>");
    }

    let target_folder = Path::new(&args[1]);
    fs::create_dir_all(target_folder).expect("Can't create target folder");

    map_stdin_sources_to_target_folder(target_folder.to_path_buf()).for_each(|p| {
        let SourceTargetPair { source, target } = p;

        if let Some(target) = target {
            if let Err(err) = fs::rename(&source, &target) {
                eprintln!("Failed to move file: {}", err);
                process::exit(1);
            }
        } else {
            eprintln!("Ignoring file: {}", source.display());
        }
    });
}
