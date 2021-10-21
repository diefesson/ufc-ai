use std::collections::HashSet;

fn main() {

    let mut numbers = HashSet::new();
    numbers.insert(1);
    numbers.insert(2);
    numbers.insert(3);

    let mut f = numbers.into_iter().find(|n| *n == 2).unwrap();

    println!("Hello, world!");
}
