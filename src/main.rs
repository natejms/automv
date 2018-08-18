use std::{thread, time};
use std::path::Path;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let path = Path::new(&args[1]);
            let dest = Path::new(&args[2]);

            println!(" ---------------");
            println!("- file-watch.rs -");
            println!("-    ~natejms   -");
            println!(" ---------------");
            println!("Starting loop.");
            loop {

                let file_paths = match fs::read_dir(&path) {
                    Ok(paths) => paths,
                    Err(e) => {
                        println!("Failed to read source directoyr: {:?}", e);
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
                        Ok(bytes) => println!("Copied {:?}: {} bytes", f, bytes),
                        Err(e) => {
                            println!("Couldn't copy file {:?}: {:?}", f, e);
                            continue;
                        }
                    }

                    match fs::remove_file(f.path()) {
                        Ok(_) => println!("Removing copied file {:?}", f),
                        Err(e) => println!("Couldn't remove file {:?}: {:?}", f, e)
                    }
                }


                println!("Waiting for more files...");
                thread::sleep(time::Duration::from_secs(5));
            }
        },
        _ => {
            println!(" ---------------");
            println!("- file-watch.rs -");
            println!(" ---------------");
            println!("Automatically move new files added to a directory to the specified destination.\n");
            println!("Syntax: file-watch <source> <destination>");
        }
    }
}
