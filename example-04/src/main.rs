use std::fs;

// A, X = Pedra   = 1
// B, Y = Papel   = 2
// C, Z = Tesoura = 3
// Perdi = 0
// Empate = 3
// Ganhei = 6

// Variações Possíveis
// p1 x p2 | Resultado | total = p2 + resultado
// --------------------------------------------
// A x X   | Empate    | (1 + 3) = 4
// A x Y   | Ganhei    | (2 + 6) = 8
// A x Z   | Perdi     | (3 + 0) = 3
// B x X   | Perdi     | (1 + 0) = 1
// B x Y   | Empate    | (2 + 3) = 5
// B x Z   | Ganhei    | (3 + 6) = 9
// C x X   | Ganhei    | (1 + 6) = 7
// C x Y   | Perdi     | (2 + 0) = 2
// C x Z   | Empate    | (3 + 3) = 6
// --------------------------------------------

fn main() {
    let mut result = 0;
    let contents = fs::read_to_string("data.txt")
        .expect("Cannot open to the file.");

    for line in contents.lines() {
        let line_result = match line.as_ref() {
            "A X" => 4, 
            "A Y" => 8, 
            "A Z" => 3, 
            "B X" => 1, 
            "B Y" => 5, 
            "B Z" => 9, 
            "C X" => 7, 
            "C Y" => 2, 
            "C Z" => 6,
            _ => 0,
        };

        result += line_result;
    }

    println!("Result {}", result);
}
