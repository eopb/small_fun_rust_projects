extern crate rand;

use rand::Rng;
fn main() {
    println!(
        "{:?}",
        bubble_sort(
            &mut {
                let mut v = Vec::new();
                for _ in 0..8000 {
                    v.push(rand::thread_rng().gen_range(1, 5001));
                }
                println!("made");
                v
            },
            |big_value, small_value| big_value > small_value
        )
    )
}

fn bubble_sort<T, F>(to_sort: &mut [T], f: F) -> &[T]
where
    F: Fn(&T, &T) -> bool,
{
    let mut number_of_reads = 0u128;
    let mut number_of_swaps = 0u128;
    while let Some(value) = &to_sort.iter().enumerate().find(|(index, value)| {
        number_of_reads += 1;
        (if *index != &to_sort.len() - 1 {
            (f(value, &to_sort[index + 1]))
        } else {
            false
        })
    }) {
        to_sort.swap(value.0, value.0 + 1);
        number_of_swaps += 1;
    }

    println!(
        "number of reads :{}, Number of swaps :{}",
        number_of_reads, number_of_swaps
    );
    to_sort
}
// if index == sorted.len() - 1 {
//     continue;
// } else if f(value, &sorted[index + 1]) {
//     continue;
// } else {
//     sorted.swap(index, index + 1);
// }
