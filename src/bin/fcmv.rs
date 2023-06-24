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
            let file_path = file_path.trim();
            let file_name = Path::new(&file_path).file_name().unwrap();
            let mut target_path = target_folder.join(file_name);
            let mut index = 1;

            while target_path.exists() {
                let file_name = Path::new(&file_path).file_stem().unwrap();
                let file_ext = Path::new(&file_path).extension().unwrap();
                let link_name = format!(
                    "{} ({}).{}",
                    file_name.to_str().unwrap(),
                    index,
                    file_ext.to_str().unwrap()
                );
                target_path = target_folder.join(link_name);
                index += 1;
            }

            if let Err(err) = fs::rename(&file_path, &target_path) {
                eprintln!("Failed to move file: {}", err);
                process::exit(1);
            }
        }
    }
}
