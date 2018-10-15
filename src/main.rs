extern crate rayon;

use rayon::prelude::*;

fn main() {
    print_primes();
}

fn print_primes() -> () {
    for x in (2..).filter(|x| {
        (2..(*x as f64).sqrt() as u64)
            .into_par_iter()
            .all(|y| x % y != 0)
    }) {
        println!("{}", x);
    }
}

// fn print_primes_all() {
//     let mut v = Vec::new();
//     for x in (2..).filter(|x| {
//         v.par_iter()
//             .filter(|z| z <= &&((*x as f64).sqrt() as u64))
//             .all(|y| x % y != 0)
//     }) {
//         println!("{}", x);
//         v.push(x);
//     }
// }

fn print_primes_all_help() {
    let mut primes = vec![2];
    for x in (1..).map(|x| x * 2 + 1) {
        let x_sqrt = (x as f64).sqrt() as u64;
        if !primes
            .iter()
            .take_while(|&y| y <= &x_sqrt)
            .any(|&y| x % y == 0)
        {
            primes.push(x);
            println!("{}", x);
        }
    }
    for prime in primes {
        println!("{}", prime);
    }
}
