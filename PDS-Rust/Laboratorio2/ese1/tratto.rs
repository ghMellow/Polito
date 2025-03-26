//use regex::Regex;
//use crate::ese1::conv;
use crate::ese1::slugify;


/// Tratto per la gestione degli slug
pub trait MySlug {
    /// Verifica se la stringa corrente è già uno slug valido
    ///
    /// # Returns
    /// `true` se la stringa è uno slug valido, `false` altrimenti
    fn is_slug(&self) -> bool;

    /// Converte la stringa corrente in uno slug
    ///
    /// # Returns
    /// Una nuova stringa trasformata in formato slug
    fn to_slug(&self) -> String;
}

/// IMPLEMENTAZIONE SPECIFICA
/*
/// Implementazione del tratto per il tipo String
/// NOTA: tutte le stringe implementeranno di default questa funzione!
impl MySlug for String {
        fn is_slug(&self) -> bool {
            let input = self;
            let output = slugify(input);

            if *input == output {true}
            else {false}
        }

        fn to_slug(&self) -> String {
            slugify(&self)
        }
}

/// Implementazione del tratto per &str
impl MySlug for &str {
    fn is_slug(&self) -> bool {
        let input = *self;
        let output = slugify(input);

        if input == output {true}
        else {false}
    }

    fn to_slug(&self) -> String {
        slugify(&self)
    }
}
*/

/// IMPLEMENTAZIONE GENERICA:
/// Implementazione generica del tratto MySlug utilizzando il constraint Deref
///
/// Il vincolo `std::ops::Deref<Target = str>` consente un'implementazione flessibile che supporta:
/// - Conversione diretta per i tipi che possono essere dereferenziati come stringa
/// - Maggiore genericità rispetto ai semplici metodi di conversione
///
/// Questo approccio permette di utilizzare il tratto con una varietà di tipi,
/// inclusi `String`, `&str`, `Cow<str>` e altri tipi personalizzati che implementano
/// il tratto `Deref` per `str`.
///
/// # Vantaggi
/// - Implementazione unica e riutilizzabile
/// - Supporto per molteplici tipi di stringhe
/// - Elevata flessibilità nella gestione dei tipi di input
impl<T> MySlug for T
where T: std::ops::Deref<Target = str> {
    fn is_slug(&self) -> bool {
        let input = self.as_ref();
        let output = slugify(input);

        input == output
    }

    fn to_slug(&self) -> String {
        slugify(self.as_ref())
    }
}