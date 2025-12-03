use std::fs::read_to_string;
use std::env;

fn jolt_choose(batteries: usize, mut accumulator: Vec<(usize, char)>, joltages: Vec<(usize, char)>) -> Vec<(usize, char)> {
    if accumulator.len() >= batteries || joltages.is_empty() {
        return accumulator;
    }
    let mut highest: (usize, char) = (0, '\0');
    for j in &joltages {
        if j.1 > highest.1 {
            highest = *j;
        }
    }
    let mut splits = joltages.splitn(2, |(i,_c)| *i == highest.0);
    let before = splits.next().unwrap();
    let after = splits.next().unwrap();
    // If the 'after' part contains more than just the highest we just found
    if after.len() > 0 {
        // Choose the largest one on the right
        // Excluding the highest already used (after[0])
        accumulator.push(highest);
        jolt_choose(batteries, accumulator, after.to_vec())
        // highest * 10 + second_highest
    } else {
        // Choose the largest one on the left
        accumulator.push(highest);
        jolt_choose(batteries, accumulator, before.to_vec())
        // second_highest * 10 + highest
    }
    
}

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
    let mut jolt_sum_p2 = 0;
    
    for bank in banks {
        // Ignore blank lines
        if bank == "" {
            continue;
        }

        // Get the highest joltage, split the list at that the first appearance of it, then
        // choose the largest one on the right
        // If none, choose the largest digit on the left
        let joltages: Vec<(usize, char)> = bank.chars().enumerate().collect();
        //print!("Joltages: {:?}, ", joltages);
        
        // Recursively choose which digits to add
        let mut output = jolt_choose(2, Vec::new(), joltages);
        output.sort();
        
        // Make the bank sum
        let bank_joltage = output.iter().fold(0, |acc, (_i, c)| (acc * 10) + c.to_digit(10).unwrap()); 
        jolt_sum += bank_joltage;
        //println!("Bank joltage: {}", bank_joltage);
    }
    
    eprintln!("Joltage sum (part 1): {}", jolt_sum);
    eprintln!("Joltage sum (part 2): {}", jolt_sum_p2);
}
