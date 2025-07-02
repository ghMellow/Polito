use std::time::{Instant, Duration};
use std::sync::{Condvar, Mutex, Arc};
use std::thread;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]
enum StatoToken{
    Empty,
    Pending,
    Valid
}


/*
Questo pattern si chiama trait object ed è una tecnica fondamentale per il polimorfismo dinamico in Rust.
È un modo per dire: "Voglio un tipo che può essere qualsiasi cosa che implementi questo comportamento, ma non so a compile-time quale sarà".
Vantaggi principali:
    Flessibilità: Puoi cambiare comportamento a runtime
    Testabilità: Mock facili da implementare
    Separazione delle responsabilità: Il manager non sa come ottenere il token
    Estensibilità: Aggiungi nuove strategie senza modificare il codice esistente
*/
type TokenAcquirer = dyn Fn() -> Result<(String, Instant), String> + Send + Sync;


struct Item{
    stato: StatoToken,
    token: Option<String>,
}



struct TokenManager {
    item: Mutex<Item>,
    function: Box<TokenAcquirer>,
    cv: Condvar,
}

impl TokenManager {
    pub fn new(acquire_token: Box<TokenAcquirer> ) -> Self {
        TokenManager { item: Mutex::new(Item{ stato: StatoToken::Empty, token: None }), function: Box::new(acquire_token), cv: Condvar::new() }
    }

    pub fn get_token(&self) -> Result<String, String> {
        loop{
            let mut item = self.item.lock().expect("Failed to lock token mutex");

            match item.stato {
                StatoToken::Empty => {
                    item.stato = StatoToken::Pending;
                    if let Some(token) = self.try_get_token(){
                        item.stato = StatoToken::Valid;
                        item.token = Some(token);
                    } else {
                        item.stato = StatoToken::Empty;
                        return Err("failure".to_string());
                    }
                },
                StatoToken::Pending => {
                    while item.stato != StatoToken::Valid {
                        item = self.cv.wait(item).unwrap();
                    }
                    continue;
                },
                StatoToken::Valid => {
                    return Ok(item.token.clone().unwrap());
                },
            }
        }
    }
    pub fn try_get_token(&self) -> Option<String>{        
        let time_now = Instant::now();
        if let Ok((token, token_time)) = (self.function)(){
            if token_time > time_now {
                return Some(token);
            }
        }
        
        return None;
    }
}

#[test]
fn a_new_manager_contains_no_token() {
    let a: Box<TokenAcquirer> = Box::new(|| Err("failure".to_string()));
    let manager = TokenManager::new(a);
    assert!(manager.try_get_token().is_none());
}
#[test]
fn a_failing_acquirer_always_returns_an_error() {
    let a: Box<TokenAcquirer> = Box::new(|| Err("failure".to_string()));
    let manager = TokenManager::new(a);
    assert_eq!(manager.get_token(), Err("failure".to_string()));
    assert_eq!(manager.get_token(), Err("failure".to_string()));
}
#[test]
fn a_successful_acquirer_always_returns_success() {
    let a: Box<TokenAcquirer> = Box::new(|| Ok( ("asdafdggd132342".to_string(), Instant::now() + Duration::from_secs(10)) ));
    let manager = TokenManager::new(a);
    
    let result = manager.get_token();
    assert!(result.is_ok());
    
    let token = result.unwrap();
    assert!(!token.is_empty()); 
    assert_eq!(token, "asdafdggd132342"); 
}

#[test]
fn a_slow_acquirer_causes_other_threads_to_wait() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Barrier;

    let barrier = Arc::new(Barrier::new(2));
    let acquirer_called = Arc::new(AtomicBool::new(false));

    let acquirer_called_clone = acquirer_called.clone();
    let barrier_clone = barrier.clone();

    let acquirer: Box<TokenAcquirer> = Box::new(move || {
        acquirer_called_clone.store(true, Ordering::SeqCst);
        // Notifica che il primo thread è entrato nell'acquirer
        barrier_clone.wait();
        // Simula lentezza
        std::thread::sleep(Duration::from_millis(300));
        Ok(("token_slow".to_string(), Instant::now() + Duration::from_secs(5)))
    });

    let manager = Arc::new(TokenManager::new(acquirer));

    let manager1 = manager.clone();
    let handle1 = thread::spawn(move || {
        manager1.get_token().unwrap()
    });

    // Aspetta che il primo thread entri nell'acquirer
    barrier.wait();

    let manager2 = manager.clone();
    let start = Instant::now();
    let handle2 = thread::spawn(move || {
        manager2.get_token().unwrap()
    });

    let token1 = handle1.join().unwrap();
    let token2 = handle2.join().unwrap();
    let elapsed = start.elapsed();

    // Entrambi devono ricevere lo stesso token
    assert_eq!(token1, "token_slow");
    assert_eq!(token2, "token_slow");
    // Il secondo thread deve aver aspettato almeno ~300ms
    assert!(elapsed >= Duration::from_millis(290));
}
