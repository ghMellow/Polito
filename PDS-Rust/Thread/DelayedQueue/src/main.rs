use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ptr::eq;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Item<T> {
    x: T,
    time: Instant,
}

pub struct DelayedQueue<T> {
    coda: Mutex<BinaryHeap<Item<T>>>,
    cv: Condvar
}

impl<T> DelayedQueue<T> {
    pub fn new() -> DelayedQueue<T> {
        DelayedQueue{coda: Mutex::new(BinaryHeap::new()), cv: Condvar::new()}
    }

    pub fn offer(&self, t:T, i: Instant) {
        let mut coda = self.coda.lock().unwrap();
        coda.push(Item{x: t, time: i}); // binary heap inserisce dati ordinati secondo a come li abbiamo implementati

        self.cv.notify_all(); // devo perchè se viene inserito un nuovo valore con tempo breve va servito lui
    }

    pub fn take(&self) -> Option<Item<T>> {
        if self.size() <= 0 {
            return None;
        }

        let mut coda = self.coda.lock().unwrap();
        loop {
            // if let Some(recent_value) = coda.pop() {
            //     let instant = Instant::now();
            //     let delta = recent_value.time - instant;
            //
            //     if delta <= Duration::from_secs(0) {
            //         drop(coda);
            //         self.cv.notify_all();
            //         return Some(recent_value);
            //     } else {
            //         // rimetto nel vettore, dato che pop. Con gli stessi valori
            //         self.offer(recent_value.x, recent_value.time);
            //         coda = self.cv.wait_timeout(coda, delta).unwrap().0; // 0 perchè wait_timeout restituisce tuple: (mutex, bool timeout result)
            //     }
            // }
            /// DeadLock togli e poi rimetti ma coda (mutex) rimane in possesso a take fino alla wait ma offer lo richiede!
            /// Bisogna usare peek() che permette di vedere senza estrarre il dato.
            /// ATTENZIONE: il metodo peek non è presente in Vec perciò bisogna usare BinaryHeap
            let first = coda.peek().unwrap();

            let now = Instant::now();
            if first.time < now {
                let item = coda.pop().unwrap();
                return Some(item);
            } else {
                let delta = first.time - now;
                coda = self.cv.wait_timeout(coda, delta).unwrap().0;
            }
        }
    }

    pub fn size(&self) -> usize {
        let coda = self.coda.lock().expect("coda mutex poisoned");
        coda.len()
    }
}


impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }

    fn ne(&self, other: &Self) -> bool {
        !eq(self, other)
    }
}

impl<T> Eq for Item<T> {}

impl<T> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time.partial_cmp(&self.time) // ordinata crescente!
    }
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)  // ordinata crescente!
    }
}


fn main() {
    let queue = Arc::new(DelayedQueue::new());
    let queue_producer = Arc::clone(&queue);
    let queue_consumer = Arc::clone(&queue);
    let current_time = Instant::now();
    // Thread Producer
    let producer_handle = thread::spawn(move || {
        println!("Producer: Adding items to the queue...");
        queue_producer.offer("Task A", Instant::now() + Duration::from_secs(3));
        queue_producer.offer("Task B", Instant::now() + Duration::from_secs(1));
        queue_producer.offer("Task C", Instant::now() + Duration::from_secs(4));
        queue_producer.offer("Task D", Instant::now() + Duration::from_secs(2));
        println!("Producer: Finished adding items.");
    });
    // Thread Consumer
    let consumer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        println!("Consumer: Starting to take items from the queue...");
        loop {
            if let Some(task) = queue_consumer.take() {
                println!("Consumer: Taken item: '{}' at {:?}", task.x, Instant::now()-current_time);
            } else {
                println!("Consumer: Queue is empty, nothing to take.");
                break;
            }
        }
        println!("Consumer: Finished taking items.");
    });
    producer_handle.join().expect("Producer thread panicked");
    consumer_handle.join().expect("Consumer thread panicked");
    println!("Main: All tasks completed. Final queue size: {}", queue.size());
}
