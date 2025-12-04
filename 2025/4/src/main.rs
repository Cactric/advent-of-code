use std::fs::read_to_string;
use std::env;

/**
 * Check how many rolls (@s) are in a column
 */
fn check_col(rolls: &Vec<&str>, x: usize, y: usize) -> usize {
    let mut amount = 0;
    // Check above
    if y > 0 {
        if rolls[y - 1].chars().nth(x) == Some('@') {
            amount += 1;
        }
    }
    // Check below
    if y+1 < rolls.len() {
        if rolls[y + 1].chars().nth(x) == Some('@') {
            amount += 1;
        }
    }
    // Check middle
    if rolls[y].chars().nth(x) == Some('@') {
        amount += 1;
    }
    return amount;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let roll_list = read_to_string(filename).expect("Couldn't read input file");
    let rolls: Vec<&str> = roll_list.trim_end().split("\n").collect();
    
    let mut accessible_rolls = 0;

    for y in 0..rolls.len() {
        for x in 0..rolls[y].len() {
            if rolls[y].chars().nth(x) != Some('@') {
                if let Some(c) = rolls[y].chars().nth(x) {
                    print!("{}", c);
                }
                continue;
            }
            let mut adj_rolls = 0;
            // Left side
            if x > 0 {
                adj_rolls += check_col(&rolls, x - 1, y);
            }
            // Right side
            if x < rolls[y].len() {
                adj_rolls += check_col(&rolls, x + 1, y);
            }
            // Middle (unconditional x)
            adj_rolls += check_col(&rolls, x, y);
            
            if adj_rolls <= 4 {
                // It's less than 4 rolls, but the count above includes itself, so do <= to compensate
                accessible_rolls += 1;
                print!("x");
            } else {
                print!("{}", rolls[y].chars().nth(x).unwrap());
            }
        }
        print!("\n");
    }
    
    println!("Result: {} accessible rolls", accessible_rolls);
}
