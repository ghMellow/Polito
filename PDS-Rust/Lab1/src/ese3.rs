use std::fs::{File};
use std::io::{self, BufRead, BufReader, Read, Write};
use hex;

fn read_file_content(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_content_2(filename: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(filename)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;
    Ok(data)
}

fn read_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: io::Result<Vec<String>> = reader.lines().collect();
    lines
}

fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// ---

fn uno_a(input: &String){
    let mut f_content: String = String::new();
    match read_file_content(&input) {
        Ok(content) => {
            for i in 1..11 {
                f_content.push_str(&format!("{} - {}\n", i, content));
            }
        },
        Err(e) => panic!("Error reading file: {}", e),
    }

    let output = String::from("src/output.txt");
    if let Err(e) = write_to_file(&output, f_content.as_str()) {
        eprintln!("Error writing file: {}", e);
    }
}

fn uno_b(input: &String){
    match read_file_content(&input) {
        Ok(content) => {
            println!("File content: {}", hex::encode(content));
        },
        Err(e) => panic!("Error reading file: {}", e),
    }

    match read_file_content_2(&input) {
        Ok(content) => {
            print!("{}", hex::encode(content));
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn uno_c(){

}


fn main() {
    println!(".");

    let input = String::from("src/example.txt");

    // read and output ten time the read text
    uno_a(&input);

    // any diff?
    uno_b(&input);

    // enum
    uno_c();

}
