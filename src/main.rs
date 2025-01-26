use std::collections::VecDeque;
use std::fs;
use std::env;
use std::io;
use std::sync::mpsc::{channel, Sender};
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};

fn recursive_search(
    pool: &ThreadPool, 
    queue: Arc<Mutex<VecDeque<String>>>, 
    to_find: String, 
    search_path: String,
    tx: Sender<String>
) -> io::Result::<()> {
    for entry in fs::read_dir(search_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            if entry.file_name().to_string_lossy().to_lowercase() == to_find.to_lowercase() {
                tx.send(entry.path().display().to_string()).unwrap();
            }
        } else if metadata.is_dir() {
            queue.lock().unwrap().push_back(entry.path().display().to_string());
        } 
    }

    while let Some(next_dir) = queue.lock().unwrap().pop_front() {
        let queue_c = Arc::clone(&queue);
        let pool_c = pool.clone();
        let to_find_c = to_find.clone();
        let tx_c = tx.clone();

        pool.execute(move || {
            let _ = recursive_search(&pool_c, queue_c, to_find_c, next_dir, tx_c);
        });
    }

    Ok(())
}

fn find_file(
    to_find: String, 
    search_path: String, 
) -> io::Result<()> {
    let n_workers = 10;
    let pool = ThreadPool::new(n_workers);

    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let (tx, rx) = channel();

    println!("Searching for {} in {}", &to_find, &search_path);
    recursive_search(&pool, Arc::clone(&queue), to_find, search_path, tx)?;

    let mut is_found = false;
    for path in rx {
        println!("> {}", path);
        is_found = true;
    }

    if !is_found {
        println!("Not found!");
    }

    pool.join();

    Ok(())
}

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

    find_file(to_find, search_path).unwrap();
    Ok(())
}
