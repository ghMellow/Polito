// WARNING: 
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implement missing functions and fix the code
// - *** see test functions in the code for usage examples

use std::io::{self};

/// test

// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        //LineEditor{ lines: vec![s.split("\n").map(String::from).collect()] }
        //LineEditor{ lines: Vec::from(s.split("\n").map(String::from)) }
        LineEditor { lines: s.split("\n").map(String::from).collect() }
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        /*let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let lines: io::Result<Vec<String>> = reader.lines().collect();
        /// ? demando al compilatore di scrivere le clausole match nel caso panic
        Ok(LineEditor{lines: lines?})*/

        let vet = std::fs::read_to_string(file_name) // from io to fs because it accepts a file path as a string
            .unwrap()  // panic on possible file-reading errors
            .lines()  // split the string into an iterator of string slices
            .map(String::from)  // make each slice into a string
            .collect();  // gather them together into a vector
        Ok(LineEditor{lines: vet})
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        self.lines[line].replace_range(start..end, subst);
    }
}



// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped 
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,  // riferimento no possesso del valore.  il riferimento al valore di text deve esistere finchè esiste l'istanza di Match
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find_example<'a>(lines: &Vec<&'a str>, pattern: &str) -> Vec<Match<'a>>{
    let mut matches = Vec::new();
    let re = regex::Regex::new(pattern).unwrap();
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}

// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
struct FindReplace<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>,
}

impl<'a> FindReplace<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        let matches = Vec::new();
        FindReplace {lines, pattern: pattern.to_string(), matches}
    }

    // return all the matches
    pub fn matches(&self) -> &Vec<Match> {
        self.matches.as_ref()
    }

    // apply a function to all matches and allow to accept them and set the repl
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        self.matches = find_example(&self.lines, &self.pattern);

        for m in self.matches.iter_mut() {
            fun(m);
        }
    }
}


//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindReplace::new(lines, "ll");

    // find all the matches and accept them 
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("some repl".to_string());
        true
    });

    // now let's replace the matches
    // why this loop won't work?
    /*for m in finder.matches() {
        if let Some(repl) = &m.repl { editor.replace(m.line, m.start, m.end, repl); }
    }*/

    // alternate method: why this one works? 

    let mut subs = Vec::new();
    for m in finder.matches() {
        if let Some(repl) = &m.repl {
            subs.push((m.line, m.start, m.end, repl.to_string()));
        }
    }

    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, &subst);
    }

}



// (6) sometimes it's very expensive to find all the matches at once before applying 
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> LazyFinder<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        LazyFinder{ lines, pattern: pattern.to_string(), pos: Some(FinderPos{ line: 0, offset: 0 }) }
    }

    pub fn next(&mut self) -> Option<Match> {
        // remember:
        // return None if there are no more matches
        // return Some(Match) if there is a match
        // each time save the position of the match for the next call

        let re = regex::Regex::new(self.pattern.as_str()).unwrap();

        // Essendo valori opzionali usati tante volte al posto di usare la notazione .?. conviene
        // fare un pattern matching condizionale. In questo modo dentro al if uso direttamente pos.
        if let Some(pos) = self.pos.as_mut() {

            for index in pos.line..self.lines.len() {
                // Uso accesso diretto e non get(index).unwrap(); poichè essendo dentro un ciclo limitato
                // nello spazio del vettore so di non poter sforare la dimensione. In altri casi meglio get.
                let line = &self.lines[index];

                // Se match partendo da ultimo offset estratto da Option il valore e salvo in match
                if let Some(mat) = re.find_at(line, pos.offset) {
                    pos.line = index;
                    pos.offset = mat.end();
                    return Some(
                        Match {
                            line: index,
                            start: mat.start(),
                            end: mat.end(),
                            text: &line[mat.start()..mat.end()],  // riferimento no possesso del valore.  il riferimento al valore di text deve esistere finchè esiste l'istanza di Match
                            repl: None
                        }
                    )
                }
            }
            // Se no match nell'ultima riga e da offset salvato, devo azzerare offset per la prossima riga
            pos.offset = 0;
        }

        None
    }
}

// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second ll line full of text.";
    let editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches and accept them 
    while let Some(m) = finder.next() {
        println!("match found: {} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) now you have everything you need to implement the real Iterator

struct FindIter<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    // ... other?
    pos: Option<FinderPos>
}

impl<'a> FindIter<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        FindIter{ lines, pattern: pattern.to_string(), pos: Some(FinderPos{ line: 0, offset: 0 }) }
    }
}

impl<'a> Iterator for FindIter<'a> {
    type Item = Match<'a>; // <== we inform the Iterator that we return a Match

    fn next(&mut self) -> Option<Self::Item> {
        let re = regex::Regex::new(self.pattern.as_str()).unwrap();

        // Essendo valori opzionali usati tante volte al posto di usare la notazione .?. conviene
        // fare un pattern matching condizionale. In questo modo dentro al if uso direttamente pos.
        if let Some(pos) = self.pos.as_mut() {

            for index in pos.line..self.lines.len() {
                // Uso accesso diretto e non get(index).unwrap(); poichè essendo dentro un ciclo limitato
                // nello spazio del vettore so di non poter sforare la dimensione. In altri casi meglio get.
                let line = &self.lines[index];

                // Se match partendo da ultimo offset estratto da Option il valore e salvo in match
                if let Some(mat) = re.find_at(line, pos.offset) {
                    pos.line = index;
                    pos.offset = mat.end();
                    return Some(
                        Match {
                            line: index,
                            start: mat.start(),
                            end: mat.end(),
                            text: &line[mat.start()..mat.end()],  // riferimento no possesso del valore.  il riferimento al valore di text deve esistere finchè esiste l'istanza di Match
                            repl: None
                        }
                    )
                }
            }
            // Se no match nell'ultima riga e da offset salvato, devo azzerare offset per la prossima riga
            pos.offset = 0;
        }

        None
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let finder = FindIter::new(lines, "ll");

    // find all the matches and accept them 
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    
    }
}
