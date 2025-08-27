/*
Un componente con funzionalità di cache permette di ottimizzare il comportamento di un sistema riducendo il numero di volte 
in cui una funzione è invocata, tenendo traccia dei risultati da essa restituiti a fronte di un particolare dato in ingresso.
Per generalità, si assuma che la funzione accetti un dato di tipo generico K e restituisca un valore di tipo generico V.

Il componente offre un unico metodo get(...) che prende in ingresso due parametri, il valore k (di tipo K, clonabile) del parametro e 
la funzione f (di tipo K -> V) responsabile della sua trasformazione, e restituisce uno smart pointer clonabile al relativo valore.
Se, per una determinata chiave k, non è ancora stato calcolato il valore corrispondente, la funzione viene invocata e ne viene restituito 
il risultato; altrimenti viene restituito il risultato già trovato. Il componente cache deve essere thread-safe perché due o più thread 
possono richiedere contemporaneamente il valore di una data chiave: quando questo avviene e il dato non è ancora presente, 
la chiamata alla funzione dovrà essere eseguita nel contesto di UN SOLO thread, mentre gli altri dovranno aspettare il risultato in corso di 
elaborazione, SENZA CONSUMARE cicli macchina.
*/

use std::{collections::HashMap, sync::{Arc, Condvar, Mutex}};
use std::hash::Hash;


pub enum ValueState<V>{
    Ready(Arc<V>),
    Pending,
}

pub struct Cache<K: Clone+Hash+Eq+PartialEq, V> {
    memoria: Mutex<HashMap<K, ValueState<V>>>,
    cv: Condvar,
}

impl<K: Clone+Hash+Eq+PartialEq, V> Cache<K, V> {
    pub fn new() -> Self {
        Cache { 
            memoria: Mutex::new(HashMap::new()), 
            cv: Condvar::new(),
        }
    }

    //pub fn get(&self, k: K, f: impl FnOnce(&K)->V) -> V {
    //pub fn get<F: FnOnce(&K)->V> (&self, k: K, f: F) -> V {
    pub fn get<F> (&self, k: K, f: F) -> Arc<V> 
    where F:FnOnce(&K)->V{
        let mut memoria = self.memoria.lock().unwrap();

        loop{
            match memoria.get(&k){
                Some(ValueState::Ready(v)) => {
                    return v.clone();
                },
                Some(ValueState::Pending) => {
                    memoria = self.cv.wait_while(memoria, |memoria| {
                        match memoria.get(&k){
                            Some(ValueState::Ready(_)) => {return false;},
                            Some(ValueState::Pending) => {return true;},
                            None => {panic!("chiave dovrebbe essere presente in questo punto!")},
                        }
                    }).unwrap();
                    continue;
                },
                None => {
                    memoria.insert(k.clone(), ValueState::Pending);
                    drop(memoria); // creo valore e droppo mutex così se ho una successiva richiesta lo addormento

                    // procedo con il creare e notificare
                    let mut memoria = self.memoria.lock().unwrap();
                    let res = Arc::new(f(&k));

                    memoria.insert(k.clone(), ValueState::Ready(res.clone())); // insert aggiunge e se presente modifica
                    self.cv.notify_all();

                    return res.clone();
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
