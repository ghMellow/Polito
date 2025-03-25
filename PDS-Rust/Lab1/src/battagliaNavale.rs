use std::{env};
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::fmt;


const FILENAME: &str = "board.txt";
fn read_file_content(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Implementazione di Display per una stampa più leggibile
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Boats: {:?}\nData:\n", self.boats)?;
        for row in &self.data {
            for &cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


/** ----------- **/

const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE], //NOTE: 0 equals to space and 1 equals to B respect to the text of the lab
}
pub enum Error {
    Overlap,
    OutOfBounds,
    BoatCount,
}
pub enum Boat {
    Vertical(usize),
    Horizontal(usize),
}

impl Board {
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board {
        let mut b = [0; 4];
        for (i, &boat) in boats.iter().enumerate() {
            b[i] = boat;
            println!("{}", boat);
        }

        Board{ boats: b, data: [[0; BSIZE]; BSIZE] }
    }


    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String) -> Board {
        let mut b = [0; 4];
        Board{ boats: b, data: [[0; BSIZE]; BSIZE] }
    }


    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */


    pub fn add_boat(self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> {
        Ok(Board{ boats: [0;4], data: [[0; BSIZE]; BSIZE] })
    }


    /* converte la board in una stringa salvabile su file */
    // Converte la struttura Board in una stringa
    pub fn to_string(&self) -> String {
        // Converti boats in stringa
        let boats_str = self.boats
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        // Converti data in stringa
        let data_str = self.data
            .iter()
            .map(|row|
                row.iter()
                    .map(|&num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .collect::<Vec<String>>()
            .join(";");

        // Combina boats e data
        format!("{};{}", boats_str, data_str)
    }

    // Converte una stringa in struttura Board
    pub fn from_string(s: &str) -> Result<Self, String> {
        // Splitta la stringa in parti
        let parts: Vec<&str> = s.split(';').collect();

        // Verifica che ci siano abbastanza parti
        if parts.len() != BSIZE + 1 {
            return Err(format!("Formato stringa invalido. Atteso {} parti, trovate {}", BSIZE + 1, parts.len()));
        }

        // Converti boats
        let boats: [u8; 4] = parts[0]
            .split(',')
            .map(|x| x.parse().map_err(|_| "Errore nel parsing dei boats"))
            .collect::<Result<Vec<u8>, _>>()?
            .try_into()
            .map_err(|_| "Numero di boats non corretto")?;

        // Converti data
        let mut data = [[0u8; BSIZE]; BSIZE];
        for (i, row_str) in parts[1..].iter().enumerate() {
            let row: Vec<u8> = row_str
                .split(',')
                .map(|x| x.parse().map_err(|_| "Errore nel parsing dei dati"))
                .collect::<Result<Vec<u8>, _>>()?;

            if row.len() != BSIZE {
                return Err(format!("Lunghezza riga {} invalida. Atteso {}, trovato {}", i, BSIZE, row.len()));
            }

            data[i].copy_from_slice(&row);
        }

        Ok(Board { boats, data })
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let functions = &args[2];
        let parameters: Vec<u8> = args[3]
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok()) // Converte e gestisce eventuali errori
            .collect();

        println!("{} {:?}", functions.to_string(), parameters);

        match functions.as_str() {
            "new" => {
                let b = Board::new(parameters.as_slice());

                if let Err(e) = write_to_file(FILENAME, &b.to_string()) {
                    eprintln!("Error writing file: {}", e);
                }
            },
            "add_boat" => {
                match read_file_content(FILENAME) {
                    Ok(content) => {
                        let b = Board::from_string(content.as_str()).unwrap();
                        println!("{}", b);
                    },
                    Err(e) => eprintln!("Error reading file: {}", e),
                }
            },
            _ => panic!("Unknown function {}", functions.as_str()),
        }
    }

}
