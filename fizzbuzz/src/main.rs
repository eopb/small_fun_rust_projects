fn main() {
    let fizzbuzz = (1..).map(|val| match (val % 3, val % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => val.to_string(),
    });
    for x in fizzbuzz.take(25) {
        println!("{}", x)
    }
}
