use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::io;
use std::collections::VecDeque;
use std::fs;
use regex::Regex;

pub struct SearchWorker {
    pool: ThreadPool,
    queue: Arc<Mutex<VecDeque<String>>>,
}

impl SearchWorker {
    pub fn new(pool: ThreadPool, queue: Arc<Mutex<VecDeque<String>>>) -> Self {
        Self { pool, queue}
    }

    pub fn execute(
        &self,
        to_find: String,
        search_path: String,
        tx: Sender<String>
    ) -> io::Result<()> {
        self.recursive_search(to_find, search_path, tx)
    }

    fn recursive_search(
        &self,
        to_find: String, 
        search_path: String,
        tx: Sender<String>
    ) -> io::Result::<()> {
        let pattern = {
            let this = Regex::new(&format!(r"(?i)^{to_find}(\.[^.]+)?$"));
            match this {
                Ok(t) => Ok(t),
                Err(e) => Err((|e| io::Error::new(io::ErrorKind::Other, e))(e)),
            }
        }?;
        for entry in fs::read_dir(search_path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
    
            if pattern.is_match(&entry.file_name().to_string_lossy().to_lowercase()) {
                tx.send(entry.path().display().to_string()).unwrap();
            }
    
            if metadata.is_dir() {
                self.queue.lock().unwrap().push_back(entry.path().display().to_string());
            } 
        }
    
        while let Some(next_dir) = self.queue.lock().unwrap().pop_front() {
            let to_find_c = to_find.clone();
            let tx_c = tx.clone();
            let worker = SearchWorker::new(self.pool.clone(), Arc::clone(&self.queue));
    
            self.pool.execute(move || {
                let _ = worker.recursive_search(to_find_c, next_dir, tx_c);
            });
        }
    
        Ok(())
    }
}