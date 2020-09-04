// Finds prime numbers. A small project for learning Rust.
// Maybe add a GUI or at least a CLI.

use std::io;
use std::io::Write;

type Num = i64;

// Return true if p is prime.
// This implementation unrolls the loop once so it only has to test 2 potential
// factors out of each 6 sequential numbers.
fn is_prime(p: Num) -> bool {
    if p < 2 { return false; }

    // Check if p is divisible by 2 or 3 so the loop can skip them.
    if p % 2 == 0 { return p == 2; }
    if p % 3 == 0 { return p == 3; }

    // Test factors up to sqrt(p), skipping multiples of 2 and 3.
    // Entering the loop, factor >= 5, p > factor * factor, and p is not
    // divisible by 2 or 3, so p >= 29 and p > factor + 2, so neither test will
    // get confused by testing factor == p.
    let mut factor: Num = 5;
    while factor * factor < p {
        if p % factor == 0 { return false; }
        factor += 2;
        if p % factor == 0 { return false; }
        factor += 4;
    }

    return factor * factor > p;
}

fn next_prime(start: Num, up: bool) -> Num {
    if start < 2 { return 2; }
    let delta = (start & 1) + 1;  // delta to the next odd number
    let mut p = start + if up { delta } else { -delta };  // the next odd
    let delta = if up { 2 } else { -2 };

    while p >= 0 {
        if is_prime(p) { return p; };
        p += delta;
    }
    return 2;
}

fn main() {
    loop {
        print!("Input a number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("read_line failed");

        let mut current: Num = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {  // any input that doesn't parse as an integer
                println!("bye");
                break;
            },
        };

        // println!("The next prime is {}", next_prime(current, true));

        println!("The next primes are:");
        for _ in 0..20 {
            current = next_prime(current, true);
            println!("{}", current);
        }
    }
}
