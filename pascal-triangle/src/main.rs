fn main() {
    let pascal_triangle = (0..).map(|n| {
        (0..=n).map(move |r| {
            let f = |x| -> u32 { (1..=x).product() };
            f(n) / (f(r) * f(n - r))
        })
    });
    let tri_test: Vec<Vec<u32>> = pascal_triangle
        .take(13)
        .map(|x| x.collect())
        .collect::<Vec<_>>();
    dbg!(tri_test);
}
