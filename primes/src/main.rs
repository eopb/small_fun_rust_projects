extern crate rayon;

use rayon::prelude::*;

fn main() {
    print_all_primes()
}

fn primes() -> impl Iterator<Item = u64> {
    (2..).filter(|x| {
        (2..(((*x as f64).sqrt() as u64) + 1))
            .into_par_iter()
            .all(|y| x % y != 0)
    })
}

fn prime_range(low: u64, high: u64) -> impl Iterator<Item = u64> {
    (low..high).filter(|x| {
        (2..(((*x as f64).sqrt() as u64) + 1))
            .into_par_iter()
            .all(|y| x % y != 0)
    })
}

fn nth_prime(n: usize) -> u64 {
    match primes().nth(n) {
        Some(val) => val,
        None => panic!("There does not seem to be an {}th prime", n),
    }
}

fn print_all_primes() {
    for x in primes() {
        println!("{}", x);
    }
}

fn print_prime_range(low: u64, high: u64) {
    for x in prime_range(low, high) {
        println!("{}", x);
    }
}

fn print_x_primes(x: usize) {
    for x in primes().take(x) {
        println!("{}", x);
    }
}

fn print_nth_prime(n: usize) {
    println!("{}", nth_prime(n));
}
