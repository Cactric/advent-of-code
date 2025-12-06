use std::fs::read_to_string;
use std::env;
use std::process::exit;

fn is_all_spaces(s: &str) -> bool {
    for c in s.chars() {
        if c != ' ' {
            return false;
        }
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let number_list = read_to_string(filename).expect("Couldn't read input file");
    let numbers: Vec<&str> = number_list.trim_end_matches('\n').split("\n").collect();
    
    // Split the input by whitespace (which conveniently handles multiple spaces for me)
    let mut split_nums: Vec<Vec<&str>> = numbers.iter().map(|s| s.split_ascii_whitespace().collect()).collect();
    
    let operations = split_nums.pop().expect("Operations not found. Empty input?");
    
    let mut total: u64 = 0;
    for (col, op) in operations.iter().enumerate() {
        // Collect the numbers in the column
        let mut column_nums = Vec::new();
        
        for x in 0..split_nums.len() {
            column_nums.push(split_nums[x][col].parse::<u64>().expect("Not a number"));
        }
        
        match *op {
            "*" => {
                total += column_nums.iter().fold(1, |acc, x| acc * x);
            }
            "+" => {
                total += column_nums.iter().fold(0, |acc, x| acc + x);
            }
            unknown => {
                eprintln!("Unknown operation {}", unknown);
                exit(2);
            }
        }
    }
    
    println!("Total (part 1): {}", total);
    
    // Part 2
    // Make a list of columns (transpose)
    let mut transposed_numbers = Vec::new();
    for col in 0..numbers[0].len() {
        let mut t_row: Vec<u8> = Vec::new();
        for row in 0..(numbers.len()-1) {
            t_row.push(numbers[row].as_bytes()[col]);
        }
        transposed_numbers.push(String::from_utf8(t_row).unwrap());
    }
    
    let t_matrix: Vec<&[String]> = transposed_numbers.split(|s| is_all_spaces(s)).collect();
    
    let mut part2_total = 0;
    // Go backwards through the transposed matrix of numbers
    for x in (0..operations.len()).rev() {
        let op = operations[x];
        let num_list = t_matrix[x].iter().map(|s| s.trim().parse::<u64>().expect("Not a number"));
        // Reversing the num_list is unnecessary here, since + and * are both symmetric
        part2_total += match op {
            "*" => num_list.rev().fold(1, |acc, x| acc * x),
            "+" => num_list.rev().fold(0, |acc, x| acc + x),
            unknown => {
                eprintln!("Unknown operation {}", unknown);
                exit(2);
            }
        }
    }
    println!("Total (part 2): {}", part2_total);
}
