use regex::Regex;
use crate::conv;
use crate::slugify;

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