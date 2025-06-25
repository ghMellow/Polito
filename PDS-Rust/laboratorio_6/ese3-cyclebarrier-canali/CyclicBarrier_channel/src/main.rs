use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Waiter {
    tx: Vec<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
}

impl Waiter {
    pub fn wait(&self) {
        for sender in &self.tx {
            sender.send(()).unwrap();
        }
        for _ in 0..self.tx.len() {
            self.rx.recv().unwrap();
        }
    }
}

pub struct CyclicBarrier {
    waiters: Vec<Arc<Mutex<Waiter>>>,
    next: AtomicUsize,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Self {
        let mut senders = Vec::with_capacity(n);
        let mut receivers = Vec::with_capacity(n);

        for _ in 0..n {
            let (tx, rx) = mpsc::channel();
            senders.push(tx);
            receivers.push(rx);
        }

        let mut waiters = Vec::with_capacity(n);
        for i in 0..n {
            let mut tx_others = Vec::with_capacity(n - 1);
            for j in 0..n {
                if i != j {
                    tx_others.push(senders[j].clone());
                }
            }
            waiters.push(Arc::new(Mutex::new(Waiter {
                tx: tx_others,
                rx: receivers.remove(0),
            })));
        }

        Self {
            waiters,
            next: AtomicUsize::new(0),
        }
    }

    pub fn get_waiter(&self) -> Arc<Mutex<Waiter>> {
        let idx = self.next.fetch_add(1, Ordering::SeqCst) % self.waiters.len();
        self.waiters[idx].clone()
    }
}

fn main() {
    let cbarrier = Arc::new(CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let waiter = cbarrier.get_waiter();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                waiter.lock().unwrap().wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}

/*use std::sync::mpsc;
use std::sync::Arc;

pub struct Waiter {
    tx: Vec<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
}

impl Waiter {
    pub fn wait(&self) {
        // Invia un messaggio a tutti gli altri thread
        for sender in &self.tx {
            sender.send(()).unwrap();
        }
        // Riceve n-1 messaggi dagli altri thread
        for _ in 0..self.tx.len() {
            self.rx.recv().unwrap();
        }
    }
}

pub struct CyclicBarrier {
    waiters: Vec<Arc<Waiter>>,
    next: std::sync::atomic::AtomicUsize,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> Self {
        let mut senders = Vec::with_capacity(n);
        let mut receivers = Vec::with_capacity(n);

        for _ in 0..n {
            let (tx, rx) = mpsc::channel();
            senders.push(tx);
            receivers.push(rx);
        }

        let mut waiters = Vec::with_capacity(n);
        for i in 0..n {
            // Ogni Waiter ha il proprio rx e i tx degli altri
            let mut tx_others = Vec::with_capacity(n - 1);
            for j in 0..n {
                if i != j {
                    tx_others.push(senders[j].clone());
                }
            }
            waiters.push(Arc::new(Waiter {
                tx: tx_others,
                rx: receivers.remove(0),
            }));
        }

        Self {
            waiters,
            next: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn get_waiter(&self) -> Arc<Waiter> {
        // Restituisce un Waiter diverso ogni volta (round robin)
        let idx = self.next.fetch_add(1, std::sync::atomic::Ordering::SeqCst) % self.waiters.len();
        self.waiters[idx].clone()
    }
}

fn main() {
    let cbarrier = Arc::new(CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let waiter = cbarrier.get_waiter();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                waiter.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}*/

// -------------------------------------------------------------------------------------------------

/*
use std::sync::mpsc;
use std::thread;

struct Waiter {
    tx: Vec<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
}

fn make_waiters(n: usize) -> Vec<Waiter> {
    let mut senders = Vec::with_capacity(n);
    let mut receivers = Vec::with_capacity(n);

    for _ in 0..n {
        let (tx, rx) = mpsc::channel();
        senders.push(tx);
        receivers.push(Some(rx));
    }

    let mut waiters = Vec::with_capacity(n);
    for i in 0..n {
        let rx = receivers[i].take().unwrap();
        let mut tx_others = Vec::with_capacity(n - 1);
        for j in 0..n {
            if i != j {
                tx_others.push(senders[j].clone());
            }
        }
        waiters.push(Waiter { tx: tx_others, rx });
    }
    waiters
}

impl Waiter {
    fn wait(&self) {
        for sender in &self.tx {
            sender.send(()).unwrap();
        }
        for _ in 0..self.tx.len() {
            self.rx.recv().unwrap();
        }
    }
}

fn main() {
    let n = 3;
    let waiters = make_waiters(n);

    let mut handles = Vec::new();
    for (i, waiter) in waiters.into_iter().enumerate() {
        handles.push(thread::spawn(move || {
            for j in 0..10 {
                waiter.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
 */