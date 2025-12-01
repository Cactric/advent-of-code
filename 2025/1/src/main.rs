use std::fs::read_to_string;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let instruction_list = read_to_string(filename).expect("Couldn't read input file");
    let instructions: Vec<&str> = instruction_list.split("\n").collect();
    
    let mut position = 50; // the dial starts pointing at 50
    let mut zeros = 0; // the actual password is the number of times it's pointing at 0
    let mut zero_clicks = 0; // part two's password
    
    for i in instructions {
        if i.is_empty() {
            continue;
        }
        
        let amount: i32 = (i[1..]).parse().expect("Failed to parse instruction");
        if i.starts_with("L") {
            position -= amount;
        } else if i.starts_with("R") {
            position += amount;
        } else {
            // Complain
            eprintln!("Invalid instruction: {}", i);
            std::process::exit(2);
        }
        
        while position < 0 {
            position += 100;
            zero_clicks += 1;
        }
        while position > 99 {
            position -= 100;
            zero_clicks += 1;
        }
        
        if position == 0 {
            zeros += 1;
        }
    }
    
    println!("Final position: {}", position);
    println!("Number of zeros: {}", zeros);
    println!("Number of zeros (part two): {}", zero_clicks);
}
