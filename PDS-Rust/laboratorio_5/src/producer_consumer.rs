use rand::Rng;
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::{thread, time::Duration};


enum Item<T> {Value(T), Stop}  // coda fifo stop viene visto per ultimo e ferma il consumer

struct Inner<T> {
    buffer: VecDeque<Item<T>>, // VecDeque allow access to front and back of the vector
    capacity: usize,           // dimension of the buffer
    is_closed: bool,           // used to stop the producer
}

struct MyChannel<T> {
    item: Mutex<Inner<T>>,  // mutex access to shared struct
    not_empty: Condvar,     // conditional variable to prevent busy waiting in these condition
    not_full: Condvar,
}

impl<T> MyChannel<T> {
    pub fn new(capacity: usize) -> MyChannel<T> {
        MyChannel{item: Mutex::new(
            Inner{
                buffer: VecDeque::with_capacity(capacity),
                capacity: capacity,
                is_closed: false
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new()
        }
    }
    pub fn write(&self, item: Item<T>) -> Result<(), usize> {
        let mut guard = self.item.lock().unwrap();

        // condition wait
        while guard.buffer.len() >= guard.capacity {
            guard = self.not_full.wait(guard).unwrap();
        }

        if guard.is_closed {
            println!("Stop Item added to buffer");
            guard.buffer.push_back(Item::Stop); // producer stopped, when possible add stop item to notify the consumer
            // Element added I must notify the consumer
            self.not_empty.notify_one();
            Err(0)
        } else {
            guard.buffer.push_back(item);
            // Element added I must notify the consumer
            self.not_empty.notify_one();
            Ok(())
        }
    }
    pub fn read(&self) -> Result<T, usize> {
        let mut guard = self.item.lock().unwrap();

        while guard.buffer.len() == 0 {
            guard = self.not_empty.wait(guard).unwrap();
        }

        if let Some(item) = guard.buffer.pop_front() {
            match item {
                Item::Value(v) => {
                    // value popped I must notify the producer
                    self.not_empty.notify_one();

                    Ok(v)
                }
                Item::Stop => {
                    println!("Stopping the reader");
                    Err(0)
                }
            }
        } else {
            println!("> Optional item vuoto, forzo chiusura del thread");
            Err(0)
        }
    }
    pub fn close(&self) {
        let mut guard = self.item.lock().unwrap();
        guard.is_closed = true;
    }
}


fn random_sleep(start_range: usize, end_range: usize) -> usize {
    rand::rng().random_range(start_range..=end_range)
}

pub fn run_prod_consumer() {
    // dato che il tipo T viene propagato fino a item ma non viene inizializzato a tempo di creazione
    // il compilatore non può capire da solo che tipo sarà. Dato che in questo esempio voglio riempire
    // il buffer con numeri casuali specifico il tipo: 'struct::<tipo>::new'
    let my_channel = Arc::new(MyChannel::<usize>::new(10));

    thread::scope(|s| {
        let mut producer_channel = Arc::clone(&my_channel);
        let consumer_channel = Arc::clone(&my_channel);
        let mut terminator_channel = Arc::clone(&my_channel);

        // producer
        s.spawn(move || {
            loop {
                let rnd = random_sleep(1, 2);
                thread::sleep(Duration::from_secs(rnd as u64));
                match producer_channel.write(Item::Value(rnd)) {
                    Ok(()) => println!("write value: {}", rnd),
                    Err(_) => {
                        println!("> Stopping the Producer");
                        break; // stop the producer thread
                    },
                }
            }
        });

        // consumer
        s.spawn(move || {
            loop {
                let rnd = random_sleep(3, 5);
                thread::sleep(Duration::from_secs(rnd as u64));
                match consumer_channel.read() {
                    Ok(value) => {
                        println!("read value: {}", value);
                    },
                    Err(_) => {
                        println!("> Buffer empty and producer stopped, stopping the Consumer..:");
                        break;
                    },
                }
            }
        });

        // terminator
        s.spawn(move || {
            thread::sleep(Duration::from_secs(random_sleep(15, 20) as u64));

            terminator_channel.close();
            println!("> Producer closed");
        });
    });
}