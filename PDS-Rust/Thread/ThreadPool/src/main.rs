// mod channel;

// use std::sync::{Arc, Condvar, Mutex};
// use std::thread;

// struct Item {
//     value: i32,
// }
// pub struct ThreadPool{
//     data: Mutex<Vec<Item>>,
//     cv: Condvar,
// }

// impl ThreadPool{
//     pub fn new() -> Self {
//         ThreadPool{ data: Mutex::new(Vec::new()), cv: Condvar::new() }
//     }

//     pub fn get_size(&self) -> usize{
//         let data = self.data.lock().unwrap();
//         data.len()
//     }
// }

// fn main() {
//     let thread_pool = Arc::new(ThreadPool::new());
//     let mut handles = vec![];

//     // producer
//     let producer = Arc::clone(&thread_pool);
//     let handle = thread::spawn(move || {
//         for i in 1..10{
//             let mut lock = producer.data.lock().expect("couldn't acquire lock");

//             lock.push(Item{value: i});
//             producer.cv.notify_one();
//             println!("producer #{}", i);
//         }
//     });
//     handles.push(handle);

//     for i in 1..10{
//         let reader = thread_pool.clone();//Arc::clone(&thread_pool);
//         let handle = thread::spawn(move || {
//             let mut guard = reader.data.lock().expect("couldn't acquire lock");

//             guard = reader.cv.wait_while(guard, |guard| {guard.len() == 0}).unwrap();
//             let val = guard.pop().unwrap();
//             println!("thread #{} pop value: {}", i, val.value);

//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }


//     //channel::run();
// }


// un produttore, n consumatori consumano con concorrenza il Job. Loop finchè produttore non termina.
pub mod thread_pool{
    use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};


    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub struct ThreadPool{
        sender: mpsc::Sender<Job>,
        workers: Vec<Worker>,
    }

    struct Worker{
        id: usize,
        job: JoinHandle<()>, // Handle del thread, ergo qui deposito un thread - () significa che non restituisce valori
    }

    impl ThreadPool{
        pub fn new(num_workers: usize) -> Self {
            let (sender, receiver) = mpsc::channel::<Job>();
            let rx = Arc::new(Mutex::new(receiver)); // rendo il ricevitore accessibile tra più thread concorrenti, dato che non posso clonarlo

            let mut workers = vec![];

            for id in 0..num_workers {
                let rx = rx.clone();
                let worker = Worker{ 
                    id: id, 
                    job: thread::spawn(move || { 
                        loop {
                            let ricevi_job = rx.lock().unwrap().recv(); // lock e mi metto in attesa di messaggio
                            if let Ok(job) = ricevi_job{
                                job()
                            } else {
                                println!("trasmettitore chiuso termino");
                                break;
                            }
                        }
                })};

                workers.push(worker);
            }

            ThreadPool { sender, workers }
        }

        pub fn send_job(&self, job: Job) {
            self.sender.send(job).unwrap();
        }
        
        pub fn shutdown(self) { // preso come possesso
            drop(self.sender);
            
            for worker in self.workers {
                worker.job.join().unwrap();
            }
        }
        
    }
}

use std::{thread, time::Duration};

use thread_pool::ThreadPool;
fn main() {
    let thread_pool = ThreadPool::new(3);


    thread_pool.send_job(Box::new(|| {
        println!("(job 1) start");
        thread::sleep(Duration::from_secs(1));
        println!("(job 1) finisched");
    }));

    thread_pool.send_job(Box::new(|| {
        println!("(job 2) start");
        thread::sleep(Duration::from_secs(1));
        println!("(job 2) finisched");
    }));

    thread_pool.send_job(Box::new(|| {
        println!("(job 3) start");
        thread::sleep(Duration::from_secs(1));
        println!("(job 3) finisched");
    }));

    thread_pool.shutdown();
}

