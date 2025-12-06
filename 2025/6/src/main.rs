use std::fs::read_to_string;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let number_list = read_to_string(filename).expect("Couldn't read input file");
    let numbers: Vec<&str> = number_list.trim_end().split("\n").collect();
    
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
}
