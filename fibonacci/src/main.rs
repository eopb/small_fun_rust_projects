use std::collections::HashMap;

fn main() {
    let mut cache = HashMap::new();
    let fibonacci = (0..).map(|n| nthfib(n, &mut cache));
    for x in fibonacci.take(187) {
        println!("{}", x)
    }
}

fn nthfib(n: u128, mut cache: &mut HashMap<u128, u128>) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if let Some(v) = cache.get(&n) {
                *v
            } else {
                let v = nthfib(n - 1, &mut cache) + nthfib(n - 2, &mut cache);
                cache.insert(n, v);
                v
            }
        }
    }
}
