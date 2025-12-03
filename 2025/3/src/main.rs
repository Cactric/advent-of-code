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

        // Get the highest joltage, split the list at that the first appearance of it, then
        // choose the largest one on the right
        // If none, choose the largest digit on the left
        let joltages: Vec<(usize, char)> = bank.chars().enumerate().collect();
        let mut highest: (usize, char) = (0, '\0');
        for j in &joltages {
            if j.1 > highest.1 {
                highest = *j;
            }
        }
        let mut second_highest: (usize, char) = (0, '\0');
        let (before, after) = joltages.split_at(highest.0);
        // If the 'after' part contains more than just the highest we just found
        if after.len() > 1 {
            // Choose the largest one on the right
            // Excluding the highest already used (after[0])
            for j in &after[1..] {
                if j.1 > second_highest.1 {
                    second_highest = *j;
                }
            }
            let bank_joltage = highest.1.to_digit(10).expect("Highest digit wasn't a digit") * 10 +
                               second_highest.1.to_digit(10).expect("Second highest wasn't a digit");
            println!("Bank joltage: {}", bank_joltage);
            jolt_sum += bank_joltage;
        } else {
            // Choose the largest one on the left
            for j in before {
                if j.1 > second_highest.1 {
                    second_highest = *j;
                }
            }
            let bank_joltage = second_highest.1.to_digit(10).expect("Second highest wasn't a digit") * 10 +
                               highest.1.to_digit(10).expect("Highest digit wasn't a digit");
            println!("Bank joltage: {}", bank_joltage);
            jolt_sum += bank_joltage;
        }
        
        /*
        // Sort the joltages of the batteries, using enumerate to have a reference
        // for the initial order
        joltages.sort_unstable_by_key(|(_i,c)| *c);
        
        
        let mut highest_two = joltages.split_at(joltages.len() - 2).1.to_vec();
        highest_two.sort();
        println!("{:?}", highest_two);
        */
    }
    
    eprintln!("Joltage sum: {}", jolt_sum);
}
