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

pub mod tratto;
use tratto::*;

pub mod my_functions {
    use regex::Regex;

    fn conv(c: char) -> char {
        /// Converte char nell'equivalente non accentato
        const SUBS_I : &str =
            "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
        const SUBS_O: &str =
            "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

        // make str indexable by transforming them into string
        let subs_i: Vec<char> = SUBS_I.chars().collect();
        let subs_o: Vec<char> = SUBS_O.chars().collect();

        let mut index_j = subs_o.len();  // Initialize index_j with the length of subs_o
        for (index, &i) in subs_i.iter().enumerate() {  // Iterate over characters in subs_i
            if i == c {
                index_j = index;  // Store the index when a match is found
                break;
            }
        }

        if index_j < subs_o.len() {
            // .chars().next() converts the string into an iterator of characters and retrieves the first character.
            // .unwrap() is used to get the value, assuming the string is non-empty.
            // If the string is empty, it will cause a panic.
            return subs_o[index_j];
        }


        // redundant control requested by the exercise
        // in fact, the code already handles it in the flow of the slugify function
        let re = Regex::new(r"[a-z]").unwrap();
        if re.is_match(&c.to_string()) {
            return c
        } else {
            return '-'
        }
    }


    pub fn slugify(s: &str) -> String {
        /// trasforma la stringa -> slugify

        // ensure to elaborate not empty str
        if s.is_empty()  {
            return String::from("-");
        }

        // removal of accented characters
        let mut normilized = String::new();
        for c in s.chars() {
            let tmp = c.to_ascii_lowercase();
            normilized.push(conv(tmp));
        }

        // regex filtering
        let re = Regex::new(r"[a-z0-9]").unwrap();
        let filtered: String = normilized.chars()
            .map(|c| if re.is_match(&c.to_string()) { c } else { '-' })
            .collect();

        // only one '-' between two characters
        let mut previusly: char = filtered.chars().next().unwrap();
        let mut res = String::from(previusly);
        for c in filtered.chars().skip(1) {
            if c == '-' && c == previusly {
                continue;
            }
            res.push(c);
            previusly = c;
        }


        // ensure last character is not '-'
        if res.ends_with('-') {
            res.pop();
        }

        // check if res is empty
        if res.is_empty()  {
            return String::from("-");
        }

        res
    }
}