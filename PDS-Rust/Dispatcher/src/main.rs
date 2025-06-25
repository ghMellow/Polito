use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use rand::{random, Rng};

#[derive(Clone, Debug)]
pub struct Msg {
    item: i32, // non essendo specificato il tipo di dati di Msg uso un semplice i32 così da implementare la clone() gratis
}

struct Subscription {
    // tx: Sender<Msg>,    // proprio
    rx: Receiver<Msg>,  // proprio
}
impl Subscription {
    pub fn read(&self) -> Option<Msg> {
        self.rx.recv().ok()
    }
}
pub struct Dispatcher {
    txs: Mutex<Vec<Sender<Msg>>>,  // tx dei th iscritti
    rx: Receiver<Msg>       // rx del canale
}

impl Dispatcher {
    pub fn new() -> Self {
        let (tx, rx) = channel();

        Dispatcher { txs: Mutex::new(vec![]), rx }
    }

    pub fn subscribe(&mut self) -> Subscription {
        let (tx, rx) = channel();

        let mut txs = self.txs.lock().unwrap();
        txs.push(tx.clone());

        Subscription{ rx }
    }

    pub fn dispatch(&self, msg:Msg) {
        let mut txs = self.txs.lock().unwrap();
        for tx in txs.iter() {
            tx.send(msg.clone()).unwrap();
        }
    }
}

fn main() {
    let mut dispatcher = Dispatcher::new();
    let mut handlers = vec![];

    for i in 0..5 {
        let subscriber = dispatcher.subscribe();
        let handle = thread::spawn(move || {
            for _ in 0..2 {
                match subscriber.read() {
                    Some(msg) => { println!("th {} received: {:?}", i, msg) },
                    None => { println!("No message"); },
                }
            }
        });
        handlers.push(handle);
    }


    let handler = thread::spawn(move || {
        thread::sleep(Duration::new(fastrand::u64(1..4), 0));

        for i in 0..2 {
            dispatcher.dispatch(Msg { item: i + 1 });
        }
    });
    handlers.push(handler);

    for handle in handlers {
        handle.join().unwrap();
    }
}
