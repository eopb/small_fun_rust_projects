extern crate rayon;

use rayon::prelude::*;
use std::ops::RangeFrom;

fn main() {
    for x in primes(2..) {
        println!("{}", x);
    }
}

fn primes(range: RangeFrom<u64>) -> impl Iterator<Item = u64> {
    range.filter(|x| {
        (2..(*x as f64).sqrt() as u64)
            .into_par_iter()
            .all(|y| x % y != 0)
    })
}
