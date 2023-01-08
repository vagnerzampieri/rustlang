fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase()
}

fn add_prefix(text: &mut String) {
    *text = format!("FOO_{text}");
}

fn main() {
    let mut name = "Zampi".to_string();
    to_uppercase(&mut name);
    add_prefix(&mut name);
    println!("{name}");
}
