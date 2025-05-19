use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_primes_sol1(limit: u64, n_threads: u64) -> Vec<u64> {
    let count = Arc::new(AtomicU64::new(2));
    let primes = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for _ in 0..n_threads {
            let thread_count = Arc::clone(&count);
            let thread_primes = Arc::clone(&primes);

            s.spawn(move || {
                // let thread_id = thread::current().id();
                loop {
                    // Ottieni il valore corrente e incrementalo atomicamente
                    let current = thread_count.fetch_add(1, Ordering::SeqCst);

                    if current > limit {
                        break;
                    }

                    if is_prime(current) {
                        //println!("thread {:?} said: {} is prime", thread_id, current);
                        // Aggiungi il numero primo al vettore condiviso
                        let mut primes_guard = thread_primes.lock().unwrap();
                        primes_guard.push(current);
                    }
                }
            });
        }
    });

    // Estrai il vettore dal Arc<Mutex<>> e restituiscilo
    // let result = Arc::try_unwrap(primes)
    //     .expect("Impossibile ottenere la proprietà esclusiva dei primi")
    //     .into_inner()
    //     .unwrap();

    primes.lock().unwrap().to_vec()
}

pub fn find_primes_sol2(limit: u64, n_threads: u64) -> Vec<u64> {
    let primes = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for i in 0..n_threads {
            let start_value = 2+i; // per il count in base al numero di thread
            let thread_primes = Arc::clone(&primes);

            s.spawn(move || {
                // let thread_id = thread::current().id();
                let mut current = start_value; // qui i count sono indipendenti
                while current < limit {
                    if is_prime(current) {
                        //println!("thread {:?} said: {} is prime", thread_id, current);
                        // Aggiungi il numero primo al vettore condiviso
                        let mut primes_guard = thread_primes.lock().unwrap();
                        primes_guard.push(current);
                    }

                    current += n_threads; // incremento dell'offset che è proprio il numero di thread
                }
            });
        }
    });

    primes.lock().unwrap().to_vec()
}