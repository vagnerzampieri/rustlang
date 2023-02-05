use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use rand::seq::SliceRandom;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("languages.txt").expect("Could not load lines");
    let result = lines.choose(&mut rand::thread_rng());

    println!("Language: {}", result.unwrap());

    let lines = lines_from_file("challenges.txt").expect("Could not load lines");
    let result = lines.choose(&mut rand::thread_rng());

    println!("Challenge: {}", result.unwrap());
}
