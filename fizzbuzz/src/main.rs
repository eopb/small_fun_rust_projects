fn main() {
    for x in (1..100).map(|val| {
        if val % 3 == 0 {
            if val % 5 == 0 {
                "fizzbuzz".to_string()
            } else {
                "fizz".to_string()
            }
        } else if val % 5 == 0 {
            "buzz".to_string()
        } else {
            val.to_string()
        }
    }) {
        println!("{}", x)
    }
}
