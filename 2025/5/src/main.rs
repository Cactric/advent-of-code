use std::fs::read_to_string;
use std::env;
use std::ops::{RangeInclusive};
use std::cmp::{min, max};

fn consolidate(ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return vec!();
    }
    
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| *r.start());
    let mut consolidated = vec!(sorted_ranges[0].clone());
    
    // For every range, check there's one that overlaps in consolidated
    for r in sorted_ranges {
        let mut overwriten = false;
        for i in 0..consolidated.len() {
            let c = &consolidated[i];
            if is_overlapping(&r, c) {
                // Rewrite the one in consolidated
                consolidated[i] = min(*c.start(), *r.start())..=max(*c.end(), *r.end());
                overwriten = true;
                break;
            }
        }
        if !overwriten {
            // Add it to consolidated
            consolidated.push(r.clone());
        }
    }
    return consolidated;
}

fn is_overlapping(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    if a.end() >= b.start() && a.start() <= b.end() {
        return true;
    }
    if b.end() <= a.start() && b.start() >= a.end() {
        return true;
    }
    // More cases?
    
    return false;
}

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
    
    eprintln!("Available fresh ingredients: {}", fresh);
    
    
    // Part 2: How many IDs are considered "fresh", even if they're not available
    
    // TODO: consolidate ranges, then count them
    ranges = consolidate(ranges);
    
    let mut count: usize = 0;
    for r in ranges.clone() {
        count += r.count();
    }

    eprintln!("Total fresh ingredients: {}", count);
}
