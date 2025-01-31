use std::io;
use std::env;
use krab::Finder;


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

    let finder = Finder::new(50);
    let receiver = finder.search(to_find, search_path)?;

    for path in receiver {
        println!("{}", path);
    }

    Ok(())
}
