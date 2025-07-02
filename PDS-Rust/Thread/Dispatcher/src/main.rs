use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread, time::Duration};

#[derive(Clone, Debug, PartialEq)]
pub enum Msg{
    Mainessage(String),
    Destroyed,
}

pub struct Dispatcher{
    txs: Mutex<Vec<Sender<Msg>>>
}

impl Dispatcher {
    pub fn new() -> Self{
        Self { txs: Mutex::new(Vec::new()) }
    }

    pub fn subscribe(&self) -> Subscription{
        let (tx, rx) = channel::<Msg>();

        let mut txs = self.txs.lock().unwrap();
        txs.push(tx);
        Subscription { rx }
    }

    pub fn dispatch(&self, msg: Msg) {
        let mut txs = self.txs.lock().unwrap();
        txs.retain(|tx: &Sender<Msg>| tx.send(msg.clone()).is_ok()); //invio msg a tutti i tx e elimino tx chiusi
    }

    pub fn destroy_dispatcher(&self){
        let mut txs = self.txs.lock().unwrap();
        txs.retain(|tx: &Sender<Msg>| tx.send(Msg::Destroyed).is_ok()); //invio chiusura a tutti i tx e elimino tx chiusi
    }
}

pub struct Subscription{
    rx: Receiver<Msg>,
}

impl Subscription{
    pub fn read(&self) -> Option<Msg>{
        match self.rx.recv(){
            Ok(Msg::Mainessage(msg)) => Some(Msg::Mainessage(msg)),
            Ok(Msg::Destroyed) => Some(Msg::Destroyed),
            Err(_) => None,
        }
    }
}

impl Iterator for Subscription{
    type Item = Msg;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}


fn main(){
    let dispatcher = Dispatcher::new();
    let mut handlers = vec![];

    
    let rx1 = dispatcher.subscribe();
    handlers.push(thread::spawn(move || {
        loop{
            match rx1.read(){
                Some(Msg::Mainessage(msg)) => println!("(rx1) ricevuto messaggio: {:?}", msg),
                Some(Msg::Destroyed) => {
                    println!("dispatcher chiuso ergo no altri messaggi, chiusura (rx1)"); 
                    break;
                },
                None => panic!("rx1 errore"),
            }
        }
    }));

    dispatcher.dispatch(Msg::Mainessage("ciao".to_string()));

    let rx2 = dispatcher.subscribe();
    handlers.push(thread::spawn(move || {
        loop{
            match rx2.read(){
                Some(Msg::Mainessage(msg)) => println!("(rx2) ricevuto messaggio: {:?}", msg),
                Some(Msg::Destroyed) => {
                    println!("dispatcher chiuso ergo no altri messaggi, chiusura (rx2)"); 
                    break;
                },
                None => panic!("rx2 errore"),
            }
        }
    }));

    dispatcher.dispatch(Msg::Mainessage("ciao mondo".to_string()));

    dispatcher.destroy_dispatcher();

    for handler in handlers{
        handler.join().unwrap();
    }




    // Test prof
    let dispatcher = Arc::new(Dispatcher::new());
    
    // Thread ricevente 1
    let dispatcher1 = Arc::clone(&dispatcher);
    let j1 = thread::spawn(move || {
        let receiver = dispatcher1.subscribe();
        drop(dispatcher1);
        for msg in receiver {
            println!("Thread 1 received: {:?}", msg);
        }
        println!("Thread 1 exiting");
    });
    
    // Thread ricevente 2
    let dispatcher2 = Arc::clone(&dispatcher);
    let j2 = thread::spawn(move || {
        let receiver = dispatcher2.subscribe();
        drop(dispatcher2);
        for msg in receiver {
            println!("Thread 2 received: {:?}", msg);
        }
        println!("Thread 2 exiting");
    });
    
    // Thread mittente
    let dispatcher_sender = Arc::clone(&dispatcher);
    let sender_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        dispatcher_sender.dispatch(Msg::Mainessage("Hello, world!".to_string() ));
        dispatcher_sender.dispatch(Msg::Mainessage("Bye!".to_string() ));
    });
    
    sender_thread.join().unwrap();
    drop(dispatcher); // questo è l'ULTIMO Arc
    j1.join().unwrap();
    j2.join().unwrap();
}
