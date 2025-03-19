use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};

// Read entire file content
fn read_file_content(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Read file line by line
fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: io::Result<Vec<String>> = reader.lines().collect();
    lines
}

// Write to a file (overwrite if exists)
fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Append to a file (create if not exists)
fn append_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        println!("Reading file: {}", filename);

        match read_file_content(filename) {
            Ok(content) => println!("Full content:\n{}", content),
            Err(e) => eprintln!("Error reading file: {}", e),
        }

        match read_file_lines(filename) {
            Ok(lines) => {
                for (i, line) in lines.iter().enumerate() {
                    println!("Line {}: {}", i + 1, line);
                }
            }
            Err(e) => eprintln!("Error reading lines: {}", e),
        }

        let fout = String::from("src/output.txt");
        let write_content = "This is a new file content.";
        if let Err(e) = write_to_file(&fout, write_content) {
            eprintln!("Error writing file: {}", e);
        }

        let append_content = "\nAppending this line.";
        if let Err(e) = append_to_file(&fout, append_content) {
            eprintln!("Error appending to file: {}", e);
        }
    }
}
