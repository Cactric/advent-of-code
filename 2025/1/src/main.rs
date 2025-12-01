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
    let mut lineno = 0;
    
    for i in instructions {
        lineno += 1;
        if lineno == 4558 {
            eprintln!("break")
        }
        let mut previously_zero = position == 0;
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
            if !previously_zero {
                zero_clicks += 1;
                println!("Click at line {} ({})", lineno, i)
            }
            previously_zero = false;
        }
        while position > 99 {
            position -= 100;
            zero_clicks += 1;
            println!("Click at line {} ({})", lineno, i);
            previously_zero = position == 0;
        }
        
        if position == 0 {
            zeros += 1;
            if !previously_zero {
                zero_clicks += 1;
                println!("Click at line {} ({})", lineno, i)
            }
        }
        //println!("trace: {} / pos = {} / zeros = {} / clicks = {}", i, position, zeros, zero_clicks);
    }
    
    eprintln!("Final position: {}", position);
    eprintln!("Number of zeros: {}", zeros);
    eprintln!("Number of zeros (part two): {}", zero_clicks);
}
