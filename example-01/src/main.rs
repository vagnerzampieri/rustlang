// fn convert_to_number(s: &str) -> i32 {
//     s.parse().unwrap()
// }

fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let input = "56 65 58 48 59 56 87 23";

    let result: Vec<i32> = input
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .map(double)
        .collect();

    println!("{:?}", result);
}
