use std::fs;

fn sum_data(block: &str) -> u32 {
    block
        .split('\n')
        .filter(|item| !item.is_empty())
        .map(|item| item.parse::<u32>().unwrap())
        .sum::<u32>()
}

fn main() {
    let contents = fs::read_to_string("data.txt")
        .expect("Cannot open to the file.");

    let result: u32 = contents
        .split("\n\n")
        .map(sum_data)
        .max()
        .unwrap();

    println!("Result {:?}", result);
}
