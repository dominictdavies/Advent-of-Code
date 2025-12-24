use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    println!("input.txt:\n{input}");
}
