use std::{env};
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::fmt;


/** I-O file **/

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


/** struct Board **/

const BSIZE: usize = 20;

pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE], //NOTE: 0 equals to space and 1 equals to B respect to the text of the lab
}
impl Board {
    // crea una board vuota con una disponibilità di navi
    pub fn new(boats: &[u8]) -> Board {
        let mut b = [0; 4];
        for (i, &boat) in boats.iter().enumerate() {
            b[i] = boat;
            println!("{}", boat);
        }

        Board{ boats: b, data: [[0; BSIZE]; BSIZE] }
    }

    /* crea una board a partire da una stringa che rappresenta tutto il contenuto del file board.txt */
    pub fn from(s: String) -> Board {
        Board::from_string(&s).unwrap_or_else(|e| {
            eprintln!("Errore nel parsing: {}", e);
            // Puoi restituire un board vuoto o fare altro
            Board::new(&[4, 3, 2, 1])
        })
    }

    /* aggiunge la nave alla board, restituendo la nuova board se possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce una nuova board con la barca, come si può fare? */
    pub fn add_boat(&mut self, boat: Boat, pos: (usize, usize), boat_type: u8) -> Result<&mut Board, Error> {
        // decrement number of boat available
        let index = (boat_type.saturating_sub(1)) as usize;
        self.boats[index] -= 1;

        // update board
        match boat {
            Boat::Vertical(y) => {
                for i in 0..boat_type as usize {
                    if !check_placement(&(pos.0 + i), &pos.1, &self){
                        panic!("Position already used by another boat");
                    }
                }
                for i in 0..boat_type as usize {
                    self.data[pos.0 + i][pos.1] = 1;
                }
            },
            Boat::Horizontal(x) => {
                for i in 0..boat_type as usize {
                    if !check_placement(&pos.0, &(pos.1 + i), &self){
                        panic!("Position already used by another boat");
                    }
                }
                for i in 0..boat_type as usize {
                    self.data[pos.0][pos.1 + i] = 1;
                }
            }
        }

        Ok(self)
    }

    /* converte la board in una stringa salvabile su file */
    // Converte la struttura Board in una stringa
    pub fn to_string(&self) -> String {
        // Converti boats in stringa
        let boats_str = format!("{:?}", self.boats);

        // Converti data in stringa
        let data_str = self.data
            .iter()
            .map(|row|
                row.iter()
                    .map(|&num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            )
            .collect::<Vec<String>>()
            .join("\n");

        format!("Boats: {}\nData:\n{}", boats_str, data_str)
    }

    // Converte una stringa in struttura Board
    pub fn from_string(s: &str) -> Result<Self, String> {
        // Splitta la stringa in righe
        let lines: Vec<&str> = s.split('\n').collect();

        // Estrai boats
        let boats_str = lines[0].trim_start_matches("Boats: ").trim_matches(&['[', ']']);
        let boats: [u8; 4] = boats_str
            .split(", ")
            .map(|x| x.parse().map_err(|_| "Errore nel parsing dei boats"))
            .collect::<Result<Vec<u8>, _>>()?
            .try_into()
            .map_err(|_| "Numero di boats non corretto")?;

        // Converti data
        let mut data = [[0u8; BSIZE]; BSIZE];
        for (i, line) in lines[2..].iter().enumerate() {
            if !line.is_empty() {
                let row: Vec<u8> = line
                    .split_whitespace()
                    .map(|x| x.parse().map_err(|_| "Errore nel parsing dei dati"))
                    .collect::<Result<Vec<u8>, _>>()?;

                if row.len() != BSIZE {
                    return Err(format!("Lunghezza riga {} invalida. Atteso {}, trovato {}", i, BSIZE, row.len()));
                }

                data[i].copy_from_slice(&row);
            }
        }

        Ok(Board { boats, data })
    }
}


/** struct PlaceBoat **/

pub struct PlaceBoat {
    direction: String,
    boat_type: u8,
    x: u8,
    y: u8,
}
impl PlaceBoat {
    pub fn get_boat_enum(&self) -> Option<Boat> {
        match self.direction.to_lowercase().as_str() {
            "v" => Some(Boat::Vertical(self.boat_type as usize)),
            "h" => Some(Boat::Horizontal(self.boat_type as usize)),
            _ => None
        }
    }
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


/** functions **/
fn check_number_of_ship(b:&Board, pl:&PlaceBoat) -> bool {
    let index = (pl.boat_type.saturating_sub(1)) as usize;
    if b.boats[index] == 0 {
        return false;
    }

    true
}

fn check_coordinate(&coordinate: &u8) -> bool {
    if coordinate >= 0 && coordinate < BSIZE as u8 {
        return true;
    }

    false
}

fn check_placement(&x:&usize, &y:&usize, b: &Board) -> bool {
    if b.data[x][y] == 1 {
        return false;
    }

    true
}

pub fn main() {
    println!("\n\n Console:\n");
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let functions = &args[2];


        /** Bisognava usare la libreria Clap **/
        match functions.as_str() {
            "new" => {
                let parameters: Vec<u8> = args[3]
                    .split(',')
                    .filter_map(|s| s.trim().parse::<u8>().ok()) // Converte e gestisce eventuali errori
                    .collect();
                let b = Board::new(parameters.as_slice());

                if let Err(e) = write_to_file(FILENAME, &b.to_string()) {
                    eprintln!("Error writing file: {}", e);
                }
            },
            "add_boat" => {
                match read_file_content(FILENAME) {
                    Ok(content) => {
                        match Board::from_string(content.as_str()) {
                            Ok(b) => {
                                let mut board = b;

                                let parameters: Vec<String> = args[3]
                                    .split(',')
                                    .map(|s| s.to_string())
                                    .collect();
                                let mut place_boat = PlaceBoat {
                                    direction: parameters[0].clone(),
                                    boat_type: parameters[1].parse::<u8>().unwrap(),
                                    x: parameters[2].parse::<u8>().unwrap(),
                                    y: parameters[3].parse::<u8>().unwrap()
                                };
                                println!("{} {} {} {}", place_boat.direction, place_boat.boat_type, place_boat.x, place_boat.y);

                                //check data validity with a function
                                if !check_number_of_ship(&board, &place_boat) {
                                    panic!("maximus number of boat of this type already placed")
                                }

                                if !check_coordinate(&place_boat.x) || !check_coordinate(&place_boat.y) {
                                    panic!("coordinate out of bounds");
                                }


                                //update board
                                let boat = PlaceBoat::get_boat_enum(&place_boat).unwrap();
                                match Board::add_boat(&mut board, boat, (place_boat.x as usize, place_boat.y as usize), place_boat.boat_type){
                                    Ok(b) => {
                                        if let Err(e) = write_to_file(FILENAME, &b.to_string()) {
                                            eprintln!("Error writing file: {}", e);
                                        }
                                    },
                                    Err(e) => {eprintln!("error adding boat");}
                                }
                            },
                            Err(e) => {
                                eprintln!("Errore {}: {}", FILENAME, e);
                                panic!("Failed to create board from file content");
                            }

                        }
                    },
                    Err(e) => eprintln!("Error reading file: {}", e),
                }
            },
            _ => panic!("Unknown function {}", functions.as_str()),
        }
    }

}
