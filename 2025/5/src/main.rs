use std::fs::read_to_string;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.get(1) {
        None => filename = "input.txt",
        Some(n) => filename = n
    }
    let ingredient_list = read_to_string(filename).expect("Couldn't read input file");
    let ingredients: Vec<&str> = ingredient_list.trim_end().split("\n").collect();
    
    // Find the blank line delimiter between ranges and available ingredients
    let mut delimiter = 0;
    for (index, ing) in ingredients.iter().enumerate() {
        if *ing == "" {
            delimiter = index;
            break;
        }
    }
    let (ranges_text, available_text) = ingredients.split_at(delimiter);
    
    // Parse the ranges
    let mut ranges = Vec::new();
    for r in ranges_text {
        if let Some((startstr, endstr)) = r.split_once("-") {
            let start = startstr.parse::<u64>().expect("Start of range not a number");
            let end = endstr.parse::<u64>().expect("End of range not a number");
            ranges.push(start..=end);
        }
    }
    
    // Check if the ingredients are fresh
    let mut fresh = 0;
    for a in available_text {
        if *a == "" {
            continue;
        }
        let id = a.parse::<u64>().expect("Ingredient ID not a number");
        // Check if it's in any of the ranges
        for range in &ranges {
            if range.contains(&id) {
                fresh += 1;
                break;
            }
        }
    }
    
    eprintln!("Fresh ingredients: {}", fresh);
    
}
