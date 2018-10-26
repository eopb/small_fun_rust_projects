fn main() {
    diago(|a, b| println!("{}  {}", a, b));
    // let mut values_to_test: Vec<[u32; 3]> = Vec::new();
    // let range = 100;
    // for x in 1..=range {
    //     for y in 1..=range {
    //         for z in 1..=range {
    //             if y < x {
    //                 values_to_test.push([z, y, x]);
    //             }
    //         }
    //     }
    // }
    // let pythagorean_triplets = values_to_test
    //     .iter()
    //     .filter(|val| val[0].pow(2) == val[1].pow(2) + val[2].pow(2));
    // for x in pythagorean_triplets {
    //     println!("{}^2 = {}^2 + {}^2", x[0], x[1], x[2])
    // }
}

// (1,1),(1,2),(1,3),(1,4),(1,5),(1,6)
// (2,1),(2,2),(2,3),(2,4),(2,5),(2,6)
// (3,1),(3,2),(3,3),(3,4),(3,5),(3,6)
// (4,1),(4,2),(4,3),(4,4),(4,5),(4,6)
// (5,1),(5,2),(5,3),(5,4),(5,5),(5,6)
// (6,1),(6,2),(6,3),(6,4),(6,5),(6,6)

// 1 1
// 1 2
// 2 1
// 1 3
// 2 2
// 3 1

fn diago<F: Fn(u32, u32) -> ()>(to_run: F) {
    for a in 1.. {
        for b in 1..=a {
            for c in 1..=b {
                to_run(b, a);
                if a != b {
                    to_run(a, b)
                }
            }
        }
    }
}
fn diagonalize<I1, I2>(it1: I1, it2: I2) -> impl Iterator<Item = (I1::Item, I2::Item)>
where
    I1: IntoIterator,
    <I1 as IntoIterator>::Item: Clone,
    I2: IntoIterator,
    <I2 as IntoIterator>::IntoIter: Clone,
{
    let it1 = it1.into_iter();
    let mut rows = Vec::new();
    let it2 = it2.into_iter();

    it1.flat_map(move |x| {
        rows.push(it2.clone().map(move |y| (x.clone(), y)));
        let v: Vec<_> = rows.iter_mut().flat_map(|it| it.next()).collect();
        v
    })
}

fn main2() {
    let mut l = 0;
    for (x, y) in diagonalize(0.., 0..) {
        println!("{}, {}", x, y);
        l += 1;
        if l > 10 {
            return;
        }
    }
}
