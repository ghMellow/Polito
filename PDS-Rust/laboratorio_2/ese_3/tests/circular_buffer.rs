use ese_3::circular_buffer;

#[cfg(test)]
mod tests {
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
}