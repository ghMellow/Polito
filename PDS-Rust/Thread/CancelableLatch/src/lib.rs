// Un CancelableEvent è un meccanismo di sincronizzazione che permette ad un thread di attendere il completamento di un insieme di operazioni eseguite da altri thread, con la possibilità di cancellazione anticipata.

// Alla creazione, viene specificato il numero di operazioni da attendere. La struttura offre:

// - signal(): segnala il completamento di un'operazione
// - cancel(): annulla l'attesa (tutti i thread in attesa vengono svegliati immediatamente)
// - wait(): attende il completamento di tutte le operazioni o una cancellazione
// - wait_timeout(d: Duration): attende con un timeout massimo

pub mod cancelable_event {
    use std::{sync::{Condvar, Mutex}, time::{Duration, Instant}};


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum WaitResult {
    Completed,
    Timeout,
    Canceled,
}

pub trait CancelableEvent {
    fn new(count: usize) -> Self;

    fn signal(&self);

    fn cancel(&self);

    fn wait(&self) -> WaitResult;

    fn wait_timeout(&self, d: Duration) -> WaitResult;
}


struct Item {
    count: usize,
    status: Option<WaitResult>,
}
pub struct CancelableEventImpl {
    item: Mutex<Item>,
    cv: Condvar
}

impl CancelableEvent for CancelableEventImpl {
    fn new(count: usize) -> Self {
        CancelableEventImpl { item: (Mutex::new(Item{ count, status: None })), cv: (Condvar::new()) }
    }

    fn signal(&self) {
        let mut item = self.item.lock().unwrap();

        if item.status.is_none() {
            item.count -= 1;
            if item.count <= 0 {
                item.status = Some(WaitResult::Completed);
                self.cv.notify_all();
            }
        }        
    }

    fn cancel(&self) {
        let mut item = self.item.lock().unwrap();

        if item.status.is_none() {
            item.status = Some(WaitResult::Canceled);
            self.cv.notify_all();
        }
        
        drop(item);
    }

    fn wait(&self) -> WaitResult {
        let mut item = self.item.lock().unwrap();

        if item.count <= 0 {
            return WaitResult::Completed;
        }
        
        while item.status.is_none() {
            item = self.cv.wait(item).unwrap();
        }

        let status = item.status.unwrap(); // implementato tratto copy = al posto di .clone()
        return status;
    }

    fn wait_timeout(&self, d: Duration) -> WaitResult {
        let mut item = self.item.lock().unwrap();
        let mut is_timeout;

        if item.count <= 0 {
            return WaitResult::Completed;
        }

        let now = Instant::now();
        let mut time_left = d;
        while item.status.is_none() {
            (item, is_timeout) = self.cv.wait_timeout(item, time_left).unwrap();

            if is_timeout.timed_out() {
                item.status = Some(WaitResult::Timeout);
                return item.status.unwrap();
            } else {
                // tempo rimanente fatto su d e now fissi e elapsed per capire quanto tempo è passato
                let istant = now.elapsed();
                time_left = d.saturating_sub(istant); // gestisce automaticamente il caso in cui elapsed superi d portando a 0 time_left.
            }
        }

        item.status.expect("status dovrebbe essere impostato a questo punto")
    }
}

}

#[cfg(test)]
mod tests {
    use super::cancelable_event::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn test_completion() {
        let event = CancelableEventImpl::new(3);
        let event_ref = Arc::new(event);

        let mut handles = vec![];
        for i in 0..3 {
            let event_clone = Arc::clone(&event_ref);
            handles.push(thread::spawn(move || {
                thread::sleep(Duration::from_millis(50 * (i + 1)));
                event_clone.signal();
            }));
        }

        let event_clone = Arc::clone(&event_ref);
        let waiter = thread::spawn(move || event_clone.wait());

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(waiter.join().unwrap(), WaitResult::Completed);
    }

    #[test]
    fn test_cancellation() {
        let event = CancelableEventImpl::new(3);
        let event_ref = Arc::new(event);

        let event_clone = Arc::clone(&event_ref);
        let canceler = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            event_clone.cancel();
        });

        let event_clone = Arc::clone(&event_ref);
        let waiter = thread::spawn(move || event_clone.wait());

        canceler.join().unwrap();
        assert_eq!(waiter.join().unwrap(), WaitResult::Canceled);
    }

    #[test]
    fn test_timeout() {
        let event = CancelableEventImpl::new(10); // Numero alto che non verrà raggiunto
        let event_ref = Arc::new(event);

        let event_clone = Arc::clone(&event_ref);
        let waiter = thread::spawn(move || {
            event_clone.wait_timeout(Duration::from_millis(100))
        });

        assert_eq!(waiter.join().unwrap(), WaitResult::Timeout);
    }

    #[test]
    fn test_cancel_after_completion() {
        let event = CancelableEventImpl::new(1);
        event.signal(); // Completa immediatamente

        // Tentativo di cancellazione dopo il completamento
        event.cancel();

        // L'attesa dovrebbe comunque restituire Completed
        assert_eq!(event.wait(), WaitResult::Completed);
    }

    #[test]
    fn test_signal_after_cancel() {
        let event = CancelableEventImpl::new(3);
        event.cancel(); // Cancella prima dei segnali

        // Segnali dopo la cancellazione dovrebbero essere ignorati
        event.signal();
        event.signal();

        // L'attesa dovrebbe restituire Canceled
        assert_eq!(event.wait(), WaitResult::Canceled);
    }

    #[test]
    fn test_multiple_waiters() {
        let event = CancelableEventImpl::new(2);
        let event_ref = Arc::new(event);

        let mut waiters = vec![];
        for _ in 0..3 {
            let event_clone = Arc::clone(&event_ref);
            waiters.push(thread::spawn(move || {
                event_clone.wait()
            }));
        }

        thread::sleep(Duration::from_millis(100));
        event_ref.signal();
        event_ref.signal();

        for waiter in waiters {
            assert_eq!(waiter.join().unwrap(), WaitResult::Completed);
        }
    }

    #[test]
    fn test_accurate_timeout() {
        let event = CancelableEventImpl::new(10); // Numero alto che non verrà raggiunto
        let start = Instant::now();

        // Attesa con timeout breve
        let result = event.wait_timeout(Duration::from_millis(50));

        let elapsed = start.elapsed();
        assert_eq!(result, WaitResult::Timeout);
        assert!(elapsed >= Duration::from_millis(50));
        assert!(elapsed < Duration::from_millis(100));
    }

    #[test]
    fn test_zero_initial_count() {
        let event = CancelableEventImpl::new(0);
        assert_eq!(event.wait(), WaitResult::Completed);
        assert_eq!(
            event.wait_timeout(Duration::from_millis(100)),
            WaitResult::Completed
        );
    }

    #[test]
    fn test_negative_behavior() {
        let event = CancelableEventImpl::new(1);

        // Tentativo di over-signaling
        event.signal();
        event.signal(); // Dovrebbe essere ignorato

        // Verifica completamento corretto
        assert_eq!(event.wait(), WaitResult::Completed);
    }

    #[test]
    fn test_mixed_operations() {
        let event = CancelableEventImpl::new(3);
        let event_ref = Arc::new(event);

        let event_clone = Arc::clone(&event_ref);
        let signaler = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            event_clone.signal();
            thread::sleep(Duration::from_millis(50));
            event_clone.signal();
        });

        let event_clone = Arc::clone(&event_ref);
        let canceler = thread::spawn(move || {
            thread::sleep(Duration::from_millis(75));
            event_clone.cancel();
        });

        let result = event_ref.wait_timeout(Duration::from_millis(200));

        signaler.join().unwrap();
        canceler.join().unwrap();

        // Dovrebbe essere canceled perché la cancellazione arriva prima del terzo signal
        assert_eq!(result, WaitResult::Canceled);
    }
}