mod canali {
    use std::sync::{mpsc, Mutex};

    struct Waiter {
        tx: Vec<mpsc::Sender<()>>, // ogni thread deve avere il collegamento agli altri canali
        rx: mpsc::Receiver<()>, // senza Vec si creano n thread aventi 1 canale indipendente dagli altri
    }

    pub struct CyclicBarrier {
        data: Vec<Waiter>, // Mutex non necessario, infatti il vettore non viene modificato!
                           // Si accede ai dati in sola lettura e si sfruttano i metodi di questi
    }

    impl CyclicBarrier {
        pub fn new(n: usize) -> Self {
            // Step 1: creo n canali indipendenti.
            let mut senders = Vec::with_capacity(n);
            let mut receivers = Vec::with_capacity(n);

            for _ in 0..n {
                let (tx, rx) = mpsc::channel();
                senders.push(tx);
                receivers.push(Some(rx));
            }

            // Step 2, creo n Waiter aventi il ricevitore del proprio canale e il trasmettitore di tutti gli altri canali
            let mut data = Vec::with_capacity(n);
            for i in 0..n {
                let channel_receiver = receivers[i].take().unwrap(); // ricevitore non clonabile, wrappo in option dopodichè take()
                let mut channels_sender = Vec::with_capacity(n);
                for j in 0..n {
                    if i != j {
                        channels_sender.push(senders[j].clone()); // mi prendo i tx degli altri canali (channels)
                    }
                }

                data.push(Waiter{ tx: channels_sender, rx: channel_receiver });
            }

            Self { data: data }
        }

        pub fn wait(&self, n: usize, id: usize) {
            // Fase 1: Invia un messaggio a tutti gli altri thread (n-1 messaggi)
            for i in 0..n-1 {
                println!("thread_{} send message", id);
                self.data[id].tx[i].send(()).unwrap();
                //drop(self.data[id].tx[i]);
            }

            // Fase 2: Ricevi n-1 messaggi dagli altri thread
            while let Ok(msg) = self.data[id].rx.recv() {
                println!("thread_{} received message", id);
            }

            println!("thread_{} passed barrier", id);
        }
    }
}


/**
    PROBLEMA DUPLICE:
        1) SE IL RICEVITORE DORME FINCHè NON VENGONO CHIUSI I TRASMETTITORI (?),
           QUESTI SONO DENTRO UNA STRUTTURA PERCIò NON POSSO CHIUDERLI e rimane sempre in attesa!
        2) ALLO STESSO TEMPO ADDORMENTANDOSI NON RILASCIA IL LOCK E SE IMPLEMENTO SENZA MUTEX MI DA ERRORE CORSA CRITICA!
 */
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use crate::canali::CyclicBarrier;

    #[test]
    fn it_works() {
        let n = 5;
        let waiter = Arc::new(Mutex::new(CyclicBarrier::new(n)));

        std::thread::scope(|s|  {
            for id in 0..n {
                let waiter_clone = waiter.clone();
                s.spawn(move || {
                    waiter_clone.lock().unwrap().wait(n, id);
                    println!("thread {} end waiting", id);
                });
            }
        });
    }
}