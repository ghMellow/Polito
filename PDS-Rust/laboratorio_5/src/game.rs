use std::sync::{Arc, Mutex};
use std::thread;
use itertools::{Itertools, Permutations};
use crate::primes::is_prime;
use evalexpr::eval;

pub fn mk_ops(symbols: &[char], n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }

    let mut result = vec![];

    for &symbol in symbols {
        for perm in mk_ops(symbols, n - 1) {
            result.push(format!("{}{}", symbol, perm));
        }
    }

    result
}

pub fn prepare(s: &str) -> Vec<String> {

    let mut result = vec![];
    let ops = mk_ops(&['+', '-', '*', '/'], 4);
    
    for digit in s.chars().permutations(s.len()) {
        for op_seq in &ops {
            let mut s = String::new();
            let mut it_op = op_seq.chars();
            for d in digit.iter() {
                s.push(*d);
                if let Some(op) = it_op.next() {
                    s.push(op);
                }
            }
            result.push(s);
        }
    }   
    result
}

#[test]
fn test_mk_ops() {
    let symbols = vec!['+', '-', '*', '/'];
    let n = 4;
    let result = mk_ops(&symbols, n);
    assert_eq!(result.len(), symbols.len().pow(n as u32));

    let res = prepare("23423");
    println!("{} {:?}", res.len(), res.iter().take(n).collect::<Vec<_>>());
}

fn split_into_n_chunks(v: &[String], n: usize) -> Vec<&[String]> {
    let len = v.len();
    let chunk_size = (len + n - 1) / n; // round up
    v.chunks(chunk_size).collect()
}

pub fn verify(v: &[String], n_threads: usize) -> Vec<String> {
    let res = Arc::new(Mutex::new(Vec::new()));

    // creo slice statici
    let chunks = split_into_n_chunks(&v, n_threads);

    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {:?}", i, chunk);
    }

    // creo n thread e passo lo slice
    thread::scope(|s| {
        for i in 0..n_threads {
            let mut thread_chunk = chunks[i].clone();
            let mut thread_res = res.clone();
            s.spawn(move || {
                for expr in thread_chunk {
                    let result = eval(expr.as_str()).unwrap();
                    if result.as_number().unwrap_or(f64::NAN) == 10.0 {
                        thread_res.lock().unwrap().push(expr.clone());
                    }
                }

            });
        }
    });

    res.lock().unwrap().to_vec()
}