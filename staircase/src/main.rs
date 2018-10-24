fn main() {
    for layer in staircase(5) {
        println!("{}", layer);
    }
}

fn staircase(height: u32) -> Vec<String> {
    let mut vec = Vec::new();
    let height = height + 1;
    for width in 1..height {
        let mut string = String::new();
        for _ in 1..height - width {
            string.push_str(" ")
        }
        for _ in 0..width {
            string.push_str("#")
        }

        vec.push(string);
    }
    vec
}
