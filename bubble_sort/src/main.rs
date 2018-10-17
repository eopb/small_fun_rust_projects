extern crate rand;

use rand::Rng;
fn main() {
    println!(
        "{:?}",
        bubble_sort(
            &mut thing_to_sort(),
            |before_value, after_value| before_value <= after_value
        )
    )
}

fn bubble_sort<T, F>(to_sort: &mut [T], f: F) -> &[T]
where
    F: Fn(&T, &T) -> bool,
{
    while let Some(value) = &to_sort
        .iter()
        .enumerate()
        .find(|(index, value)| *index != &to_sort.len() - 1 && !f(value, &to_sort[index + 1]))
    {
        to_sort.swap(value.0, value.0 + 1);
    }
    to_sort
}

fn thing_to_sort() -> Vec<u64> {
    let mut v = Vec::new();
    for _ in 0..8000 {
        v.push(rand::thread_rng().gen_range(1, 5001));
    }
    println!("made");
    v
}
