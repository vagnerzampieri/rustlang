use std::fs;

fn parse_to_u32(item: &str) -> u32 {
    item.parse().unwrap()
}

fn sum_data(block: &str) -> u32 {
    block
        .trim()
        .split('\n')
        // .map(|item| item.parse::<u32>().unwrap())
        .map(parse_to_u32)
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
