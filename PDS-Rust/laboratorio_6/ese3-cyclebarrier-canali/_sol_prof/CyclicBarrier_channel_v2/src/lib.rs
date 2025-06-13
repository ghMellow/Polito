use std::sync::mpsc;

pub struct Waiter {
    tx: Vec<mpsc::Sender<()>>,
    rx: mpsc::Receiver<()>,
}

impl Waiter {

    pub fn wait(&self) {
        for i in 0..self.tx.len() {
            self.tx[i].send(()).unwrap();
        }

        for _ in 0..self.tx.len() {
            self.rx.recv().unwrap();
        }
    }
}

pub fn cyclic_barrier(n: usize) -> Vec<Waiter> {
    let mut txs = vec![];
    let mut rxs = vec![];

    // Creazione canali indipendenti !separati in due vettori! tx e rx sono legati a livello logico ma quando creati li posso spostare come voglio:
    //      il primo contenete tutti i ricevitori
    //      il secondo contenete tutti i trasmettitori
    for _ in 0..n {
        let (tx, rx) = mpsc::channel();
        txs.push(tx);
        rxs.push(rx);
    }

    // creazione vincolo tra canali
    // avendo il vettore di tutti i trasmettitori viene gratis, ciclo tra i trasmettitori e creo oggetto Waiter
    let mut res = vec![];
    for rx in rxs.into_iter() { //- into_iter per consumare il vettore e prendere possesso dei singoli elementi. (iter mi prede solo il riferimento)
        let txs = txs.clone();
        let w = Waiter{ tx: txs, rx };

        res.push(w);
    }

    res
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use crate::cyclic_barrier;

    #[test]
    fn it_works() {
        let ws = cyclic_barrier(2);
        let (tx, rx) = channel();

        std::thread::scope(|s| {
           for w in ws.into_iter() {    // cycle_barrier tra n thread
               let tx = tx.clone();

               s.spawn(move || {
                   for i in 0..2 { // cycle_barrier nello stesso thread
                       w.wait();
                       tx.send(i).unwrap();
                   }
               });
           }
        });
        drop(tx);
        assert_eq!(rx.recv().unwrap(), 0); // thread ma mi posso aspettare ordine perchè vado a step con il cycle barrier
        assert_eq!(rx.recv().unwrap(), 0);
        assert_eq!(rx.recv().unwrap(), 1);
        assert_eq!(rx.recv().unwrap(), 1);
        assert!(rx.recv().is_err());
    }
}