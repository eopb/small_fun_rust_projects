# Small fun rust projects

Here is a collection of small rust algorithms that do different things. These programs are too small to have their own repositories. I have made them very small just for fun. I enjoy the challenge.

## Bubble sort

This bubble sort is generic. It will work on any type.

```rust
fn bubble_sort<T, F: Fn(&T, &T) -> bool>(to_sort: &mut [T], is_sorted: F) {
    while let Some((index, _)) = &to_sort.iter().enumerate().find(|(index, value)| {
        *index != &to_sort.len() - 1 && !is_sorted(value, &to_sort[index + 1])
    }) {
        to_sort.swap(*index, index + 1)
    }
}
```

## FizzBuzz

This program stores the all infinite values in [fizzbuzz](https://en.wikipedia.org/wiki/Fizz_buzz) in a lazily evaluated way and then prints out the first 25 values.

```rust
fn main() {
    let fizzbuzz = (1..).map(|val| match (val % 3, val % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz"    .to_string(),
        (_, 0) => "Buzz"    .to_string(),
        (_, _) =>  val      .to_string(),
    });
    for x in fizzbuzz.take(25) {
        println!("{}", x)
    }
}
```

## Nvec

This has got to be one of the most flexible collection types.

```rust
#[derive(Clone, Debug)]
struct Nvec<T>(Vec<NOpt<T>>);

#[derive(Clone, Debug)]
enum NOpt<T> {
    Vect(Vec<NOpt<T>>),
    Value(T),
}

fn main() {
    let value = Nvec(vec![NOpt::Vect(vec![NOpt::Value(5); 3]); 3]);
    println!("{:#?}", value);
}
```
