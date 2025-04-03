use ese_3::*;

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};
    use ese_3::circular_buffer_heterogenous::{CircularBufferHeterogenous};
    use ese_3::complex_number::solution::ComplexNumber;
    use super::circular_buffer::{CircularBuffer, Error};
    use ese_3::circular_buffer_heterogenous_static::{CircularBufferHeterogenousStatic, TryDeref};

    #[test]
    fn test_inserire_elemento_e_controllare_dimensione() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);

        assert_eq!(buffer.size(), 0);
        buffer.write(42).unwrap(); // Vec<Option<T>> uso di unwrap per ottenere il valore di Option.
        assert_eq!(buffer.size(), 1);
    }

    #[test]
    fn test_inserire_elemento_leggerlo_e_verificare() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);

        buffer.write(42).unwrap();
        let valore = buffer.read(); // legge da head rimuovendone il valore (size -1)

        assert_eq!(valore, Some(42));
        assert_eq!(buffer.size(), 0);
    }

    #[test]
    fn test_inserire_n_elementi_e_leggerli() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);
        let elementi = [10, 20, 30, 40, 50];

        // Inserimento elementi
        for &elem in &elementi {
            // unwrap equivale a un match di libreria dove restituisce il val di Ok() oppure stampa Err()
            buffer.write(elem).unwrap();
        }

        assert_eq!(buffer.size(), 5);

        // Lettura elementi
        for &expected in &elementi {
            assert_eq!(buffer.read(), Some(expected));
        }

        assert_eq!(buffer.size(), 0);
    }

    #[test]
    fn test_indici_ritornano_a_zero() {
        // In questo test utilizziamo una tecnica per verificare che gli indici ritornino a zero
        // riempiendo e svuotando il buffer più volte
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);

        // Prima iterazione: riempi e svuota
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();

        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));

        // Seconda iterazione: riempi e svuota di nuovo
        buffer.write(4).unwrap();
        buffer.write(5).unwrap();
        buffer.write(6).unwrap();

        assert_eq!(buffer.read(), Some(4));
        assert_eq!(buffer.read(), Some(5));
        assert_eq!(buffer.read(), Some(6));

        // Il buffer dovrebbe essere vuoto e gli indici dovrebbero essere tornati all'inizio
        assert_eq!(buffer.size(), 0);
    }

    #[test]
    fn test_leggere_da_buffer_vuoto() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);

        assert_eq!(buffer.read(), None);

        // Inserisci e leggi per svuotare il buffer
        buffer.write(42).unwrap();
        buffer.read();

        // Leggi di nuovo da buffer vuoto
        assert_eq!(buffer.read(), None);
    }

    #[test]
    fn test_scrivere_su_buffer_pieno() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);

        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();

        // Il buffer è pieno, la prossima scrittura dovrebbe fallire
        let risultato = buffer.write(4);
        assert_eq!(risultato, Err(Error::FullBuffer));

        // La dimensione dovrebbe rimanere 3
        assert_eq!(buffer.size(), 3);

        // Il contenuto dovrebbe rimanere invariato
        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
    }

    #[test]
    fn test_overwrite_su_buffer_pieno() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);

        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();

        // Ora facciamo overwrite (il buffer è pieno)
        buffer.overwrite(4);

        // La dimensione dovrebbe rimanere 3
        assert_eq!(buffer.size(), 3);

        // Il primo elemento (1) dovrebbe essere stato sovrascritto
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
        assert_eq!(buffer.read(), Some(4));
    }

    #[test]
    fn test_overwrite_su_buffer_non_pieno() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(3);

        buffer.write(1).unwrap();
        buffer.write(2).unwrap();

        // Ora facciamo overwrite ma il buffer non è pieno
        buffer.overwrite(3);

        // La dimensione dovrebbe essere 3
        assert_eq!(buffer.size(), 3);

        // Ora facciamo overwrite con buffer pieno, head +1 ordine lettura sfasato.
        buffer.overwrite(4);

        // Dovrebbe comportarsi come write
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
        assert_eq!(buffer.read(), Some(4)); // Valore più vecchio aggiornato. 1 -> 4
    }

    #[test]
    fn test_make_contiguous_su_buffer_non_contiguo() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(5);

        // Riempi il buffer
        buffer.write(1).unwrap();
        buffer.write(2).unwrap();
        buffer.write(3).unwrap();
        buffer.write(4).unwrap();
        buffer.write(5).unwrap();

        // Leggi alcuni elementi per spostare l'indice di lettura
        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));

        // Aggiungi nuovi elementi
        buffer.write(6).unwrap();
        buffer.write(7).unwrap();

        // Ora il buffer contiene [3, 4, 5, 6, 7] con read_index = 2 e write_index = 2

        // Rendi contiguo
        buffer.make_contiguous();

        // Ora il buffer dovrebbe essere [3, 4, 5, 6, 7] con read_index = 0 e write_index = 5

        // Verifica che gli elementi siano nell'ordine corretto
        assert_eq!(buffer.read(), Some(3));
        assert_eq!(buffer.read(), Some(4));
        assert_eq!(buffer.read(), Some(5));
        assert_eq!(buffer.read(), Some(6));
        assert_eq!(buffer.read(), Some(7));

        // Buffer dovrebbe essere vuoto
        assert_eq!(buffer.size(), 0);
    }

    #[test]
    fn test_heterogeneouso_buffer() {
        let mut buffer = CircularBufferHeterogenous::new(5);

        // Inserisci tipi diversi
        buffer.write(42).unwrap();
        buffer.write(ComplexNumber::new(4.0, 2.0)).unwrap();
        buffer.write("hello".to_string()).unwrap();

        // Leggi e verifica usando downcast
        let item = buffer.read().unwrap();
        if let Some(val) = item.as_any().downcast_ref::<i32>() {
            assert_eq!(*val, 42);
        } else {
            panic!("Expected i32");
        }

        let item = buffer.read().unwrap();
        if let Some(complex) = item.as_any().downcast_ref::<ComplexNumber>() {
            assert_eq!(complex.real(), 4.0);
            assert_eq!(complex.imag(), 2.0);
        } else {
            panic!("Expected ComplexNumber");
        }

        let item = buffer.read().unwrap();
        if let Some(string) = item.as_any().downcast_ref::<String>() {
            assert_eq!(*string, "hello".to_string());
        } else {
            panic!("Expected 'hello'");
        }
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_out_of_bounds() {
        let buffer = CircularBufferHeterogenous::new(3);
        // Il buffer è vuoto, quindi qualsiasi accesso dovrebbe causare panic
        let _should_panic = &buffer[0];
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_index_mut_out_of_bounds() {
        let mut buffer = CircularBufferHeterogenous::new(3);
        // Il buffer è vuoto, quindi qualsiasi accesso dovrebbe causare panic
        let _should_panic = &mut buffer[0];
    }

    #[test]
    fn test_index_access() {
        let mut buffer = CircularBufferHeterogenous::new(3);
        buffer.write(10).unwrap();
        buffer.write("hello".to_string()).unwrap();
        buffer.write(ComplexNumber::new(2.0, 2.0)).unwrap();

        let item = buffer[0].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(item, &10);
        let item = buffer[1].as_any().downcast_ref::<String>().unwrap();
        assert_eq!(item, &"hello".to_string());
        let item = buffer[2].as_any().downcast_ref::<ComplexNumber>().unwrap();
        assert_eq!(item, &ComplexNumber::new(2.0, 2.0));
    }
    #[test]
    fn test_index_mut_access() {
        let mut buffer = CircularBufferHeterogenous::new(3);
        buffer.write(10).unwrap();
        buffer.write(20).unwrap();

        let item_mut = buffer[1].as_any_mut().downcast_mut::<i32>().unwrap();
        *item_mut = 99;

        let item = buffer[1].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(item, &99);
    }

    #[test]
    fn test_deref_functionality() {
        let mut buffer = CircularBufferHeterogenousStatic::new(5);

        // Inserisci elementi
        buffer.write(10i32).unwrap();
        buffer.write(20i32).unwrap();
        buffer.write(30i32).unwrap();

        // Rendi il buffer contiguo
        buffer.make_contiguous();

        // Verifica che la lunghezza dello slice sia corretta usando 'DEREF'
        let slice_len = buffer.deref().len();
        assert_eq!(slice_len, 3);

        // Verifica individualmente i valori usando l'operatore Index invece di Deref
        // per accedere ai singoli elementi
        let value = buffer[0].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(*value, 10);

        let value = buffer[1].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(*value, 20);

        let value = buffer[2].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(*value, 30);
    }

    #[test]
    #[should_panic(expected = "Buffer is not contiguous!")]
    fn test_deref_panic_on_non_contiguous() {
        let mut buffer = CircularBufferHeterogenousStatic::new(5);

        // Crea una configurazione non contigua
        buffer.write(1i32).unwrap();
        buffer.write(2i32).unwrap();
        buffer.read(); // Sposta head a 1
        buffer.write(3i32).unwrap();
        buffer.write(4i32).unwrap();
        buffer.write(5i32).unwrap(); // Questo fa avvolgere tail a 0; vet da 0 a 4 elementi, 5 è il 0 nel buffer circolare

        // Verifica che il deref faccia panic su buffer non contiguo
        let _ = buffer.deref().len();
    }

    #[test]
    fn test_deref_after_make_contiguous() {
        let mut buffer = CircularBufferHeterogenousStatic::new(5);

        // Crea configurazione non contigua
        buffer.write(1i32).unwrap();
        buffer.write(2i32).unwrap();
        buffer.read(); // Sposta head a 1
        buffer.write(3i32).unwrap();

        // Rendi contiguo
        buffer.make_contiguous();

        // Verifica lunghezza
        let slice_len = buffer.deref().len();
        assert_eq!(slice_len, 2);

        // Verifica valori usando Index
        let value = buffer[0].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(*value, 2);

        let value = buffer[1].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(*value, 3);
    }

    #[test]
    fn test_try_deref_error_on_non_contiguous() {
        let mut buffer = CircularBufferHeterogenousStatic::new(5);

        buffer.write(1i32).unwrap();
        buffer.write(2i32).unwrap();
        buffer.read(); // Sposta head a 1
        buffer.write(3i32).unwrap();
        buffer.write(4i32).unwrap();
        buffer.write(5i32).unwrap(); // Fa avvolgere tail a 0

        // Verifica che try_deref restituisca un errore
        assert!(buffer.try_deref().is_err());
    }

    #[test]
    fn test_derefmut_should_work_for_non_contiguous() {
        let mut buffer = CircularBufferHeterogenousStatic::new(5);

        // Crea configurazione non contigua
        buffer.write(1i32).unwrap();
        buffer.write(2i32).unwrap();
        buffer.read(); // Sposta head a 1
        buffer.write(3i32).unwrap();
        buffer.write(4i32).unwrap();
        buffer.write(5i32).unwrap(); // Fa avvolgere tail a 0

        // Verifica lunghezza
        let slice_len = buffer.deref_mut().len(); // unwrap del result
        assert_eq!(slice_len, 4);

        // Verifica valori usando Index
        let value = buffer[0].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(*value, 2);

        let value = buffer[1].as_any().downcast_ref::<i32>().unwrap();
        assert_eq!(*value, 3);
    }
}