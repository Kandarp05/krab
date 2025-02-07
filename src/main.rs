mod cli;

use std::env;
use cli::output::print_options;
use krab::Finder;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("No arguments provided!");
        return Ok(());
    }

    let to_find = args[1].clone();
    if (to_find == "-h") || (to_find == "--help") {
        println!("Usage: krab <filename> <path>");
        return Ok(());
    }

    let search_path = if args.len() > 2 {
        args[2].clone()
    } else {
        ".".to_string()
    };

    let finder = Finder::new(100);
    
    match finder.search(to_find, search_path) {
        Ok(results) => {
            if results.is_empty() {
                println!("No marches found!")
            } else {
                print_options(results)?;
            }
        }
        Err(e) => eprintln!("{e}"),
    }

    Ok(())

}
