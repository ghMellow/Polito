use crate::game::verify;
use crate::primes::find_primes_sol1;
use crate::primes::find_primes_sol2;
use crate::producer_consumer::run_prod_consumer;
use std::time::Instant;
mod primes;
mod game;
mod producer_consumer;

fn main() {
    let duration = Instant::now();
    let res = find_primes_sol1(1000000, 16);
    println!("{:?}\nTotal execution time: {:?}", res, duration.elapsed());

    let duration = Instant::now();
    let res = find_primes_sol2(1000000, 16);
    println!("{:?}\nTotal execution time: {:?}", res, duration.elapsed());

    let test = ["2+2+2+2+2".to_string(),
                          "7+4+6+4-8".to_string(),
                          "7+4+6+4*8".to_string(),
                          "7+4+6+4/8".to_string(),
                          "2*2*2*2+2".to_string(),
                          "2*5/2+5".to_string()];
    let res = verify(&test, 2);

    println!("{:?}\n", res);

    println!("\nproducer consumer script:\n");
    run_prod_consumer();
}
