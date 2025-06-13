mod channel;

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Item {
    value: i32,
}
pub struct ThreadPool{
    data: Mutex<Vec<Item>>,
    cv: Condvar,
}

impl ThreadPool{
    pub fn new() -> Self {
        ThreadPool{ data: Mutex::new(Vec::new()), cv: Condvar::new() }
    }

    pub fn get_size(&self) -> usize{
        let data = self.data.lock().unwrap();
        data.len()
    }
}

fn main() {
    let thread_pool = Arc::new(ThreadPool::new());
    let mut handles = vec![];

    // producer
    let producer = Arc::clone(&thread_pool);
    let handle = thread::spawn(move || {
        for i in 1..10{
            let mut lock = producer.data.lock().expect("couldn't acquire lock");

            lock.push(Item{value: i});
            producer.cv.notify_one();
            println!("producer #{}", i);
        }
    });
    handles.push(handle);

    for i in 1..10{
        let reader = Arc::clone(&thread_pool);
        let handle = thread::spawn(move || {
            let mut guard = reader.data.lock().expect("couldn't acquire lock");

            guard = reader.cv.wait_while(guard, |guard| {guard.len() == 0}).unwrap();
            let val = guard.pop().unwrap();
            println!("thread #{} pop value: {}", i, val.value);

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    channel::run();
}
