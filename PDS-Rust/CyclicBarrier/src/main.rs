use std::sync::{Arc, Condvar, Mutex, MutexGuard};

struct Waiter {
    size: usize,
    counter: usize,
    external_barrier: bool
}

pub struct CyclicBarrier {
    data: Mutex<Waiter>,
    cv: Condvar,
}

impl CyclicBarrier {
    pub fn new(n: usize) -> CyclicBarrier {
        CyclicBarrier{ data: Mutex::new(Waiter{ size: n, counter: 0, external_barrier: true }), cv: Condvar::new(), }
    }

    fn toggle_barrier(mut data: MutexGuard<Waiter>) -> MutexGuard<Waiter> {
        data.counter = 0;
        data.external_barrier = !data.external_barrier;

        data
    }

    pub fn wait(&self) {
        let mut data = self.data.lock().expect("poisoned lock");

        //porta esterna
        /*while !data.external_barrier {
            data = self.cv.wait(data).unwrap(); // parametro da liberare e da riprendere
        }*/

        //thread fa count++
        data.counter += 1;
        println!("count ++");


        if data.counter % data.size == 0 {
            println!("sveglio tutti i thread e inverto");
            data.counter=0;
            //data = Self::toggle_barrier(data);
            self.cv.notify_all();
        } else {  }

        //se count != size mi addormento
        while data.counter < data.size {
            println!("mi addormento");
            data = self.cv.wait(data).unwrap(); // parametro da liberare e da riprendere
        }

        println!("sveglio");
        drop(data);
    }
}

fn main() {
    let abarrrier = Arc::new(CyclicBarrier::new(3));
    let mut vt = Vec::new();
    for i in 0..3 {
        let cbarrier = abarrrier.clone();
        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("after barrier {} {}", i, j);
            }
        }));
    }
    for t in vt {
        t.join().unwrap();
    }
}

