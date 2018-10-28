fn main() {
    let mut pi = 4.0;
    for x in (1..).map(|n| (4.0 * f64::from(n)) - 1.0) {
        pi -= 8.0 / (x * (x + 2.0));
        println!("{}", pi);
    }
}
