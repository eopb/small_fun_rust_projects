fn main() {
    let mut values_to_test: Vec<[u32; 3]> = Vec::new();
    let range = 100;
    for x in 1..=range {
        for y in 1..=range {
            for z in 1..=range {
                if y < x {
                    values_to_test.push([z, y, x]);
                }
            }
        }
    }
    let pythagorean_triplets = values_to_test
        .iter()
        .filter(|val| val[0].pow(2) == val[1].pow(2) + val[2].pow(2));
    for x in pythagorean_triplets {
        println!("{}^2 = {}^2 + {}^2", x[0], x[1], x[2])
    }
}
