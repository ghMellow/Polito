use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};


pub struct Barrier{
    barrier_size: usize,
    actual_number_thread: Mutex<(usize, bool)>,
    cv: Condvar,
}

impl Barrier{
    pub fn new(size: usize) -> Result<Barrier, usize>{
        if size < 2 {
            return Err(0 as usize);
        }
        Ok(Barrier { barrier_size: size, actual_number_thread: Mutex::new((0, false)), cv: Condvar::new() })
    }

    pub fn wait(&self) -> usize{
        let mut actual_number_thread = self.actual_number_thread.lock().unwrap();

        actual_number_thread.0 +=  1;
        let rank = actual_number_thread.0;
        let stato_barriera =  actual_number_thread.1;

        if actual_number_thread.0 == self.barrier_size {
            actual_number_thread.0 +=  0;
            actual_number_thread.1 = !stato_barriera; // generico così funziona sia in apertura che chiusura
            self.cv.notify_all();
        } else {
            while actual_number_thread.1 == stato_barriera {
                actual_number_thread = self.cv.wait(actual_number_thread).unwrap();
            }
        }

        rank
    }
}

fn main() {
    let size = 3;
    let ranking_barrier = Arc::new(Barrier::new(size).unwrap());
    let mut handlers = vec![];

    for j in 0..2 {
        println!("\nCiclo barriera numero {}", j);
        for i in 0..size {
            let b = ranking_barrier.clone();
            handlers.push(thread::spawn(move || {
                let res = b.wait();
                println!("{} esce dalla barriera {}", i, res);
            }));
        }

        thread::sleep(Duration::from_secs(1));
    }
        

    // let b1 = ranking_barrier.clone();
    // handlers.push(thread::spawn(move || {
    //     let res = b1.wait();
    //     println!("(b1) esce dalla barriera {}", res);
    // }));

    // let b2 = ranking_barrier.clone();
    // handlers.push(thread::spawn(move || {
    //     let res = b2.wait();
    //     println!("(b2) esce dalla barriera {}", res);
    // }));

    // let b3 = ranking_barrier.clone();
    // handlers.push(thread::spawn(move || {
    //     let res = b3.wait();
    //     println!("(b3) esce dalla barriera {}", res);
    // }));

    // let b4 = ranking_barrier.clone();
    // handlers.push(thread::spawn(move || {
    //     //thread::sleep(Duration::from_secs(1));
    //     let res = b4.wait();
    //     println!("(b4) esce dalla barriera {}", res);
    // }));
    

    for handler in handlers{
        handler.join().unwrap();
    }
}