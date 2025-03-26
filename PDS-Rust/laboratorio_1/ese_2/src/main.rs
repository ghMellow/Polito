use regex::Regex;

fn conv(c: char) -> char {
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


fn slugify(s: &str) -> String {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accented_conversion() {
        assert_eq!(conv('á'), 'a');
        assert_eq!(conv('é'), 'e');
        assert_eq!(conv('ü'), 'u');
        assert_eq!(conv('ų'), 'u');
        assert_eq!(conv('ẃ'), 'w');
    }

    #[test]
    fn test_non_accented_conversion() {
        assert_eq!(conv('a'), 'a');
        assert_eq!(conv('b'), 'b');
        assert_eq!(conv('z'), 'z');
    }

    #[test]
    fn test_unknown_character_conversion() {
        assert_eq!(conv('Ω'), '-');
    }

    #[test]
    fn test_unlisted_accented_conversion() {
        assert_eq!(conv('ῶ'), '-'); // Special accented character not in list
    }

    #[test]
    fn test_string_with_multiple_words() {
        let input = "hello world";
        let output = slugify(input);
        assert_eq!(output, "hello-world");
    }

    #[test]
    fn test_string_with_capital_letters() {
        let input = "Hello WORLD";
        let output = slugify(input);
        assert_eq!(output, "hello-world");
    }

    #[test]
    fn test_string_with_accented_characters() {
        let input = "héllo wôrld";
        let output = slugify(input);
        assert_eq!(output, "hello-world");
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let output = slugify(input);
        assert_eq!(output, "-");
    }

    #[test]
    fn test_string_with_multiple_spaces() {
        let input = "hello    world";
        let output = slugify(input);
        assert_eq!(output, "hello-world");
    }

    #[test]
    fn test_string_with_consecutive_invalid_characters() {
        let input = "hello!@#$world";
        let output = slugify(input);
        assert_eq!(output, "hello-world");
    }

    #[test]
    fn test_string_with_only_invalid_characters() {
        let input = "!@#$%^&*";
        let output = slugify(input);
        assert_eq!(output, "-");
    }

    #[test]
    fn test_string_with_trailing_space() {
        let input = "hello world ";
        let output = slugify(input);
        assert_eq!(output, "hello-world");
    }

    #[test]
    fn test_string_with_consecutive_invalid_characters_at_end() {
        let input = "hello world!!!";
        let output = slugify(input);
        assert_eq!(output, "hello-world");
    }
}



fn main() {
    let input = "SSSS";//"test__3-é_";
    let output = slugify(input);
    println!("\nfrom '{}' to '{}'", input, output);
}
