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