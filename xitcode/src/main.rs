use rand::seq::SliceRandom;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Error, Write},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn rand_data(lines: Vec<String>) -> String {
    lines.choose(&mut rand::thread_rng()).unwrap().to_string()
}

fn main() -> Result<(), Error> {
    let languages = lines_from_file("languages.txt").expect("Could not load lines");
    let language = rand_data(languages);

    let challenges = lines_from_file("challenges.txt").expect("Could not load lines");
    let challenge = rand_data(challenges);

    let result = format!("{}: {}", language, challenge);

    println!("{}", result);

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
