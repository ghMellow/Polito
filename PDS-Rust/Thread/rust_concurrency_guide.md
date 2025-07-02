# 🦀 Guida Completa: Rust Concurrency & Metodi Utili

## 📡 MPSC (Message Passing)

### Receiver Methods
```rust
// Bloccanti
rx.recv()                    // Aspetta messaggio per sempre
rx.recv_timeout(duration)    // Aspetta con timeout

// Non-bloccanti  
rx.try_recv()               // Ritorna subito Ok/Err
rx.try_iter()               // Iterator sui messaggi disponibili

// Iterator
for msg in rx.iter() { }    // Loop finché canale aperto
```

### Sender Methods
```rust
tx.send(value)              // Invia (può bloccare su bounded)
tx.try_send(value)          // Non-bloccante (solo bounded)
```

### Esempio Pratico: MultiChannel
```rust
pub enum Message {
    Data(u8),
    Shutdown,
}

pub struct MultiChannel {
    senders: Mutex<Vec<Sender<Message>>>,
    creator_thread_id: ThreadId, // Solo creatore può shutdown
}

impl MultiChannel {
    pub fn send(&self, data: u8) -> Result<(), SendError<Message>> {
        let mut senders = self.senders.lock().unwrap();
        
        // Pattern: Cleanup automatico sender morti
        senders.retain(|sender| {
            sender.send(Message::Data(data)).is_ok()
        });
        
        Ok(())
    }
    
    pub fn shutdown(&self) -> Result<(), SendError<Message>> {
        // Pattern: Autorizzazione thread-based
        if thread::current().id() != self.creator_thread_id {
            return Err(SendError(Message::Shutdown));
        }
        
        let mut senders = self.senders.lock().unwrap();
        senders.retain(|sender| sender.send(Message::Shutdown).is_ok());
        senders.clear();
        Ok(())
    }
}
```

## 🧵 Thread Management

### Thread Creation & Control
```rust
// Creazione
let handle = thread::spawn(|| { /* work */ });
let handle = thread::spawn(move || { /* with ownership */ });

// Controllo
handle.join().unwrap();     // Aspetta terminazione
handle.is_finished();       // Check non-bloccante

// Info thread
thread::current().id();     // ID univoco
thread::current().name();   // Nome del thread

// Pausa e yield
thread::sleep(duration);    // Pausa thread
thread::yield_now();        // Cede CPU volontariamente
```

### Pattern: Thread Pool con Cleanup
```rust
// Rimuovi thread terminati
threads.retain(|handle| {
    match handle.try_join() {
        Ok(_) => false,     // Thread finito, rimuovi
        Err(_) => true,     // Thread attivo, mantieni
    }
});
```

## ⏱️ Time Management

### Duration
```rust
Duration::from_secs(5)      // 5 secondi
Duration::from_millis(100)  // 100 millisecondi
Duration::from_micros(50)   // 50 microsecondi

// Operazioni
d1 + d2                     // Somma durate
d1 * 2                      // Moltiplicazione
duration.as_millis()        // Conversione
```

### Instant
```rust
let start = Instant::now();
let elapsed = start.elapsed();       // Durata trascorsa
let since = start.duration_since(other); // Durata tra instant
```

### Pattern: Timeout Operations
```rust
match rx.recv_timeout(Duration::from_secs(5)) {
    Ok(msg) => process_message(msg),
    Err(_) => println!("Timeout scaduto!"),
}
```

### Pattern: Rate Limiting
```rust
let mut last_execution = Instant::now();
if last_execution.elapsed() > Duration::from_millis(100) {
    execute_rate_limited_task();
    last_execution = Instant::now();
}
```

## 📦 Vec & Collections

### Filtri e Pulizia
```rust
// ⭐ retain - Mantieni elementi che soddisfano condizione
vec.retain(|x| *x > 5);

// Rimuovi duplicati
vec.dedup();
vec.dedup_by(|a, b| a.id == b.id);

// Ordinamento
vec.sort();
vec.sort_by(|a, b| a.priority.cmp(&b.priority));
```

### Ricerca e Accesso
```rust
vec.contains(&value);           // Controlla presenza
vec.position(|x| *x == target); // Prima posizione
vec.binary_search(&value);      // Ricerca binaria (ordinato)
vec.get(index);                 // Accesso sicuro (Option)
```

### Divisione e Processamento
```rust
vec.chunks(size);               // Batch processing
vec.windows(size);              // Finestre scorrevoli
vec.split_at(index);            // Dividi in due parti
```

### Pattern: Batch Processing
```rust
for batch in messages.chunks(100) {
    process_batch(batch);
    thread::sleep(Duration::from_millis(10)); // Rate limiting
}
```

## 🔄 Iterator Patterns

### Trasformazioni Comuni
```rust
iter.map(|x| x * 2)                    // Trasforma
iter.filter(|x| **x > threshold)      // Filtra
iter.filter_map(|x| try_convert(x))   // Filtra + trasforma
iter.enumerate()                       // Aggiungi indici
iter.zip(other_iter)                   // Combina iteratori
```

### Aggregazioni
```rust
iter.fold(0, |acc, x| acc + x);       // Accumula con iniziale
iter.reduce(|acc, x| acc.max(x));     // Accumula senza iniziale
iter.collect::<Vec<_>>();             // Raccogli in collezione
iter.partition(|x| *x > 5);           // Dividi in due gruppi
```

### Pattern: Data Processing Pipeline
```rust
let results: Vec<_> = data
    .iter()
    .filter(|item| item.is_valid())
    .map(|item| item.process())
    .filter_map(|result| result.ok())
    .collect();
```

## 🔐 Synchronization Primitives

### Mutex
```rust
let guard = mutex.lock().unwrap();    // Blocca
let guard = mutex.try_lock();         // Non-bloccante
if mutex.is_poisoned() { /* handle */ } // Check corruption
```

### RwLock
```rust
let read_guard = rwlock.read().unwrap();   // Lettura
let write_guard = rwlock.write().unwrap(); // Scrittura
let read_guard = rwlock.try_read();        // Non-bloccante
```

### Condvar
```rust
// Aspetta notifica
let guard = condvar.wait(guard).unwrap();

// Aspetta con timeout
let (guard, timeout) = condvar.wait_timeout(guard, duration).unwrap();

// Aspetta condizione specifica
let guard = condvar.wait_while(guard, |state| !state.ready).unwrap();

// Notifica
condvar.notify_one();   // Un thread
condvar.notify_all();   // Tutti i thread
```

### Atomic Operations
```rust
atomic.load(Ordering::Relaxed);              // Leggi
atomic.store(value, Ordering::Relaxed);      // Scrivi
atomic.swap(new_val, Ordering::Relaxed);     // Scambia
atomic.compare_and_swap(old, new, ordering); // CAS
atomic.fetch_add(increment, ordering);       // Somma atomica
```

## 🏗️ Design Patterns

### 1. Producer-Consumer con Shutdown
```rust
enum WorkerMessage {
    Task(Task),
    Shutdown,
}

// Producer
tx.send(WorkerMessage::Task(task)).unwrap();
tx.send(WorkerMessage::Shutdown).unwrap(); // Graceful shutdown

// Consumer
loop {
    match rx.recv() {
        Ok(WorkerMessage::Task(task)) => process_task(task),
        Ok(WorkerMessage::Shutdown) => break,
        Err(_) => break, // Channel closed
    }
}
```

### 2. Thread-Safe Resource Pool
```rust
pub struct ResourcePool<T> {
    resources: Mutex<Vec<T>>,
    creator_thread: ThreadId,
}

impl<T> ResourcePool<T> {
    pub fn acquire(&self) -> Option<T> {
        self.resources.lock().unwrap().pop()
    }
    
    pub fn release(&self, resource: T) {
        self.resources.lock().unwrap().push(resource);
    }
    
    pub fn cleanup(&self) {
        if thread::current().id() == self.creator_thread {
            self.resources.lock().unwrap().clear();
        }
    }
}
```

### 3. Event-Driven System
```rust
#[derive(Debug, Clone)]
pub enum SystemEvent {
    UserAction(UserId, Action),
    SystemShutdown,
    HealthCheck,
}

pub struct EventSystem {
    subscribers: Mutex<Vec<Sender<SystemEvent>>>,
}

impl EventSystem {
    pub fn publish(&self, event: SystemEvent) {
        let mut subscribers = self.subscribers.lock().unwrap();
        
        // Pattern: Auto-cleanup dead subscribers
        subscribers.retain(|tx| tx.send(event.clone()).is_ok());
    }
}
```

## 🎯 Best Practices

### 1. Error Handling
```rust
// ✅ Graceful degradation
if let Err(_) = channel.send(message) {
    log::warn!("Receiver disconnected, message dropped");
    // Continue operation instead of panicking
}

// ✅ Timeout operations
match operation_with_timeout(Duration::from_secs(30)) {
    Ok(result) => handle_success(result),
    Err(_) => handle_timeout(),
}
```

### 2. Resource Management
```rust
// ✅ RAII pattern with scope guards
{
    let _guard = resource.lock().unwrap();
    // Resource automatically released when guard drops
} // ← Resource released here
```

### 3. Shutdown Coordination
```rust
// ✅ Graceful shutdown pattern
pub struct System {
    shutdown_tx: Option<Sender<()>>,
    worker_handles: Vec<JoinHandle<()>>,
}

impl System {
    pub fn shutdown(&mut self) {
        // Signal all workers
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()); // Ignore errors
        }
        
        // Wait for workers
        for handle in self.worker_handles.drain(..) {
            let _ = handle.join(); // Ignore panics
        }
    }
}
```

## 🚀 Performance Tips

### 1. Minimize Lock Contention
```rust
// ❌ Bad: Long critical section
let mut data = shared_data.lock().unwrap();
let result = expensive_computation(&data);
data.update(result);

// ✅ Good: Short critical section
let snapshot = {
    let data = shared_data.lock().unwrap();
    data.clone()
};
let result = expensive_computation(&snapshot);
{
    let mut data = shared_data.lock().unwrap();
    data.update(result);
}
```

### 2. Batch Operations
```rust
// ✅ Process in batches for better performance
for batch in work_items.chunks(1000) {
    process_batch(batch);
    
    // Optional: yield CPU between batches
    if should_yield() {
        thread::yield_now();
    }
}
```

### 3. Use Appropriate Synchronization
```rust
// ✅ RwLock for read-heavy workloads
let config = Arc::new(RwLock::new(Config::new()));

// Many readers
let config_ref = config.read().unwrap();
let setting = config_ref.get_setting("key");

// Occasional writer
let mut config_ref = config.write().unwrap();
config_ref.update_setting("key", "value");
```

## 📚 Common Patterns Cheat Sheet

| Pattern | Code | Use Case |
|---------|------|----------|
| Cleanup Dead Connections | `connections.retain(\|c\| c.is_alive())` | Network servers |
| Timeout Operations | `rx.recv_timeout(duration)` | Robust services |
| Batch Processing | `data.chunks(size)` | High throughput |
| Rate Limiting | `last.elapsed() > interval` | API throttling |
| Graceful Shutdown | `tx.send(Shutdown)` | Clean termination |
| Thread Authorization | `thread::current().id()` | Security control |
| Resource Pooling | `pool.acquire() / pool.release()` | Database connections |
| Event Broadcasting | `senders.retain(\|tx\| tx.send().is_ok())` | Pub/Sub systems |

---

*Generated by Claude - Rust Concurrency Guide v1.0*