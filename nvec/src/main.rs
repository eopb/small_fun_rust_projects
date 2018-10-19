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
    println!("TEST");
}
