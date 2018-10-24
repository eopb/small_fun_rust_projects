fn main() {
    println!("{}", staircase(5));
}

fn staircase(height: u32) -> String {
    let mut string = String::new();
    for width in 1..=height {
        for _ in 1..=height - width {
            string.push_str(" ")
        }
        for _ in 0..width {
            string.push_str("#")
        }
        string.push_str("\n")
    }
    string
}
