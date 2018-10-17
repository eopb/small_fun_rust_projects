use rand::Rng;

fn main() {
    let mut thing_to_sort = thing_to_sort();
    bubble_sort(&mut thing_to_sort, |before, after| before <= after);
    println!("{:?}", thing_to_sort)
}

fn bubble_sort<T, F: Fn(&T, &T) -> bool>(to_sort: &mut [T], is_sorted: F) {
    while let Some((index, _)) = &to_sort.iter().enumerate().find(|(index, value)| {
        *index != &to_sort.len() - 1 && !is_sorted(value, &to_sort[index + 1])
    }) {
        to_sort.swap(*index, index + 1)
    }
}

// Makes a list of random numbers
fn thing_to_sort() -> Vec<u64> {
    std::iter::repeat_with(|| rand::thread_rng().gen_range(1, 5001))
        .take(1000)
        .collect()
}
