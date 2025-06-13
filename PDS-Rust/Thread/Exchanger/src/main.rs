use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Exchanger<T> {
    tx: mpsc::Sender<Option<T>>, // other
    rx: mpsc::Receiver<Option<T>>, // its
}

impl<T> Exchanger<T> {
    pub fn exchange(&self, t:T) -> Option<T> {
        println!("Exchanging");
        if self.tx.send(Some(t)).is_err() { // gestione errore nel caso il canale (rx) venga chiuso
            return None;
        }

        println!("Waiting");
        let msg = self.rx.recv().unwrap();

        println!("Received");
        match msg {
            Some(text) => Some(text),
            None => None
        }
    }
}

pub fn create_exchanger<T>() -> (Exchanger<T>, Exchanger<T>) {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    (Exchanger{ tx: tx2, rx: rx1 }, Exchanger{ tx: tx1, rx: rx2 })
}


fn main() {
    let (exchanger1, exchanger2) = create_exchanger();
    let mut handlers = vec![];

    let handler = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::new(1, 0));
            let res = exchanger1.exchange("> th1 say hi to th2");
            println!("{:?}", res);
        }
    });
    handlers.push(handler);

    let handler = thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::new(1, 0));
            let res = exchanger2.exchange("> th2 say hi to th1");
            println!("{:?}", res);
        }
    });
    handlers.push(handler);

    for handler in handlers {
        handler.join().unwrap();
    }
}
