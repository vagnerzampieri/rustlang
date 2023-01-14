use std::io;
use std::collections::HashSet;

// split cada linha na metade
// encontrar qual letra se repete
// a - z = 1 a 26
// A - Z = 27 a 52
// soma do valor de cada item

fn get_priority(n: u32) -> u32 {
    if n >= 97 {
        n - 97 + 1
    } else {
        n - 65 + 27
    }
}

fn main() {
    let result: u32 = io::stdin()
        .lines()
        .filter_map(|line| match line {
            Ok(text) => {
                let (a, b) = text.split_at(text.len()/2);

                Some((a.to_owned(), b.to_owned()))
            },
            Err(_) => None,
        })
        .map(|(left, right)| {
            let a_set: HashSet<u32> = HashSet::from_iter(left.chars().map(|c| c as u32));
            let b_set: HashSet<u32> = HashSet::from_iter(right.chars().map(|c| c as u32));
            let intersection = a_set.intersection(&b_set).next().unwrap();

            get_priority(*intersection)
        })
        .sum();

    println!("Result {:?}", result);
}
