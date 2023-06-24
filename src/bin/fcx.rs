use std::env;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    /*
        if args.is_empty() {
            Command::new("fcinit")
                .spawn()
                .expect("Failed to execute fcinit")
                .wait() ////// none of this commands is aborting on error

            return;
        }
    */
    let fcwalk_output = Command::new("fcwalk")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute fcwalk")
        .stdout
        .expect("Failed to capture fcwalk output");

    let fcq_output = Command::new("fcq")
        .args(&args)
        .stdin(fcwalk_output)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute fcq")
        .stdout
        .expect("Failed to capture fcq output");

    let fclink_output = Command::new("fclink")
        .stdin(fcq_output)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute fclink")
        .stdout
        .expect("Failed to capture fclink output");

    let reader = io::BufReader::new(fclink_output);
    for line in reader.lines() {
        if let Ok(link_path) = line {
            println!("{}", link_path);
        }
    }
}
