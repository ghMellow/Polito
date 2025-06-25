/*
/// Rust Module and Import Best Practices
///
/// Key Principles:
/// 1. Module Visibility
/// - Use `pub mod` in `lib.rs` or `main.rs` to declare public modules
/// - Mark traits, structs, and functions with `pub` to allow cross-module access
///
/// 2. Importing Modules
/// - Use `crate::` to import from the root of the current crate
/// - Ensures clean, absolute path references between modules
///
/// 3. Project Structure Checks
/// - Confirm module file names exactly match module declarations
/// - Verify external dependencies are added to `Cargo.toml`
/// - Ensure all referenced modules exist in the `src/` directory
///
/// Common Gotchas:
/// - Forgetting to make items `pub`
/// - Using relative imports instead of `crate::`
/// - Mismatched module and file names
/// - Missing dependency declarations
*/

pub mod solution{
    use std::{error, fmt};
    use std::cmp::Ordering;
    use std::ops::{Add, AddAssign};
    use std::default::Default;
    use std::hash::Hasher;
    use std::hash::Hash;

    #[derive(Debug, Copy, Clone, Default)] //used for {:?} debug; Copy Clone used for posses
    pub struct ComplexNumber{
        real:f64,
        imag:f64,
    }

    /// Orphan rule: Rust, impedisce di implementare un trait esterno (Default) su un tipo
    /// esterno direttamente (e.g. un array [ComplexNumber; N]).
    /// Bisogna Usare un <! WRAPPER STRUCT !> e definire un nuovo tipo che incapsula l'array
    /// e implementare Default su di esso.
    pub struct ComplexArray<const N: usize>(pub [ComplexNumber; N]);
    impl<const N: usize> Default for ComplexArray<N> {
        fn default() -> Self {
            ComplexArray([ComplexNumber::default(); N])
        }
    }
    impl fmt::Display for ComplexNumber{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    impl ComplexNumber{
        pub fn new(real:f64, imag:f64) -> ComplexNumber{
            ComplexNumber{
                real, imag
            }
        }

        pub fn from_real(real:f64) -> ComplexNumber{
            ComplexNumber{
                real, imag:0.0
            }
        }

        pub fn to_tuple(&self) -> (f64, f64){
            (self.real, self.imag)
        }

        pub fn real(&self) -> f64{
            self.real
        }

        pub fn imag(&self) -> f64{
            self.imag
        }

        // Computes the modulus of the complex number
        fn modulus(&self) -> f64 {
            self.real.hypot(self.imag) // Equivalent to sqrt(real^2 + imag^2)
        }

        pub fn as_ref(&self) -> &f64{
            &self.real
        }

        pub fn as_mut(&mut self) -> &mut f64{
            &mut self.real
        }
    }


    /// IMPLEMENTAIONE STRUTTURA DATI HASH
    impl Hash for ComplexNumber {
        // Implementation of the `Hash` trait for the `ComplexNumber` struct.
        // This ensures that the struct can be used in hashed collections like HashMap and HashSet.
        // The `real` and `imag` fields, which are of type `f64`, are converted to raw bits
        // using the `to_bits()` method before being fed into the hasher to calculate the hash.

        // In Rust, .hash(state) is a method call used within the implementation of the Hash trait.
        // It is used to combine the hash value of the current object with the existing state of the hasher.
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.real.to_bits().hash(state);
            self.imag.to_bits().hash(state);
        }
    }


    /// IMPLEMENTAZIONE 'operator overloading' (sovraccarico degli operatori) come la somma
    /// Questa tecnica si chiama syntactic sugar (zucchero sintattico). Serve per rendere il codice
    /// più leggibile e pulito, permettendo di scrivere a + b invece di a.add(b).

    /* Nota: 'Self' si riferisce a chi è il soggetto di 'for'
       perciò nelle funzioni si passa come parametro 'self' se il soggetto di 'for' va utilizzato
       altrimenti si passa l'oggetto specifico necessario. try_from per esempio è implementato per f64
       che è l'output '-> Result<Self, Self::Error>' mentre in input arriva un oggetto.
        Add ha come soggetto ComplexNumber e necessita di lui perciò lo passiamo come parametro insieme
        al secondo valore della somma che però va esplicitato, un solo self.
    **/
    impl Add for ComplexNumber{
        // Addizione con lo stesso tipo (ComplexNumber + ComplexNumber)
        type Output = ComplexNumber;
        fn add(self, rhs: ComplexNumber) -> Self::Output {
            return ComplexNumber{real: self.real + rhs.real, imag: self.imag + rhs.imag}
        }
    }

    // Res = a + b
    // Add<tipo di b> mentre for indica la tipologia di a
    // se Add e basta allora si indica self e dipende da cos'è b

    impl Add<f64> for ComplexNumber{
        // Addizione con f64 (ComplexNumber + f64)
        type Output = ComplexNumber;
        fn add(self, rhs: f64) -> Self::Output {
            return ComplexNumber{real: self.real + rhs, imag: self.imag}
        }
    }

    impl AddAssign for ComplexNumber{
        // Auto incremento di self con ComplexNumber(ComplexNumber += ComplexNumber)
        fn add_assign(&mut self, rhs: ComplexNumber) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    impl Add<&ComplexNumber> for ComplexNumber{
        // Addizione con &ComplexNumber come riferimento (ComplexNumber + &ComplexNumber)
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag
            }
        }
    }

    impl Add<&ComplexNumber> for &ComplexNumber{
        // Addizione tra &ComplexNumber come riferimento (&ComplexNumber + &ComplexNumber)
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag
            }
        }
    }

    /// IMPLEMENTAZIONE TRATTI DI CONFRONTO

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> Ordering {
            self.modulus().total_cmp(&other.modulus())
        }
    }

    impl PartialOrd for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for ComplexNumber {
        fn eq(&self, other: &Self) -> bool {
            self.modulus() == other.modulus()
        }
    }

    impl Eq for ComplexNumber {}


    /// CONVERSIONE DA UN TIPO AD UN ALTRO

    // da f64 a ComplexNumber
    impl From<f64> for ComplexNumber {
        // self qui è ComplexNumber!
        fn from(value: f64) -> ComplexNumber {
            ComplexNumber{ real: value, imag: 0.0 }
        }
    }

    // into chiama from, bisogna implementare lui
    // questo permette di avere in automatico la conversione della struct in un tipo voluto
    /** nota: diverso da Add, qui si ha from <valore iniziale> for ..output voluto..  **/
    /*impl From<ComplexNumber> for f64 {
        fn from(complex: ComplexNumber) -> Self {
            complex.real
        }
    }
    /// Versione con gestione del panic! Include la commentata sopra
    impl From<ComplexNumber> for f64 {
        fn from(complex: ComplexNumber) -> Self {
            if complex.imag == 0.0 {
                complex.real
            } else {
                panic!("Cannot convert complex number with non-zero imaginary part")
            }
        }
    }*/

    
    /// Commentati per implementare test try_into. CONVERSIONE E GESTIONE ERRORE CON USO DI ENUM.

    // Definizione dell'errore per la conversione TryInto
    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError {
        ImaginaryNotZero,
    }

    impl fmt::Display for ComplexNumberError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ComplexNumberError::ImaginaryNotZero => write!(f, "Cannot convert complex number with non-zero imaginary part"),
            }
        }
    }

    impl error::Error for ComplexNumberError {}

    // Implementazione del trait TryFrom (necessario per TryInto)
    // Nota:implementiamo TryFrom<ComplexNumber> per f64 invece di TryInto<f64> per ComplexNumber
    // perché Rust automaticamente implementa TryInto<U> per T quando esiste TryFrom<T> per U.
    // Questo avviene grazie a un'implementazione generica nella libreria standard:
    //      impl<T, U> TryInto<U> for T where U: TryFrom<T>
    impl TryFrom<ComplexNumber> for f64 {
        type Error = ComplexNumberError;

        fn try_from(complex: ComplexNumber) -> Result<Self, Self::Error> {
            if complex.imag == 0.0 {
                Ok(complex.real)
            } else {
                Err(ComplexNumberError::ImaginaryNotZero)
            }
        }
    }
}