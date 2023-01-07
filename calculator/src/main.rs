use std::io;

fn main() {
    println!("{:-^40}", "");
    println!("{:-^40}", " Calculadora ");
    println!("{:-^40}", "");

    let mut s = String::new();
    let banner = "Digite uma sequência de números\n\
                 separados por vírgula:";

    println!("{banner}");

    io::stdin()
        .read_line(&mut s)
        .expect("Error reading console");

    let nums: Vec<i32> = s.split(",")
        .map(|c| c.trim().parse().expect("Error"))
        .collect();

    let result: i32 = nums.iter().sum();

    println!("O total é {}", result);

    println!("{}", "-".repeat(40));
}
