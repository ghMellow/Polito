use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

fn stats(text: &str) -> [u32; 26] {
    let mut array = [0; 26];

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let index = (c.to_ascii_lowercase() as u32 - 'a' as u32) as usize;
            array[index] += 1;
        }
    }

    return array;
}

fn is_pangram(counts: &[u32]) -> bool {
    if counts.len() != 26 {
        return false;
    }

    // Using `&count` in the loop declaration (`for &count in ...`) applies pattern matching
    // to directly unpack the value. Here, `count` is an `i32` rather than a reference,
    // so there is no need for explicit dereferencing inside the loop.
    // In contrast, `counts.iter()` returns an iterator over `&i32` (references to elements of `counts`).
    // Thus, `count` is of type `&i32`, requiring explicit dereferencing with `*count` to access the value.

    for &count in counts.iter() {
        if count == 0 {
            return false;
        }
    }
    return true
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut content = String::new();
    for line in reader.lines() {
        content.push_str(&line?);
    }

    Ok(content)
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        println!("Reading file: {}", filename);

        let string;
        match read_file(filename) {
            Ok(content) => {
                println!("{}", content);
                string = content;
            },
            Err(e) => return eprintln!("Error reading file: {}", e),
        }

        let arraystat = stats(string.as_str());

        if is_pangram(&arraystat) {
            println!("'{}' is a pangram", string);
            arraystat.iter().enumerate().for_each(|(index, &x)| {
                let char = (index as u8 + 'a' as u8) as char;
                println!("{}: {}", char, x);
            });
        } else {
            println!("'{}' is not a pangram", string);
        }


    } else {
        eprintln!("No correct number of arguments passed in");
    }
}


// please note, code has been splittend in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{
    // tests are separated modules, yuo must import the code you are testing
    use super::*;

    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }

        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    println!("Running tests");
}

