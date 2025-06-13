mod delayed_queue {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::sync::{Condvar, Mutex};
    use std::time::Instant;
    use std::marker::Send;

    // privata
    struct Item<T: Send> {
        t: T,
        i: Instant,
    }

    // voglio ordinare-> ord che necessita di partialOrd che a sua volta necessita di Eq e partialEq
    impl<T: Send> PartialEq for Item<T> {
        fn eq(&self, other: &Self) -> bool {
            self.i.eq(&other.i)
        }
    }
    
    impl<T: Send> Eq for Item<T>{} //uguaglianza riflessiva, oggetto uguale a se stesso
    
    impl<T:Send> PartialOrd for Item<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.i.partial_cmp(&self.i)
        }
    }
    
    impl<T:Send> Ord for Item<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.i.cmp(&self.i)
        }
    }

    pub struct DelayedQueue<T:Send> {
        // campi della struttura, binary heap
        data: Mutex<BinaryHeap<Item<T>>>,
        cv: Condvar,
    }

    impl<T:Send> DelayedQueue<T> {
        pub fn new() -> DelayedQueue<T> {
            Self {
                data: Mutex::new(BinaryHeap::new()),
                cv: Condvar::new(),
            }
        }

        pub fn offer(&self, t: T, i: Instant) {
            let mut data = self.data.lock().expect("mutex poisoned"); // come unwrap ma da messaggop personalizzato

            data.push(Item {t, i});
            drop(data); // implicito (ovv quando esco dallo scope) quando va bene notificare e poi rilasciare
            self.cv.notify_all(); // qui invece voglio prima lasciare il lock e poi notificare
        }

        pub fn take(&self) -> Option<T> {
            let mut data = self.data.lock().expect("mutex poisoned");
            loop {
                let now = Instant::now(); // guardo che ore sono.
                if let Some(item) = data.peek() { // sbircio la coda
                    let i = item.i; // che essendo copiabile lo copio e posso muoverlo
                    println!("Checking item expiring (i:{:?}) on Instant time (now:{:?})", i, now);
                    if i < now {
                        // time out superato dell'item
                        let res = data.pop().unwrap(); // unwrap garantito dall'if
                        return Some(res.t);
                    } else {
                        // addormento per raggiungere il time out restante
                        let d  = i.duration_since(now);
                        println!("Sleeping for {:?}", d);
                        data = self.cv.wait_timeout(data, i.duration_since(now)).expect("Mutex poisoned").0; // 0 per riprendere il lock
                    }
                } else {
                    // coda vuota
                    return None;
                }
            }
        }

        pub fn size(&self) -> usize {
            let data = self.data.lock().expect("mutex poisoned");
            data.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use std::time::{Duration, Instant};
    use crate::delayed_queue::DelayedQueue;

    #[test]
    fn an_empty_queue_return_none() {
        let q = DelayedQueue::<i32>::new(); // per specificare sin da subito T della struct cos'è

        assert_eq!(q.take(), None);
        assert_eq!(q.size(), 0);
    }

    #[test]
    fn items_are_returned_in_order() {
        let q = DelayedQueue::<i32>::new(); // per specificare sin da subito T della struct cos'è
        let now = Instant::now();
        q.offer(1500, now.add(Duration::from_millis(10)));
        q.offer(500, now.add(Duration::from_millis(5)));

        assert_eq!(q.take(), Some(500));
        assert_eq!(q.take(), Some(1500));
        assert_eq!(q.take(), None);
    }

    #[test]
    fn items_are_returned_in_orderedeven_if_insered_after_waiting_starts() {
        let q = DelayedQueue::<i32>::new();


        std::thread::scope(|s| {
            let now = Instant::now();
            q.offer(42, now.add(Duration::from_millis(10)));

            s.spawn(|| {
                assert_eq!(q.take(), Some(20));
            });
            s.spawn(|| {
                std::thread::sleep(Duration::from_millis(2));
                q.offer(20, Instant::now().add(Duration::from_millis(1)));
            });
        });
    }

    #[test]
    fn two_threads_reading_the_queque() {
        let q = DelayedQueue::<i32>::new();
        q.offer(1500, Instant::now().add(Duration::from_millis(10)));
        q.offer(500, Instant::now().add(Duration::from_millis(5)));
        std::thread::scope(|s| {
            for _ in 0..2 {
                s.spawn(|| {
                    let r = q.take();
                    assert!(r == Some(1500) || r == Some(500));
                });
            }
        });
    }
}
