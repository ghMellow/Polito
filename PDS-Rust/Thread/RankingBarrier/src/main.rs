use std::collections::VecDeque;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Mutex, Arc, mpsc};
use std::thread;
use std::time::Duration;
use fastrand;

pub struct Canale {
    tx: Sender<usize>, // trasmettitore al proprietario
    rx: Receiver<usize>, // ricevitore proprio
}
pub struct RankingBarrier{
    txs: Vec<Sender<usize>>, // trasmettitore di tutti i canali
    rx: Receiver<usize>, // ricevitore proprio

    coda: VecDeque<usize> // vec di sender perciò mi basta l'indice
}

pub fn crea_ranking_barrier() -> (RankingBarrier, Vec<Canale>) {
    let (txs_ranking_barrier, rx_ranking_barrier) = mpsc::channel();
    let mut txs = Vec::new();

    let mut canali = vec![];

    for _ in 0..5 {
        let (tx, rx) = mpsc::channel();

        txs.push(tx);
        canali.push(Canale{ tx: txs_ranking_barrier.clone(), rx });
    }

    (RankingBarrier{txs, rx: rx_ranking_barrier, coda: VecDeque::new()}, canali)
}

fn main() {
    let mut ranking_barrier;
    let canali;
    (ranking_barrier, canali) = crea_ranking_barrier();

    let mut handles = vec![];

    for (i, canale) in canali.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            // sleep causale
            thread::sleep(Duration::new(fastrand::u64(0..10), 0));

            // scrivo al capo
            canale.tx.send(i).unwrap();

            // aspetto capo che dovrebbe tornare in ordine di arrivo FIFO
            let msg = canale.rx.recv().unwrap();
            println!("th{}: FIFO position {}", i, msg);
        });
        handles.push(handle);
    }

    let handle = thread::spawn(move || {
        for i in 0..5 {
            let id = ranking_barrier.rx.recv().unwrap();
            ranking_barrier.coda.push_back(id);
            println!("ranking_barrier: received th {}", id);
        }

        for (i, id_canale) in ranking_barrier.coda.into_iter().enumerate() {
            ranking_barrier.txs[id_canale].send(i).unwrap();
        }
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
