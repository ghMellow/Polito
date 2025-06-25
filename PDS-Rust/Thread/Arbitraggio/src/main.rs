use std::sync::{mpsc, Arc, Condvar};
use std::{thread};
use std::time::Duration;

/**
struct Arbitro{
    cv_arbitro: Condvar,
    cv: Condvar,
}

* qua meglio usare canali.

 */
pub struct Arbitro {
    txs: Vec<mpsc::Sender<String>>, // txs dei lavoratori
    rx: mpsc::Receiver<String>, // arbitro
}

pub struct Lavoratore {
    tx: mpsc::Sender<String>, // tx dell arbitro
    rx: mpsc::Receiver<String>, // lavoratore
}

pub fn crea_arbitro() -> (Arbitro, Vec<Lavoratore>) {
    // arbitro
    let (tx_arbitro, rx_arbitro) = mpsc::channel();
    let mut txs_lavoratori = vec![];

    // lavoratori
    let mut vec_lavoratori = vec![];
    for _ in 0..10 {
        let (tx, rx) = mpsc::channel();

        txs_lavoratori.push(tx.clone());
        vec_lavoratori.push(Lavoratore{ tx: tx_arbitro.clone(), rx });
    }

    (Arbitro{txs: txs_lavoratori, rx: rx_arbitro}, vec_lavoratori)
}

fn main() {
    let (arbitro, lavoratori) = crea_arbitro();
    let mut handles = vec![];

    let size = lavoratori.len();

    for (i, lavoratore) in lavoratori.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            lavoratore.tx.send(format!("{}", i)).unwrap();

            let message = lavoratore.rx.recv().unwrap();
            println!("  > lavoro assegnato: {}", message);
            thread::sleep(Duration::new(1, 0));
            println!("  < lavoro #{} finito", message);
        });
        handles.push(handle);
    }

    thread::spawn(move || {
        for _ in 0..size {
            let id = arbitro.rx.recv().unwrap();
            println!("- arbitro riceve richiesta da {} e assegna il job", id);
            thread::sleep(Duration::new(1, 0));
            arbitro.txs[id.parse::<usize>().unwrap()].send(format!("work #{}", id)).unwrap();
        }
    });

    for handle in handles {
        handle.join().unwrap();
    }
}
