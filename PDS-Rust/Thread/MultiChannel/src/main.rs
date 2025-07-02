use std::{sync::Arc, thread, time::Duration};

use crate::multi_channel::{Message, MultiChannel};


pub mod multi_channel{
    use std::sync::{mpsc::{Sender, Receiver}, mpsc::{self, SendError}, Mutex};
    
    #[derive(Debug, PartialEq)]
    pub enum Message{
        message(u8),
        shoutdown,
    }
    pub struct MultiChannel {
       // multichannel: Mutex<Data>,
       multichannel: Mutex<Vec<Sender<Message>>> // proprietà della libreria mpsc che permette di eliminare i canali non più validi
    }

    impl MultiChannel {
        pub fn new() -> Self {
            // MultiChannel{ multichannel: Mutex::new(Data{ txs: Vec::new(), id_counter: 0 }) }
            MultiChannel{ multichannel: Mutex::new(Vec::new()) }
        }

        pub fn subscribe(&self) -> Receiver<Message> {
            let (sender, receiver) = mpsc::channel();

            let mut multichannel = self.multichannel.lock().unwrap();
            multichannel.push(sender);
            drop(multichannel);

            return receiver;
        }

        pub fn send(&self, data: u8) -> Result<(), SendError<u8>> {
            let mut multichannel = self.multichannel.lock().unwrap();

            if multichannel.is_empty() {
                return Err(SendError(data));
            }

            // invio messaggio a tutti i canali e filtro quelli che sono stati chiusi nel frattempo
            multichannel.retain(|sender| { sender.send(Message::message(data)).is_ok() });
            drop(multichannel);

            Ok(())
        }

        pub fn shoutdown(&self) {
            let mut multichannel = self.multichannel.lock().unwrap();

            multichannel.retain(|sender| { sender.send(Message::shoutdown).is_ok() });
            multichannel.clear();
        }
    }
}

fn main(){
    let multi_channel = Arc::new(MultiChannel::new());
    let mut handlers = vec![];

    let sub1 = multi_channel.clone();
    let handler = thread::spawn(move || {
        let rec = sub1.subscribe();

        thread::sleep(Duration::from_millis(10)); // per dar tempo a sub2 di iscriversi 
        // posso anche far scrivere ai singoli thread. 
        sub1.send(u8::from(1)).unwrap(); // Attenzione: non mettere dentro a loop altrimenti deadloop!!

        loop{
            match rec.recv() {
                Ok(Message::message(data)) => {
                    println!("(Sub1) dato ricevuto: {}", data);
                    if data == u8::from(5) {
                        // ATTENZIONE: 
                        // qui chiudi il singolo ricevitore ma rimane nel vett dei txs fino al prossimo send. Versione non pulira, aggiungere un enum per astrarre il discorso.
                        println!("(Sub1) canale chiuso"); 
                        drop(rec);
                        break;
                    }
                },
                Ok(Message::shoutdown) => {
                    println!("(Sub1) canale chiuso"); 
                    drop(rec);
                    break;
                }
                Err(_) => {println!("canale chiuso"); break;}
            }

            
        }
    });
    handlers.push(handler);

    let sub2 = multi_channel.clone();
    let handler = thread::spawn(move || {
        let rec = sub2.subscribe();

        loop{
            match rec.recv() {
                Ok(Message::message(data)) => {
                    println!("(Sub2) dato ricevuto: {}", data);
                    if data == (1 as u8) {
                        println!("(Sub2) canale chiuso");
                        drop(rec);
                        break;
                    }
                },
                Ok(Message::shoutdown) => {
                    println!("(Sub2) canale chiuso");
                    drop(rec);
                    break;
                }
                Err(_) => {println!("canale chiuso"); break;}
            }
        }
    });
    handlers.push(handler);


    thread::sleep(Duration::from_millis(10));
    if let Err(err) = multi_channel.send(u8::from(4)) {
        println!("err nel send: {:?}", err);
    }
    if let Err(err) = multi_channel.send(u8::from(5)) {
        println!("err nel send: {:?}", err);
    }    
    if let Err(err) = multi_channel.send(u8::from(9)) {
        println!("err nel send: {:?}", err);
    }    

    thread::sleep(Duration::from_millis(1000));
    multi_channel.shoutdown();
    drop(multi_channel);


    for handler in handlers{
        handler.join().unwrap();
    }
}