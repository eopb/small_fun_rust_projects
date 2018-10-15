extern crate rayon;

use rayon::prelude::*;

fn main() {
    for x in (2..).filter(|x| {
        (2..(*x as f64).sqrt() as u64)
            .into_par_iter()
            .all(|y| x % y != 0)
    }) {
        println!("{}", x);
    }
}
