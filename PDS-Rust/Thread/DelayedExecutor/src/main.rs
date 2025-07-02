use std::{collections::BinaryHeap, sync::{Arc, Condvar, Mutex}, thread, time::{Duration, Instant}};


// alias dell'oggetto tratto
type Funzione = Box<dyn FnOnce()+Send+'static>; // send se 

struct Item {
    delay: Duration,
    //f: Box<dyn FnOnce()+Send+'static>, versione esplicita
    f: Funzione // versione con alias
}

impl PartialEq for Item{
    fn eq(&self, other: &Self) -> bool {
        self.delay == other.delay
    }
}
impl Eq for Item{}
// other - self , invertiti, poichè binaryheap ha nella testa il valore più grande e voglio il più piccolo
impl PartialOrd for Item{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.delay.partial_cmp(&self.delay)
    }
}
impl Ord for Item{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.delay.cmp(&self.delay)
    }
}


struct DelayedExecutor {
    coda: Mutex<BinaryHeap<Item>>,
    stato: Mutex<bool>,
    cv: Condvar,
}

impl DelayedExecutor {
    pub fn new() -> Self{
        DelayedExecutor { coda: Mutex::new(BinaryHeap::new()), stato: Mutex::new(true), cv: Condvar::new() }
    }

    //pub fn execute<F: FnOnce()+Send+'static>(&self, f:F, delay: Duration) -> bool {
    pub fn execute(&self, f: impl FnOnce()+Send+'static, delay: Duration) -> bool { 
    //pub fn execute(&self, f:Funzione, delay: Duration) -> bool {
        let stato = self.stato.lock().unwrap();
        let stato_delayexecutor = *stato;
        drop(stato);

        if stato_delayexecutor == true {
            let mut coda = self.coda.lock().unwrap();
            
            coda.push(Item{delay, f: Box::new(f)}); // in cima sempre quello con delay minore, ragionamenti successivi rimangono validi
            //coda.push(Item{delay, f});
            
            let instant_for_each_thread = Instant::now(); //ogni thread che esegue la funizone avrà questa var locale e perciò il tempo trascorso rispetto a delay.
            let mut is_timeout;
            let mut remaining_delay = delay;

            while instant_for_each_thread.elapsed() < delay {
                (coda, is_timeout) = self.cv.wait_timeout(coda, remaining_delay).unwrap();

                remaining_delay = delay.saturating_sub(instant_for_each_thread.elapsed());
                if is_timeout.timed_out() {
                    if let Some(item) = coda.pop() {
                        (item.f)(); // eseguo funzione
                    }
                }
            } 
        }

        return stato_delayexecutor;
    }

    pub fn close(&self, drop_pending_tasks: bool) {
        let mut stato = self.stato.lock().unwrap();
        *stato = !(*stato);
        drop(stato);

        if drop_pending_tasks == true {
            let mut coda = self.coda.lock().unwrap();
            coda.clear();
        }
    }

}

fn main() {
    let delayed_executor = Arc::new(DelayedExecutor::new());
    let mut handlers = vec![];

    for i in 0..=4 {
        let d1 = delayed_executor.clone();
        handlers.push(thread::spawn(move || {
            let mut id = i;
            if id == 0{
                //thread::sleep(Duration::from_secs(4));
                id = 4;
            }
            if id == 3 {
                d1.close(false);
            }


            //let res = d1.execute(move || println!("esecuzioni job {}", id), Duration::from_secs(1 + id));
            let res = d1.execute(Box::new(move || println!("esecuzioni job {}", id)), Duration::from_secs(1 + id));
            println!("risutlato esecuzione {}: {}", i, res);
        }));

        thread::sleep(Duration::from_secs(1));
        // if i == 3 {
        //     delayed_executor.close(false);
        // }
    }

    for handler in handlers{
        handler.join().unwrap();
    }
}
