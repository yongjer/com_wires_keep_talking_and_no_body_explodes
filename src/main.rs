use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    // Create a mapping of binary combinations to letters
    let mut mapping = HashMap::new();
    mapping.insert("0000", 'C');
    mapping.insert("0001", 'D');
    mapping.insert("0010", 'C');
    mapping.insert("0011", 'B');
    mapping.insert("0100", 'S');
    mapping.insert("0101", 'P');
    mapping.insert("0110", 'D');
    mapping.insert("0111", 'P');
    mapping.insert("1000", 'S');
    mapping.insert("1001", 'B');
    mapping.insert("1010", 'C');
    mapping.insert("1011", 'B');
    mapping.insert("1100", 'S');
    mapping.insert("1101", 'S');
    mapping.insert("1110", 'P');
    mapping.insert("1111", 'D');

    println!("Please enter four binary numbers, separated by spaces:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let binary_numbers: Vec<&str> = input.trim().split_whitespace().collect();

    if binary_numbers.len() != 4 {
        println!("Error: Please enter exactly four binary numbers.");
        return;
    }

    let max_length = binary_numbers.iter().map(|s| s.len()).max().unwrap_or(0);

    println!("Result:");

    for i in 0..max_length {
        let mut combined = String::new();
        for number in &binary_numbers {
            combined.push(number.chars().nth(i).unwrap_or('0'));
        }
        
        match mapping.get(combined.as_str()) {
            Some(&letter) => println!("{} -> {}", combined, letter),
            None => println!("{} -> Invalid combination", combined),
        }
    }
}