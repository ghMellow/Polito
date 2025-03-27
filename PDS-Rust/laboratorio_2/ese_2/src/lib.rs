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
    use std::ops::{Add, AddAssign};
    use std::default::Default;
    use std::fmt::Error;

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

    #[derive(Debug)]
    struct Err;

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

    }

    impl Add for ComplexNumber{
        // Addizione con lo stesso tipo (ComplexNumber + ComplexNumber)
        type Output = ComplexNumber;
        fn add(self, rhs: ComplexNumber) -> Self::Output {
            return ComplexNumber{real: self.real + rhs.real, imag: self.imag + rhs.imag}
        }
    }

    /// Res = a + b
    /// Add<tipo di b> mentre for indica la tipologia di a
    /// se Add e basta allora si indica self e dipende da cos'è b

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
    /// commentati per implementare test try_into


}