use std::{thread, time, env, fs};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Automatically move new files added to a directory to the destination.\n");
        println!("Syntax: file-watch <source> <destination>\n");

        println!("Error: Missing arguments\n");

        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let dest = Path::new(&args[2]);

    let quiet = if args.contains(&"--quiet".to_string()) { true } else { false };

    if !quiet { println!("Starting loop.") }

    loop {

        let file_paths = match fs::read_dir(&path) {
            Ok(paths) => paths,
            Err(e) => {
                println!("Failed to read source directoy:\n{:?}", e);
                thread::sleep(time::Duration::from_secs(5));

                continue;
            }
        };

        for file in file_paths {
            let f = match file {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to read file: {:?}", e);
                    continue;
                }
            };

            match fs::copy(&f.path(), &dest.join(&f.file_name())) {
                Ok(bytes) => {
                    if !quiet { println!("Copied {:?}: {} bytes", f.path(), bytes) }
                },
                Err(e) => {
                    println!("Couldn't copy file {:?}: {:?}", f.path(), e);
                    continue;
                }
            }

            match fs::remove_file(f.path()) {
                Ok(_) => {
                    if !quiet { println!("Removing copied file {:?}", f.path()) }
                },
                Err(e) => {
                    println!("Couldn't remove file {:?}: {:?}", f.path(), e)
                }
            }
        }

        thread::sleep(time::Duration::from_secs(5));
    }
}
