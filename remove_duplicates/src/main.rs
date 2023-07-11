use std::collections::HashSet;

fn remove_duplicates(numbers: Vec<i32>) -> Vec<i32> {
    let mut unique_numbers = Vec::new();
    let mut seen = HashSet::new();

    for number in numbers {
        // HashSet::insert() returns true if the value was not present in the set
        if seen.insert(number) {
            // Vec::push() returns nothing
            // so we need to push the number into the vector
            unique_numbers.push(number);
        }
    }

    unique_numbers
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 4, 4, 5, 5, 5];
    let unique_numbers = remove_duplicates(numbers);
    println!("{:?}", unique_numbers);
}