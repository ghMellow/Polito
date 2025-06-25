mod esame {
    use std::{
        sync::{Condvar, Mutex},
        time::{Duration},
    };

    struct Item {
        countdown: usize,
    }

    pub struct CountDownLatch {
        data: Mutex<Item>,
        cv: Condvar,
    }

    impl CountDownLatch {
        pub fn new(n: usize) -> Self {
            Self {
                data: Mutex::new(Item { countdown: n }),
                cv: Condvar::new(),
            }
        }

        // wait zero aspetta al massimo timeout ms

        // se esce per timeout ritorna Err altrimenti Ok

        pub fn wait_zero(&self, timeout: Option<std::time::Duration>) -> Result<(), ()> {
            match timeout {
                Some(timeout) => {
                    let mut data = self.data.lock().expect("Poisoned lock");
                    let mut timeout_res;
                    while data.countdown != 0 {
                        (data, timeout_res) = self.cv.wait_timeout(data, timeout).unwrap();
                        if timeout_res.timed_out() {
                            return Err(());
                        }
                    }

                    Ok(())
                }
                None => Err(()),
            }
        }

        pub fn count_down(&self) {
            let mut data = self.data.lock().expect("Poisoned lock");

            data.countdown -= 1;
            if data.countdown <= 0 {
                drop(data);
                self.cv.notify_all();
            }
        }
    }

    pub fn doSomeWork(id: &str) {
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use crate::esame::doSomeWork;
    use std::{thread, time::Duration};

    #[test]
    pub fn demo_latch() {
        let mut handles = vec![];

        for _ in 0..10 {
            let h = thread::spawn(|| {
                doSomeWork("(2) lavoro che necessita driver");

                doSomeWork("(3) altro lavoro che non necessita driver");
            });

            handles.push(h);
        }

        doSomeWork("(1) prepapara il driver");

        doSomeWork("(4) rilascia il driver");

        for h in handles {
            let _ = h.join();
        }
    }

    #[cfg(test)]
    mod tests {
        #[cfg(test)]
        mod tests {
            use crate::esame::{doSomeWork, CountDownLatch};
            use std::{sync::Arc, thread, time::Duration};

            #[test]
            pub fn test_dual_latch_synchronization() {
                println!("=== Test Sincronizzazione Bidirezionale con Due Latch ===");

                // Primo latch: Thread principale sblocca i 10 worker
                // Inizializzato a 1, verrà decrementato dal thread principale
                let start_latch = Arc::new(CountDownLatch::new(1));

                // Secondo latch: I 10 worker sbloccano il thread principale
                // Inizializzato a 10, verrà decrementato da ogni worker
                let completion_latch = Arc::new(CountDownLatch::new(10));

                let mut handles = vec![];

                // Avvia 10 thread worker
                for i in 1..=10 {
                    let start_latch_clone = Arc::clone(&start_latch);
                    let completion_latch_clone = Arc::clone(&completion_latch);

                    let handle = thread::spawn(move || {
                        println!("Worker {}: Avviato, in attesa del segnale di start...", i);

                        // FASE 1: Aspetta che il thread principale dia il via
                        match start_latch_clone.wait_zero(Some(Duration::from_secs(10))) {
                            Ok(()) => {
                                println!("Worker {}: Ricevuto segnale di start! Inizio lavoro...", i);

                                // Simula del lavoro
                                doSomeWork(&format!("Worker {} - lavoro principale", i));

                                // Simula tempo di elaborazione variabile
                                thread::sleep(Duration::from_millis(100 + (i * 50) as u64));

                                println!("Worker {}: Lavoro completato, notifico al thread principale", i);

                                // FASE 2: Notifica al thread principale che ha finito
                                completion_latch_clone.count_down();
                            },
                            Err(()) => {
                                println!("Worker {}: Timeout nell'attesa del segnale di start!", i);
                            }
                        }
                    });

                    handles.push(handle);
                }

                // Il thread principale fa del lavoro preparatorio
                println!("Thread principale: Preparazione iniziale...");
                doSomeWork("(1) Preparazione sistema");
                thread::sleep(Duration::from_millis(500)); // Simula preparazione

                println!("Thread principale: Preparazione completata, sblocco i worker!");

                // FASE 1: Il thread principale sblocca tutti i worker
                start_latch.count_down();

                println!("Thread principale: Segnale di start inviato, aspetto che tutti i worker finiscano...");

                // FASE 2: Il thread principale aspetta che tutti i worker finiscano
                match completion_latch.wait_zero(Some(Duration::from_secs(15))) {
                    Ok(()) => {
                        println!("Thread principale: Tutti i worker hanno completato il lavoro!");
                        doSomeWork("(2) Finalizzazione sistema");
                    },
                    Err(()) => {
                        println!("Thread principale: Timeout nell'attesa dei worker!");
                    }
                }

                // Aspetta che tutti i thread terminino
                for (i, handle) in handles.into_iter().enumerate() {
                    match handle.join() {
                        Ok(()) => println!("Worker {} terminato correttamente", i + 1),
                        Err(_) => println!("Worker {} terminato con errore", i + 1),
                    }
                }

                println!("=== Test Completato ===");
            }
        }
    }
}
