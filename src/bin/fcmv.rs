use fileclass::utils::fs::get_unique_target;
use std::{env, fs, io, path::Path, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: fcmv <target_folder>");
    }

    let target_folder = Path::new(&args[1]);
    fs::create_dir_all(target_folder).expect("Can't create target folder");

    let stdin = io::stdin();
    let lines = stdin.lines();

    for line in lines {
        if let Ok(file_path) = line {
            let source_path = Path::new(&file_path);
            let target_path = get_unique_target(source_path, target_folder);

            if let Err(err) = fs::rename(&source_path, &target_path) {
                eprintln!("Failed to move file: {}", err);
                process::exit(1);
            }
        }
    }
}
