use std::ptr::null;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::Sender;
use std::thread;


/**

++ SUPER COMPLICATO USARE I CANALI QUI, DATO CHE PER NATURA PER CANALE SI HA 1 RICEVITORE.

*/

pub struct Item{
    value: i32,
}

pub struct ThreadPool{
    producer: std::sync::mpsc::Sender<Item>,
    consumers: Vec<std::sync::mpsc::Receiver<Item>>
}

impl ThreadPool{
    pub fn new(tx: std::sync::mpsc::Sender<Item>, rxs: Vec<std::sync::mpsc::Receiver<Item>>)->Self{
        ThreadPool{producer: tx, consumers:rxs}
    }
}

pub fn thread_pool() -> ThreadPool { //Result<ThreadPool, std::io::Error> {
    let mut s = Option::None;
    let mut rxs = Vec::new();

    for i in 0..10 {
        let (tx, rx) = std::sync::mpsc::channel();

        s = Some(tx); // sovrascritto e mi piglio l'ultimo
        rxs.push(rx);
    }

    ThreadPool::new(s.unwrap(), rxs)
}

pub fn run() {
    println!();
    println!("lunch of the channel version");


}