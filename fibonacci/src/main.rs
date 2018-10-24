fn main() {
    let fibonacci = (0..).map(|n| nthfib(n));
    for x in fibonacci {
        println!("{}", x)
    }
}

fn nthfib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => nthfib(n - 1) + nthfib(n - 2),
    }
}
