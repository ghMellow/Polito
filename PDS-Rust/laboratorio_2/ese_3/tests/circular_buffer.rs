use ese_3::*;

#[cfg(test)]
mod tests {
    use ese_3::circular_buffer_heterogenous::{CircularBufferHeterogenous};
    use ese_3::complex_number::solution::ComplexNumber;
    use super::circular_buffer::{CircularBuffer, Error};

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
    fn test_index_mut_access() {
        // Creiamo un buffer circolare
        let mut buffer = CircularBufferHeterogenous::new(5);

        // Aggiungiamo alcuni elementi
        buffer.write(Box::new(10)).unwrap();
        buffer.write(Box::new("hello".to_string())).unwrap();
        buffer.write(Box::new(20)).unwrap();

        // Modifichiamo un elemento tramite index_mut
        // Nota: dobbiamo fare un downcast per modificare il valore interno
        let item = &buffer[0];
        let int_item = *item.as_any().downcast_ref::<i32>().unwrap();
        // Per modificare il valore dovremmo avere un metodo specifico o utilizzare
        // un downcast_mut, ma questo dipende dall'implementazione di BufferItem
        assert_eq!(int_item, 10);

        /*
        // Testiamo il caso in cui il buffer effettua wrap-around
        let mut circular = CircularBufferHeterogenous::new(3);
        circular.write(Box::new(10)).unwrap();
        circular.write(Box::new(20)).unwrap();
        circular.write(Box::new(30)).unwrap();

        // Leggiamo il primo elemento, poi ne aggiungiamo uno nuovo
        // che dovrebbe sovrascrivere il primo (se il buffer è pieno)
        assert_eq!(circular[0], "10");

        // Aggiungiamo un nuovo elemento che dovrebbe causare l'avanzamento di head
        circular.write(Box::new(40)).unwrap();

        // Ora il buffer dovrebbe contenere [20, 30, 40] con head a 1
        assert_eq!(circular[0], "20");
        assert_eq!(circular[1], "30");
        assert_eq!(circular[2], "40");
         */
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
}