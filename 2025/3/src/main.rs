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
    let mut jolt_sum_p2 = 0;
    
    for bank in banks {
        // Ignore blank lines
        if bank == "" {
            continue;
        }

        // Get the highest joltage, split the list at that the first appearance of it, then
        // choose the largest one on the right
        // If none, choose the largest digit on the left
        let mut joltages: Vec<(usize, char)> = bank.chars().enumerate().collect();
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
            jolt_sum += bank_joltage;
        }
        
        // Sort the joltages of the batteries, using enumerate to have a reference
        // for the initial order.
        // Joltage has priority, but index is still accounted for
        joltages.sort_unstable_by_key(|(i,c)| (c.to_digit(10).unwrap() as usize) * bank.len() + *i);
        
        let mut highest_twelve = joltages.split_at(joltages.len() - 12).1.to_vec();
        // Re-sort by index
        highest_twelve.sort();
        println!("{:?}", highest_twelve);
        let mut bank_jolts: u64 = 0;
        for (_,j) in highest_twelve {
            let jolts: u64 = j.to_digit(10).unwrap() as u64;
            bank_jolts *= 10;
            bank_jolts += jolts;
            println!("Added {} to produce {}", jolts, bank_jolts);
        }
        jolt_sum_p2 += bank_jolts;
        
        // Idea to solve p2: same as p1 basically, but obviously it'll need to be a loop and actually remove the element
    }
    
    eprintln!("Joltage sum (part 1): {}", jolt_sum);
    eprintln!("Joltage sum (part 2): {}", jolt_sum_p2);
}
