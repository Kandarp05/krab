use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::io;
use std::collections::VecDeque;

use super::worker::SearchWorker;

pub struct Finder {
    pool: ThreadPool,
    queue: Arc<Mutex<VecDeque<String>>>,
}

impl Finder {
    pub fn new(thread_count: usize) -> Self {
        Finder {
            pool: ThreadPool::new(thread_count),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn search(
        &self,
        to_find: String, 
        search_path: String, 
    ) -> io::Result<Vec<String>> {
        let (tx, rx) = channel();
    
        let worker = SearchWorker::new(self.pool.clone(), Arc::clone(&self.queue));
        worker.execute(to_find, search_path, tx.clone())?;
        drop(tx);

        Ok(rx.iter().collect())
    }
}