use std::collections::VecDeque;
use std::fs;
use std::env;
use std::io;

fn find_file(to_find: &str, search_path: &str) -> io::Result<bool> {
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut is_found = false;

    for entry in fs::read_dir(search_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() { //File
            if entry.file_name().to_string_lossy() == to_find {
                println!("Found file: {}", entry.path().display());
                is_found = true;
            }
        } else if metadata.is_dir() {   //Directory
            queue.push_back(entry.path().display().to_string());
        } 
    }

    while let Some(next_dir) = queue.pop_front() {
        if find_file(to_find, &next_dir)? {
            is_found = true;
        }
    }

    Ok(is_found)
}
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("No arguments provided!");
        return Ok(());
    }

    let to_find = &args[1];
    if (to_find == "-h") || (to_find == "--help") {
        println!("Usage: krab <filename> <path>");
        return Ok(());
    }

    let search_path = if args.len() > 2 {
        &args[2]
    } else {
        "."
    };

    println!("Searching for {} in {}", to_find, search_path);
    match find_file(to_find, search_path) {
        Ok(true) => (),
        Ok(false) => println!("File not found!"),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
