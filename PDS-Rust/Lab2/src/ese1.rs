use regex::Regex;
use crate::tratto::MySlug; // Use crate:: to import from the root of the current crate


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

    return res;
}


pub fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true
    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();

    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice
}
