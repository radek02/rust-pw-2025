use std::io;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let was_error = loop {
        println!("Enter a number.");

        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        let mut x: u64 = match x.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Parsing a number failed.");
                break true;
            }
        };

        if x == 0 {
            break false;
        }

        let random = rand::thread_rng().gen_range(1..=5);

        x += random;
        println!("Your number after addition is: {x}");

        let powers_arr = powers(x);
        println!("Powers of {}: {:?}", x, powers_arr); // TODO: delete this debug line

        let collatz_arr = is_collatz_ok(&powers_arr);
        println!("Collatz conjecture results: {:?}", collatz_arr);

        if write_to_file(&collatz_arr) {
            break true;
        }

        let (first_prime_index, msg) = find_first_prime(&powers_arr);
        println!("{}, index: {}", msg, first_prime_index);
    };

    if was_error {
        println!("An error occurred during execution :(");
    } else {
        println!("User decided to exit :(");
    }
}

fn powers(base: u64) -> [u64; 10] {
    let mut arr: [u64; 10] = [0; 10];
    let mut cur_power: u64 = base;
    for elem in &mut arr {
        *elem = cur_power;
        cur_power *= base;
    }
    arr
}

fn is_collatz_ok(arr: &[u64; 10]) -> [bool; 10] {
    const MAX_ITERATIONS: u32 = 100;
    let mut ans_arr = [false; 10];
    for i in 0..10 {
        let mut n = arr[i];
        for _j in 0..MAX_ITERATIONS {
            if n == 1 {
                ans_arr[i] = true;
                break;
            }
            if n.is_multiple_of(2) {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
        }
        if !ans_arr[i] {
            eprintln!("arr[{i}] is not confirmed to satisfy Collatz conjecture after {MAX_ITERATIONS} iterations.");
        }
    }
    ans_arr
}

// returns true if there was an error
fn write_to_file(arr: &[bool; 10]) -> bool {
    let mut file = File::create("xyz.txt").expect("Could not create file :(");
    let content = format!("{:?}", arr);
    match file.write_all(content.as_bytes()) {
        Ok(_) => {
            println!("Data successfully written to xyz.txt");
            false
        }
        Err(e) => {
            println!("Failed to write to file: {}", e);
            true
        }
    }
}

fn find_first_prime(arr: &[u64; 10]) -> (u64, String) {
    let mut ind = 0;
    'outer: loop {
        let elem = arr[ind];
        let mut i = 2;
        'inner: loop {
            if i * i > elem {
                // is prime
                break 'outer (ind as u64, format!("First prime found: {}", elem))
            
            }
            if elem.is_multiple_of(i) {
                // is not prime
                break 'inner;
            } 
            i += 1;
        }
        ind += 1;
        if ind >= arr.len() {
            break 'outer (arr.len() as u64, "No primes found".to_string())
        }
    }
}