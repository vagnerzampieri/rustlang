use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt")
        .expect("Cannot open to the file.");

    let result: u32 = contents
        .split("\n\n")
        .map(|block| 
            block
                .split('\n')
                .filter(|item| !item.is_empty())
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        )
        .max()
        .unwrap();

    println!("Result {:?}", result);
}
