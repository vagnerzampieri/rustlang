use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Error, Write},
    path::Path,
};
use rand::seq::SliceRandom;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() -> Result<(), Error> {
    let lines = lines_from_file("languages.txt").expect("Could not load lines");
    let language = lines.choose(&mut rand::thread_rng()).unwrap();

    println!("Language: {}", language);

    let lines = lines_from_file("challenges.txt").expect("Could not load lines");
    let challenge = lines.choose(&mut rand::thread_rng()).unwrap();

    println!("Challenge: {}", challenge);

    let result = format!("{}, {}", language, challenge);

    println!("{:?}", result);

    let file_name = "result.txt";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(file_name)?;

    writeln!(file, "{}", result)?;
    Ok(())
}
