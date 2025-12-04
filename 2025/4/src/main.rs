use std::fs::read_to_string;
use std::env;

/**
 * Check how many rolls (@s) are in a column
 */
fn check_col(rolls: &Vec<String>, x: usize, y: usize) -> usize {
    let mut amount = 0;
    // Check above
    if y > 0 && check_roll(rolls, x, y-1) {
        amount += 1;
    }
    // Check below
    if y+1 < rolls.len() && check_roll(rolls, x, y+1) {
        amount += 1;
    }
    // Check middle
    if check_roll(rolls, x, y) {
        amount += 1;
    }
    return amount;
}

fn check_roll(rolls: &Vec<String>, x: usize, y: usize) -> bool {
    return rolls[y].as_bytes().get(x) == Some(&b'@');
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let roll_list = read_to_string(filename).expect("Couldn't read input file");
    let mut rolls: Vec<String> = roll_list.trim_end().split("\n").map(|s| s.to_string()).collect();
    
    let mut accessible_rolls = 0;

    loop {
        let prev_acc_rolls = accessible_rolls;
        let mut new_rolls: Vec<String> = Vec::new();
        
        for y in 0..rolls.len() {
            let mut new_roll_row = String::new();
            for x in 0..rolls[y].len() {
                if !check_roll(&rolls, x, y) {
                    if let Some(c) = rolls[y].as_bytes().get(x) {
                        new_roll_row.push(char::from(*c));
                        //print!("{}", c);
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
                    new_roll_row.push('.');
                    //print!("x");
                } else {
                    new_roll_row.push(char::from(*(rolls[y].as_bytes().get(x).unwrap())));
                    //print!("{}", rolls[y].chars().nth(x).unwrap());
                }
            }
            //print!("\n");
            new_rolls.push(new_roll_row);
        }
        
        rolls = new_rolls;
        if prev_acc_rolls == accessible_rolls {
            break;
        }
    }
        
    println!("Result: {} accessible rolls", accessible_rolls);
}
