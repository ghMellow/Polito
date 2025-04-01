pub mod complex_number;

pub mod circular_buffer {
    #[derive(Debug)]
    pub struct CircularBuffer<T> {
        buffer: Vec<Option<T>>,    // vector of option: None value or Some(value) which is of type generic
        capacity: usize,    // dimension of the circular buffer
        head: usize,    // read index
        tail: usize,    // write index
        size: usize,    // number of elements in the buffer
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        FullBuffer,
    }

    impl<T> CircularBuffer<T> {
        pub fn new(capacity: usize) -> Self {
            let mut buffer = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                buffer.push(None);
            }

            CircularBuffer {
                buffer,
                capacity,
                head: 0,
                tail: 0,
                size: 0,
            }
        }

        pub fn write(&mut self, item: T) -> Result<(), Error> {
            if self.size == self.capacity {
                return Err(Error::FullBuffer);
            }

            // 'Some' incapsula il valore di item in un oggetto Option, perchè il vettore è un
            // vettore di Option<T>. Spiegazione del perchè non viene inizializzato come Vec<T> nella fn read().
            self.buffer[self.tail] = Some(item);
            // L'operatore modulo (%) viene utilizzato per implementare il comportamento "circolare" del buffer.
            // Quando un indice raggiunge la fine del buffer (capacity), l'operazione (indice + 1) % capacity
            // fa "avvolgere" l'indice riportandolo all'inizio (0).
            self.tail = (self.tail + 1) % self.capacity;
            self.size += 1;

            Ok(())
        }

        pub fn read(&mut self) -> Option<T> {
            if self.size == 0 {
                return None;
            }

            // Motivo per cui il vettore è di Vec<Option<T>> e non direttamente di Vec<T>
            // Option è un oggetto a due stati: Some(valore) e None, questo mi permette di indicare
            // se la posizione nel buffer è libera oppure occupata. 'take()' infatti prende il valore
            // e lo rimpiazza con None.
            // Modi di estrazione di un valore: unwrap(), take(), pattern matching.
            let item = self.buffer[self.head].take();
            self.head = (self.head + 1) % self.capacity;
            self.size -= 1;

            item
        }

        pub fn clear(&mut self) {
            for i in 0..self.capacity {
                self.buffer[i] = None;
            }
            self.head = 0;
            self.tail = 0;
            self.size = 0;
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn overwrite(&mut self, item: T) {
            if self.size < self.capacity {
                // If the buffer isn't full, just do a normal write
                // 'unwrap' estrae il valore da un Option<T> o Result<T, E> esistente (causando panic se è None o Err)
                self.write(item).unwrap();
            } else {
                // If the buffer is full, overwrite the oldest item (at head)
                self.buffer[self.head] = Some(item);
                self.head = (self.head + 1) % self.capacity;
                self.tail = (self.tail + 1) % self.capacity;
                // Size remains the same as we're replacing an element
            }
        }

        pub fn make_contiguous(&mut self) {
            if self.head == 0 || self.size == 0 {
                // Already contiguous or empty
                return;
            }

            let mut temp_buffer = Vec::with_capacity(self.capacity);

            // First, collect all elements in order
            for _ in 0..self.capacity {
                temp_buffer.push(None);
            }

            // Copy elements in their logical order
            for i in 0..self.size {
                let index = (self.head + i) % self.capacity;
                temp_buffer[i] = self.buffer[index].take();
            }

            // Update indices
            self.buffer = temp_buffer;
            self.head = 0;
            self.tail = self.size % self.capacity;
        }
    }
}

pub mod circular_buffer_heterogenous {
    use std::fmt::Debug;
    use std::any::Any;
    use std::ops::{Deref, Index, IndexMut};

    // Trait che tutti i tipi, singoli elementi, nel buffer circolare devono implementare
    pub trait BufferItem: Debug + Any {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
        fn clone_box(&self) -> Box<dyn BufferItem>;
    }

    // Implementazione automatica per i tipi che soddisfano i requisiti
    impl<T: 'static + Debug + Clone> BufferItem for T {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn clone_box(&self) -> Box<dyn BufferItem> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug)]
    pub struct CircularBufferHeterogenous {
        buffer: Vec<Option<Box<dyn BufferItem>>>,
        capacity: usize,
        head: usize,
        tail: usize,
        size: usize,
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        FullBuffer,
    }

    impl CircularBufferHeterogenous {
        pub fn new(capacity: usize) -> Self {
            let mut buffer = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                buffer.push(None);
            }

            CircularBufferHeterogenous {
                buffer,
                capacity,
                head: 0,
                tail: 0,
                size: 0,
            }
        }

        pub fn write<T: 'static + BufferItem>(&mut self, item: T) -> Result<(), Error> {
            if self.size == self.capacity {
                return Err(Error::FullBuffer);
            }

            self.buffer[self.tail] = Some(Box::new(item));
            self.tail = (self.tail + 1) % self.capacity;
            self.size += 1;

            Ok(())
        }

        pub fn read(&mut self) -> Option<Box<dyn BufferItem>> {
            if self.size == 0 {
                return None;
            }

            let item = self.buffer[self.head].take();
            self.head = (self.head + 1) % self.capacity;
            self.size -= 1;

            item
        }

        // Gli altri metodi rimangono simili...

        pub fn clear(&mut self) {
            for i in 0..self.capacity {
                self.buffer[i] = None;
            }
            self.head = 0;
            self.tail = 0;
            self.size = 0;
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn overwrite<T: 'static + BufferItem>(&mut self, item: T) {
            if self.size < self.capacity {
                // If the buffer isn't full, just do a normal write
                // 'unwrap' estrae il valore da un Option<T> o Result<T, E> esistente (causando panic se è None o Err)
                self.write(item).unwrap();
            } else {
                // If the buffer is full, overwrite the oldest item (at head)
                self.buffer[self.head] = Some(Box::new(item));
                self.head = (self.head + 1) % self.capacity;
                self.tail = (self.tail + 1) % self.capacity;
                // Size remains the same as we're replacing an element
            }
        }

        pub fn make_contiguous(&mut self) {
            if self.head == 0 || self.size == 0 {
                // Already contiguous or empty
                return;
            }

            let mut temp_buffer = Vec::with_capacity(self.capacity);

            // First, collect all elements in order
            for _ in 0..self.capacity {
                temp_buffer.push(None);
            }

            // Copy elements in their logical order
            for i in 0..self.size {
                let index = (self.head + i) % self.capacity;
                temp_buffer[i] = self.buffer[index].take();
            }

            // Update indices
            self.buffer = temp_buffer;
            self.head = 0;
            self.tail = self.size % self.capacity;
        }
    }

    // Altre funzionalità richieste. Implementazione personalizzata dell'accesso ai valori del
    // vettore tramite indice: buf[i] restituirà il valore del buffer al valore di head + offset
    // passato come parametro.
    impl Index<usize> for CircularBufferHeterogenous {
        type Output = Box<dyn BufferItem>;
        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.size {
                panic!("Index out of bounds");
            }

            let actual_index = (self.head + index) % self.capacity;

            match &self.buffer[actual_index] {
                Some(item) => item,
                None => panic!("Trying to access an empty slot in the buffer"),
            }

            // Alternativa valida quando self è prestato (&self) infatti unwrap ne richiede il passaggio di possesso!
            // self.buffer[actual_index].as_ref().unwrap()
        }
    }
    impl IndexMut<usize> for CircularBufferHeterogenous {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if index >= self.size {
                panic!("Index out of bounds");
            }

            let actual_index = (self.head + index) % self.capacity;

            match &mut self.buffer[actual_index] {
                Some(item) => item,
                None => panic!("Trying to access an empty slot in the buffer"),
            }
        }
    }

    // Implementazione personalizzata della differenziazione ossia *val infatti nelle struct definite
    // da noi questo non è implementato di default.
    impl Deref for CircularBufferHeterogenous {
        // Implementazione personalizzata della differenziazione ossia *val infatti nelle struct definite
        // da noi questo non è implementato di default.
        type Target = [Option<Box<dyn BufferItem>>]; // Vogliamo che il nostro buffer si comporti come uno slice di T
        fn deref(&self) -> &Self::Target {
            // Verifica che il buffer sia contiguo (head <= tail)
            if self.head > self.tail && self.size > 0 {
                panic!("Buffer is not contiguous!");
            }

            // Restituisce lo slice che va da head a tail
            &self.buffer[self.head..self.tail]
        }
    }

}

pub mod circular_buffer_heterogenous_static {
    use std::fmt::Debug;
    use std::any::Any;
    use std::ops::{Deref, Index, IndexMut};

    // Trait modificato per usare un lifetime esplicito invece di 'static
    pub trait BufferItem<'a>: Debug + Any {
        fn as_any(&self) -> &dyn Any;
        fn as_any_mut(&mut self) -> &mut dyn Any;
        fn clone_box(&self) -> Box<dyn BufferItem<'a>>;
    }

    // Implementazione automatica modificata con lifetime
    impl<'a, T: 'a + Debug + Clone + Any> BufferItem<'a> for T {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }

        fn clone_box(&self) -> Box<dyn BufferItem<'a>> {
            Box::new(self.clone())
        }
    }

    #[derive(Debug)]
    pub struct CircularBufferHeterogenousStatic<'a> {
        buffer: Vec<Option<Box<dyn BufferItem<'a>>>>,
        capacity: usize,
        head: usize,
        tail: usize,
        size: usize,
    }

    #[derive(Debug, PartialEq)]
    pub enum Error {
        FullBuffer,
    }

    impl<'a> CircularBufferHeterogenousStatic<'a> {
        pub fn new(capacity: usize) -> Self {
            let mut buffer = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                buffer.push(None);
            }

            CircularBufferHeterogenousStatic {
                buffer,
                capacity,
                head: 0,
                tail: 0,
                size: 0,
            }
        }

        pub fn write<T: 'a + BufferItem<'a>>(&mut self, item: T) -> Result<(), Error> {
            if self.size == self.capacity {
                return Err(Error::FullBuffer);
            }

            self.buffer[self.tail] = Some(Box::new(item));
            self.tail = (self.tail + 1) % self.capacity;
            self.size += 1;

            Ok(())
        }

        pub fn read(&mut self) -> Option<Box<dyn BufferItem<'a>>> {
            if self.size == 0 {
                return None;
            }

            let item = self.buffer[self.head].take();
            self.head = (self.head + 1) % self.capacity;
            self.size -= 1;

            item
        }

        pub fn clear(&mut self) {
            for i in 0..self.capacity {
                self.buffer[i] = None;
            }
            self.head = 0;
            self.tail = 0;
            self.size = 0;
        }

        pub fn size(&self) -> usize {
            self.size
        }

        pub fn overwrite<T: 'a + BufferItem<'a>>(&mut self, item: T) {
            if self.size < self.capacity {
                self.write(item).unwrap();
            } else {
                self.buffer[self.head] = Some(Box::new(item));
                self.head = (self.head + 1) % self.capacity;
                self.tail = (self.tail + 1) % self.capacity;
            }
        }

        pub fn make_contiguous(&mut self) {
            if self.head == 0 || self.size == 0 {
                return;
            }

            let mut temp_buffer = Vec::with_capacity(self.capacity);

            for _ in 0..self.capacity {
                temp_buffer.push(None);
            }

            for i in 0..self.size {
                let index = (self.head + i) % self.capacity;
                temp_buffer[i] = self.buffer[index].take();
            }

            self.buffer = temp_buffer;
            self.head = 0;
            self.tail = self.size % self.capacity;
        }
    }

    impl<'a> Index<usize> for CircularBufferHeterogenousStatic<'a> {
        type Output = Box<dyn BufferItem<'a>>;

        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.size {
                panic!("Index out of bounds");
            }

            let actual_index = (self.head + index) % self.capacity;

            match &self.buffer[actual_index] {
                Some(item) => item,
                None => panic!("Trying to access an empty slot in the buffer"),
            }
        }
    }

    impl<'a> IndexMut<usize> for CircularBufferHeterogenousStatic<'a> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if index >= self.size {
                panic!("Index out of bounds");
            }

            let actual_index = (self.head + index) % self.capacity;

            match &mut self.buffer[actual_index] {
                Some(item) => item,
                None => panic!("Trying to access an empty slot in the buffer"),
            }
        }
    }

    // Implementazione di Deref modificata per usare lifetime esplicito
    impl<'a> Deref for CircularBufferHeterogenousStatic<'a> {
        type Target = [Option<Box<dyn BufferItem<'a>>>];

        fn deref(&self) -> &Self::Target {
            if self.head > self.tail && self.size > 0 {
                panic!("Buffer is not contiguous!");
            }

            &self.buffer[self.head..self.tail]
        }
    }
}