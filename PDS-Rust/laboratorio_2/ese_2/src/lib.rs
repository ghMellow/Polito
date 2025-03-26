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

pub mod solution{
    pub struct ComplexNumber{
        real:f64,
        imag:f64,
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

        pub fn real(&self) -> f64{
            self.real
        }

        pub fn imag(&self) -> f64{
            self.imag
        }

    }
}