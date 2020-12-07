use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file");
    
    let lines: Vec<&str> = input.split("\n").collect();
    for line in lines {
        println!(">> {} <<", line);
    }
}
