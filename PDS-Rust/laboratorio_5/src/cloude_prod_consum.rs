use std::collections::VecDeque;
use std::ops::DerefMut;
use std::sync::{Arc, Condvar, Mutex};
use rand::Rng;
use std::{thread, time::Duration};

pub enum Item<T> {
    Value(T),
    Stop
}  // coda fifo stop viene visto per ultimo e ferma il consumer

struct Inner<T> {
    buffer: VecDeque<Item<T>>, // VecDeque allow access to front and back of the vector
    capacity: usize,           // dimension of the buffer
    is_closed: bool,           // used to stop the producer
}

pub struct MyChannel<T> {
    item: Mutex<Inner<T>>,  // mutex access to shared struct
    not_empty: Condvar,     // conditional variable to prevent busy waiting in these condition
    not_full: Condvar,
}

impl<T> MyChannel<T> {
    pub fn new(capacity: usize) -> MyChannel<T> {
        MyChannel {
            item: Mutex::new(Inner {
                buffer: VecDeque::with_capacity(capacity),
                capacity,
                is_closed: false
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new()
        }
    }

    pub fn write(&self, item: Item<T>) -> Result<(), usize> {
        let mut guard = self.item.lock().unwrap();

        // Attendi finché il buffer non è pieno usando la condition variable
        while guard.buffer.len() >= guard.capacity && !guard.is_closed {
            guard = self.not_full.wait(guard).unwrap();
        }

        if guard.is_closed {
            return Err(guard.buffer.len());
        }

        guard.buffer.push_back(item);

        // Notifica ai consumer che il buffer non è più vuoto
        self.not_empty.notify_one();

        Ok(())
    }

    pub fn read(&self) -> Result<T, usize> {
        let mut guard = self.item.lock().unwrap();

        // Attendi finché il buffer non è vuoto usando la condition variable
        while guard.buffer.is_empty() {
            guard = self.not_empty.wait(guard).unwrap();
        }

        if let Some(item) = guard.buffer.pop_front() {
            // Notifica ai producer che il buffer non è più pieno
            self.not_full.notify_one();

            match item {
                Item::Value(v) => Ok(v),
                Item::Stop => {
                    // Rimetti Stop in coda così altri consumer possono vederlo
                    guard.buffer.push_front(Item::Stop);
                    self.not_empty.notify_one();
                    Err(guard.buffer.len())
                }
            }
        } else {
            // Non dovrebbe mai accadere dato il controllo precedente
            panic!("buffer here should be non-empty, something bad happened");
        }
    }

    pub fn close(&self) {
        let mut guard = self.item.lock().unwrap();

        if !guard.is_closed {
            guard.is_closed = true;

            // Aggiungi un segnale di stop se il buffer non è pieno
            if guard.buffer.len() < guard.capacity {
                guard.buffer.push_back(Item::Stop);
                self.not_empty.notify_all(); // Notifica tutti i consumer
            }

            // Notifica tutti i producer bloccati
            self.not_full.notify_all();
        }
    }
}

fn random_sleep() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=10)
}

pub fn run_prod_consumer() {
    // dato che il tipo T viene propagato fino a item ma non viene inizializzato a tempo di creazione
    // il compilatore non può capire da solo che tipo sarà. Dato che in questo esempio voglio riempire
    // il buffer con numeri casuali specifico il tipo: 'struct::<tipo>::new'
    let my_channel = Arc::new(MyChannel::<usize>::new(10));
    let producer_channel = Arc::clone(&my_channel);
    let consumer_channel = Arc::clone(&my_channel);

    // Per fermare i thread dopo un po' di tempo
    let stop_channel = Arc::clone(&my_channel);

    thread::scope(|s| {
        // producer
        let producer_handle = s.spawn(move || {
            for i in 0..50 {  // Produci un numero limitato di valori
                let rnd = random_sleep();
                thread::sleep(Duration::from_secs(rnd as u64));
                if producer_channel.write(Item::Value(i)).is_err() {
                    println!("Producer stopping due to channel closure");
                    break;
                }
                println!("Produced: {}", i);
            }
        });

        // consumer
        let consumer_handle = s.spawn(move || {
            loop {
                let rnd = random_sleep();
                thread::sleep(Duration::from_secs(rnd as u64));

                match consumer_channel.read() {
                    Ok(val) => println!("Consumed: {}", val),
                    Err(_) => {
                        println!("Consumer stopping due to Stop signal");
                        break;
                    }
                }
            }
        });

        // Dopo un po' chiudi il canale
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(30));
            println!("Closing channel");
            stop_channel.close();
        });

        // Attendi che i thread terminino
        let _ = producer_handle.join();
        let _ = consumer_handle.join();
    });
}