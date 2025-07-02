/*

Un paradigma frequentemente usato nei sistemi reattivi è costituito dall'astrazione detta Looper.
Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread.
Il thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda, 
li estrae a uno a uno nell'ordine di arrivo, e li elabora. Il costruttore di Looper riceve due parametri, 
entrambi di tipo (puntatore a) funzione: process(...) e cleanup(). La prima è una funzione responsabile 
di elaborare i singoli messaggi ricevuti attraverso la coda; tale funzione accetta un unico parametro in ingresso 
di tipo Message e non ritorna nulla; La seconda è funzione priva di argomenti e valore di ritorno e verrà invocata 
dal thread incapsulato nel Looper quando esso starà per terminare.
Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per gestirne il ciclo di vita: send(msg), che accetta come parametro un oggetto generico di tipo Message che verrà inserito nella coda e successivamente estratto dal thread ed inoltrato alla funzione di elaborazione. Quando un oggetto Looper viene distrutto, occorre fare in modo che il thread contenuto al suo interno invochi la seconda funzione passata nel costruttore e poi termini.
Si implementi, utilizzando il linguaggio Rust o C++, tale astrazione tenendo conto che i suoi metodi
dovranno essere thread-safe.
*/

use std::{sync::{mpsc}, thread::{self, JoinHandle}};
use std::marker::Send;

enum Msg<T: Clone+ Send + 'static> {
    Message(T),
    Shoutdown,
}

struct Looper<T: Clone+ Send + 'static>{
    tx: mpsc::Sender<Msg<T>>, // tx già implementa clonazione
    handler: Option<JoinHandle<()>>,
}

impl<T: Clone+ Send + 'static> Looper<T> {
    pub fn new<P: Fn(Msg<T>) + Send + 'static, Q: FnOnce() + Send + 'static>(p: P, q: Q) -> Looper<T> {
        let (tx, rx) = mpsc::channel::<Msg<T>>();
        Looper { tx, handler: Some(thread::spawn(move || {
            loop{
                let messaggio = rx.recv().unwrap();

                match messaggio {
                    Msg::Message(t) => {
                        let msg = Msg::Message(t.clone());
                        p( msg )
                    },
                    Msg::Shoutdown => {
                        q();
                        drop(rx);
                        break;
                    },
                }
            }
        })) }
    }

    pub fn send(&self, msg: Msg<T>) {
        self.send(msg);
    }
}


impl<T: Clone+ Send + 'static> Drop for Looper<T> {
    fn drop(&mut self) {
        // 1. Invia segnale di shutdown
        self.tx.send(Msg::Shoutdown).ok();
        
        // 2. Aspetta che il thread finisca
        if let Some(handle) = self.handler.take() { 
            handle.join().ok(); 
        }
    }
}


fn main() {
    println!("Hello, world!");
}
