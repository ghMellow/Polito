use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
use fastrand;

#[derive(PartialEq, Eq, Debug)]
pub enum WaitResult {
    Success,
    Timeout,
    Canceled,

    Running // custom ridondante ma per maggiore chiarezza voglio inizializzare con un valore diverso da Success
}

pub trait CancelableLatch {
    fn new(count: usize) -> Self;
    fn count_down(&self);
    fn cancel(&self);
    fn wait(&self) -> WaitResult;
    fn wait_timeout(&self, d: Duration) -> WaitResult;
}

struct Item {
    count: usize,
    wait_result: WaitResult
}
pub struct Struttura {
    item: Mutex<Item>,
    cv: Condvar
}

impl CancelableLatch for Struttura {
    fn new(count: usize) -> Self {
        Struttura{ item: Mutex::new(Item{count, wait_result: WaitResult::Running}), cv: Condvar::new() }
    }

    fn count_down(&self) {
        let mut item = self.item.lock().unwrap();

        // gestione esecuzione non deterministica
        if item.wait_result != WaitResult::Running {
            drop(item);
            return;
        }

        item.count -= 1;

        if item.count == 0 {
            item.wait_result = WaitResult::Success;
            self.cv.notify_all();
        }
    }

    fn cancel(&self) {
        let mut item = self.item.lock().unwrap();
        item.count = 0;
        item.wait_result = WaitResult::Canceled;
        self.cv.notify_all();
    }

    fn wait(&self) -> WaitResult {
        let mut item = self.item.lock().unwrap();

        while item.count > 0 {
            item = self.cv.wait(item).unwrap();
        }

        /// Attenzione: item.wait_result //non puoi spostare un solo valore della struttura!
        match item.wait_result {
            WaitResult::Success => WaitResult::Success,
            WaitResult::Timeout => WaitResult::Timeout,
            WaitResult::Canceled => WaitResult::Canceled,
            WaitResult::Running => WaitResult::Success // check ridondante infatti dovrebbe essere
                                                       // sempre Success nel caso non entri nel while
                                                       // ma so che se esco e sono ancora in running
                                                       // jobs completati con successo.
        }
    }

    fn wait_timeout(&self, d: Duration) -> WaitResult {
        let mut item = self.item.lock().unwrap();

        if item.count > 0 {
            let timeout_result;

            (item, timeout_result) = self.cv.wait_timeout(item, d).unwrap();

            if timeout_result.timed_out() {
                item.wait_result = WaitResult::Timeout;
            }
        }

        match item.wait_result {
            WaitResult::Success => WaitResult::Success,
            WaitResult::Timeout => WaitResult::Timeout,
            WaitResult::Canceled => WaitResult::Canceled,
            WaitResult::Running => WaitResult::Success // come per while vale per if
        }
    }
}

fn main() {
    // --
    // caso success
    let mut cancellable_latch = Arc::new(Struttura::new(5));
    let mut handles = Vec::new();

    for i in 0..5 {
        let cancellable_latch = cancellable_latch.clone();
        let handle = thread::spawn(move || {
            println!("th {} fa count", i);
            cancellable_latch.count_down();

            let res = cancellable_latch.wait();
            println!("> th {} weak up with result {:?}", i, res);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    // --
    // caso cancel
    let mut cancellable_latch = Arc::new(Struttura::new(5));
    let mut handles = Vec::new();

    for i in 0..3 {
        let cancellable_latch = cancellable_latch.clone();
        let handle = thread::spawn(move || {
            println!("th {} fa count", i);
            cancellable_latch.count_down();

            let res = cancellable_latch.wait();
            println!("> th {} weak up with result {:?}", i, res);
        });
        handles.push(handle);
    }
    let cancellable_latch = cancellable_latch.clone();
    let handle = thread::spawn(move || {
        println!("th 4 chiude");
        cancellable_latch.cancel();
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }


    // --
    // caso timeout
    let mut cancellable_latch = Arc::new(Struttura::new(5));
    let mut handles = Vec::new();

    for i in 0..4 {
        let cancellable_latch = cancellable_latch.clone();
        let handle = thread::spawn(move || {
            println!("th {} fa count", i);
            cancellable_latch.count_down();

            let res = cancellable_latch.wait_timeout(Duration::new(fastrand::u64(0..10), 0));
            println!("> th {} weak up with result {:?}", i, res);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
