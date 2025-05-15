use std::time::Instant;
use crate::primes::find_primes_sol1;
use crate::primes::find_primes_sol2;
use crate::game::verify;
mod primes;
mod game;

fn main() {
    // let duration = Instant::now();
    // let res = find_primes_sol1(1000000, 16);
    // println!("{:?}\nTotal execution time: {:?}", res, duration.elapsed());
    let test = ["2+2+2+2+2".to_string(), "7+4+6+4-8".to_string(), "7+4+6+4*8".to_string(), "7+4+6+4/8".to_string()];
    let res = verify(&test, 2);

    println!("{:?}", res);
}
