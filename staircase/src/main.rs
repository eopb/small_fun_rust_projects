fn main() {
    println!("{}", staircase(30));
}

fn staircase(height: usize) -> String {
    let mut string = String::new();
    for width in 1..=height {
        string.push_str(&" ".repeat(height - width));
        string.push_str(&"#".repeat(width));
        string.push_str("\n")
    }
    string
}
