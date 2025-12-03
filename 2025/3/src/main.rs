use std::fs::read_to_string;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let bank_list = read_to_string(filename).expect("Couldn't read input file");
    let banks: Vec<&str> = bank_list.split("\n").collect();
    
    let mut jolt_sum = 0;
    
    for bank in banks {
        // Ignore blank lines
        if bank == "" {
            continue;
        }
        
        // Sort the joltages of the batteries, using enumerate to have a reference
        // for the initial order
        let mut joltages: Vec<(usize, char)> = bank.chars().enumerate().collect();
        joltages.sort_unstable_by_key(|(_i,c)| *c);
        
        // Get the highest two joltages and sort them by position
        let mut highest_two = joltages.split_at(joltages.len() - 2).1.to_vec();
        highest_two.sort();
        println!("{:?}", highest_two);
        let mut bank_joltage = 0;
        for (_i, j) in highest_two {
            bank_joltage *= 10;
            bank_joltage += j.to_digit(10).unwrap();
        }
        println!("Bank joltage: {}", bank_joltage);
        jolt_sum += bank_joltage;
    }
    
    eprintln!("Joltage sum: {}", jolt_sum);
}
