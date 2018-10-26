extern crate rayon;

use rayon::prelude::*;

fn main() {
    for x in primes() {
        println!("{}", x);
    }
}

fn primes() -> impl Iterator<Item = u64> {
    (2..).filter(|x| {
        (2..(((*x as f64).sqrt() as u64) + 1))
            .into_par_iter()
            .all(|y| x % y != 0)
    })
}
