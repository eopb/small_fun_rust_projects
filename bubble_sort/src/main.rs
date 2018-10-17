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
    while let Some(value) = &to_sort.iter().enumerate().find(|(index, value)| {
        (if *index != &to_sort.len() - 1 {
            (f(value, &to_sort[index + 1]))
        } else {
            false
        })
    }) {
        to_sort.swap(value.0, value.0 + 1);
    }
    to_sort
}
