use std::fs::{File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::time::SystemTime;
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

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
}

fn enum_handle_errors(err: Error) {
    match err {
        Error::Simple(time) => {
            println!("Error Type: Simple");
            println!("Timestamp: {:?}", time);
        }
        Error::Complex(time, msg) => {
            println!("Error Type: Complex");
            println!("Timestamp: {:?}", time);
            println!("Message: {}", msg);
        }
    }
}

fn due(){
    let simple_error = Error::Simple(SystemTime::now());
    let complex_error = Error::Complex(SystemTime::now(), String::from("Something went wrong"));

    enum_handle_errors(simple_error);
    enum_handle_errors(complex_error);
}

enum MulErr {Overflow, NegativeNumber}

fn mul(a: i32, b: i32) -> Result<u32, MulErr> {

    // checked_mul is used to safely perform the multiplication. If an overflow occurs, it returns None
    match a.checked_mul(b) {
        // Some optional element, two possible value: <T> and None
        Some(mul) => {
            if mul < 0 {
                Err(MulErr::NegativeNumber)
            } else {
                Ok(mul as u32)
            }
        }
        None => Err(MulErr::Overflow), // Overflow detected
    }
}

pub fn tre(){
    let a = 2_i32.pow(30) - 1; // 2^31 - 1 ! overflow già qui
    let b = 5;
    match mul(a, b){
        Ok(mul) => {
            println!("{} * {} = {}", 5, 5, mul);
        },
        Err(MulErr::NegativeNumber) => {
            println!("Negative number");
        }
        Err(MulErr::Overflow) => {
            println!("Overflow");
        }
    }
}


pub fn main() {
    println!(".");

    //let input = String::from("src/example.txt");

    // read and output ten time the read text
    //uno_a(&input);

    // any diff?
    //uno_b(&input);

    // enum
    due();

}
